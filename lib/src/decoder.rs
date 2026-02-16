use crate::error::{F2V2FError, Result};
use crate::config::DecodeConfig;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Write, Read, BufReader};
use std::path::Path;
use tracing::info;
use tempfile;

/// Decodes a video back to the original file
pub struct Decoder {
    config: DecodeConfig,
}

/// Metadata extracted from encoded video
#[derive(Debug, Clone)]
pub struct DecodedFileInfo {
    pub extracted_size: u64,
    pub checksum: String,
    pub matches_checksum: bool,
}

impl Decoder {
    pub fn new(config: DecodeConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Decode a video back to file
    pub async fn decode<P: AsRef<Path>>(&self, input: P, output: P) -> Result<DecodedFileInfo> {
        let input_path = input.as_ref();
        let output_path = output.as_ref();

        info!("Starting file decoding: {}", input_path.display());

        // Use a temporary file for extraction if compression is enabled
        if self.config.use_compression {
            let mut temp_file = tempfile::NamedTempFile::new()?;
            info!("Extracting compressed data to temp file: {}", temp_file.path().display());
            
            let (_extracted_size, _checksum) = self
                .process_video(input_path, &mut temp_file)
                .await?;
            
            temp_file.as_file_mut().flush()?;
            
            // Re-open for reading
            let temp_reader = BufReader::new(File::open(temp_file.path())?);
            
            let mut final_output = File::create(output_path)?;
            let mut hasher = Sha256::new();
            
            info!("Decompressing extracted data...");
            let start = std::time::Instant::now();
            
            let mut total_restored = 0;
            // Use a limited reader if encoded_size is set
            if let Some(limit) = self.config.encoded_size {
                let mut limited = temp_reader.take(limit);
                let mut zstd_decoder = zstd::stream::read::Decoder::new(&mut limited)?;
                let mut buffer = vec![0u8; 128 * 1024];
                loop {
                    let n = zstd_decoder.read(&mut buffer)?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                    final_output.write_all(&buffer[..n])?;
                    total_restored += n as u64;
                }
            } else {
                let mut zstd_decoder = zstd::stream::read::Decoder::new(temp_reader)?;
                let mut buffer = vec![0u8; 128 * 1024];
                loop {
                    let n = zstd_decoder.read(&mut buffer)?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                    final_output.write_all(&buffer[..n])?;
                    total_restored += n as u64;
                }
            }
            
            let duration = start.elapsed();
            let checksum = format!("{:x}", hasher.finalize());
            info!("Decompression complete in {:?}. Restored {} bytes", duration, total_restored);
            
            Ok(DecodedFileInfo {
                extracted_size: total_restored,
                checksum,
                matches_checksum: false,
            })
        } else {
            let mut final_output = File::create(output_path)?;
            let (_extracted_size, _checksum) = self
                .process_video(input_path, &mut final_output)
                .await?;
            
            // Truncate if encoded_size is known
            if let Some(limit) = self.config.encoded_size {
                final_output.set_len(limit)?;
            }
            
            final_output.sync_all()?;
            
            // Recalculate checksum if truncated
            let checksum = if self.config.encoded_size.is_some() {
                let mut hasher = Sha256::new();
                let mut f = File::open(output_path)?;
                std::io::copy(&mut f, &mut hasher)?;
                format!("{:x}", hasher.finalize())
            } else {
                _checksum
            };

            Ok(DecodedFileInfo {
                extracted_size: self.config.encoded_size.unwrap_or(_extracted_size),
                checksum,
                matches_checksum: false,
            })
        }
    }

    async fn process_video<W: Write>(
        &self,
        video_path: &Path,
        writer: &mut W,
    ) -> Result<(u64, String)> {
        let mut hasher = Sha256::new();
        let mut total_bytes_written: u64 = 0;

        let composer = crate::video_composer::VideoComposer::new(
            self.config.width,
            self.config.height,
            30,
        );

        let generator = crate::image_generator::GeometricArtGenerator::new(
            self.config.width,
            self.config.height,
            42,
        );

        // Process frames one by one via callback
        composer.extract_frames(video_path, |frame| {
            let data = generator.decode_from_image(&frame, self.config.chunk_size)?;
            
            hasher.update(&data);
            writer.write_all(&data).map_err(|e| F2V2FError::Io(e.to_string()))?;
            total_bytes_written += data.len() as u64;
            
            Ok(())
        }).await?;

        let checksum = format!("{:x}", hasher.finalize());
        Ok((total_bytes_written, checksum))
    }

    // fn check_operation_health(&self, frame_number: u64) -> Result<()> {
    //     // Check for memory pressure, cancellation signals, etc.
    //     if frame_number % 100 == 0 {
    //         debug!("Health check at frame {}", frame_number);
    //     }
    //     Ok(())
    // }

    /// Verify that decoded file matches original checksum
    pub fn verify_checksum<P: AsRef<Path>>(
        &self,
        file_path: P,
        expected_checksum: &str,
    ) -> Result<bool> {
        let path = file_path.as_ref();
        let file = File::open(path)?;
        let mut reader = std::io::BufReader::new(file);

        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 1024 * 1024]; // 1MB chunks

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(e) => {
                    return Err(F2V2FError::Io(e.to_string()).into());
                }
            }
        }

        let checksum = format!("{:x}", hasher.finalize());
        Ok(checksum == expected_checksum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_decoder_creation() {
        let config = DecodeConfig::default();
        let decoder = Decoder::new(config).unwrap();
        assert!(decoder.config.validate().is_ok());
    }

    #[tokio::test]
    async fn test_verify_checksum() -> Result<()> {
        let config = DecodeConfig::default();
        let decoder = Decoder::new(config)?;

        let mut file = NamedTempFile::new()?;
        file.write_all(b"test data")?;
        file.flush()?;

        // Calculate checksum of test file
        let mut hasher = Sha256::new();
        hasher.update(b"test data");
        let expected_checksum = format!("{:x}", hasher.finalize());

        let matches = decoder.verify_checksum(file.path(), &expected_checksum)?;
        assert!(matches);

        let no_match = decoder.verify_checksum(file.path(), "wrongchecksum")?;
        assert!(!no_match);

        Ok(())
    }
}
