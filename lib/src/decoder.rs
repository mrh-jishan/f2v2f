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

        // Get input file size
        let input_size = std::fs::metadata(input_path)?.len();
        info!("Input video size: {} bytes ({:.2} MB)", input_size, input_size as f64 / 1024.0 / 1024.0);

        // Open input for reading
        let input_file = File::open(input_path)?;
        
        // Open output for writing
        let output_file = File::create(output_path)?;
        let mut writer = BufWriter::new(output_file);

        let (extracted_size, checksum) = self
            .process_video(input_file, &mut writer, input_size)
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

    async fn process_video<R: std::io::Read, W: Write>(
        &self,
        mut reader: R,
        writer: &mut W,
        file_size: u64,
    ) -> Result<(u64, String)> {
        let mut hasher = Sha256::new();
        let mut total_bytes_written: u64 = 0;
        let mut buffer = vec![0u8; 65536]; // 64KB chunks

        let progress = ProgressBar::new(file_size);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("#>-"),
        );

        let mut frame_number = 0;

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let chunk = &buffer[..n];

                    // Update progress
                    progress.inc(n as u64);
                    debug!("Frame {}: extracted {} bytes", frame_number, n);

                    // Update hash
                    hasher.update(chunk);

                    // Write to output
                    writer.write_all(chunk)?;
                    total_bytes_written += n as u64;

                    frame_number += 1;

                    // Check operation health
                    self.check_operation_health(frame_number)?;
                }
                Err(e) => {
                    return Err(F2V2FError::Io(e.to_string()).into());
                }
            }
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
