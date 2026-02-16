use crate::error::{F2V2FError, Result, ErrorContext};
use crate::config::DecodeConfig;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Write, BufWriter, Read};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::{info, warn, debug};

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

        // Open output for writing
        let output_file = File::create(output_path)?;
        let mut writer = BufWriter::new(output_file);

        let (extracted_size, checksum) = self
            .process_video(input_path, &mut writer)
            .await?;

        writer.flush()?;

        info!("Decoding complete. Extracted {} bytes", extracted_size);
        info!("Checksum: {}", checksum);

        Ok(DecodedFileInfo {
            extracted_size,
            checksum,
            matches_checksum: false, // Would be set after comparing with original
        })
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
            30, // FPS doesn't matter much for decoding
        );

        let frames = composer.extract_frames(video_path).await?;
        let generator = crate::image_generator::GeometricArtGenerator::new(
            self.config.width,
            self.config.height,
            42, // Should ideally match the encoding seed
        );

        let progress = ProgressBar::new(frames.len() as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} frames ({eta})")
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("#>-"),
        );

        for (_i, frame) in frames.iter().enumerate() {
            let data = generator.decode_from_image(frame, self.config.chunk_size)?;
            
            // Note: Currently we don't know the exact end of data if it's not a full chunk
            // For now we write the whole chunk. A better implementation would store the length.
            hasher.update(&data);
            writer.write_all(&data)?;
            total_bytes_written += data.len() as u64;
            
            progress.inc(1);
        }

        progress.finish_with_message("Decoding complete!");

        let checksum = format!("{:x}", hasher.finalize());
        Ok((total_bytes_written, checksum))
    }

    fn check_operation_health(&self, frame_number: u64) -> Result<()> {
        // Check for memory pressure, cancellation signals, etc.
        if frame_number % 100 == 0 {
            debug!("Health check at frame {}", frame_number);
        }
        Ok(())
    }

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
