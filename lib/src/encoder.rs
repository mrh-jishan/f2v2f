use crate::error::{F2V2FError, Result, ErrorContext};
use crate::config::EncodeConfig;
use crate::image_generator::GeometricArtGenerator;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use tracing::{info, warn, debug};

/// Encodes a file into a video with artistic frames
pub struct Encoder {
    config: EncodeConfig,
}

/// Information about encoded file
#[derive(Debug, Clone)]
pub struct EncodedFileInfo {
    pub original_file_size: u64,
    pub checksum: String,
    pub num_frames: u64,
    pub chunk_size: usize,
    pub art_style: String,
}

impl Encoder {
    pub fn new(config: EncodeConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Encode a file to video
    pub async fn encode<P: AsRef<Path>>(&self, input: P) -> Result<(EncodedFileInfo, Vec<u8>)> {
        let path = input.as_ref();
        
        info!("Starting file encoding: {}", path.display());

        // Get file information
        let file_size = std::fs::metadata(path)?.len();
        info!("File size: {} bytes ({:.2} MB)", file_size, file_size as f64 / 1024.0 / 1024.0);

        if file_size == 0 {
            return Err(F2V2FError::InvalidInput("Cannot encode empty files".to_string()));
        }

        // Calculate number of frames needed
        let bytes_per_frame = self.config.width as usize * self.config.height as usize * 3; // RGB
        let num_frames = ((file_size as usize + self.config.chunk_size - 1) / self.config.chunk_size) as u64;
        
        info!("Will generate {} frames", num_frames);

        // Read and process file
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let (checksum, encoded_data) = self
            .process_file(reader, file_size, num_frames)
            .await?;

        let info = EncodedFileInfo {
            original_file_size: file_size,
            checksum,
            num_frames,
            chunk_size: self.config.chunk_size,
            art_style: self.config.art_style.clone(),
        };

        info!("Encoding complete. Checksum: {}", info.checksum);

        Ok((info, encoded_data))
    }

    async fn process_file(
        &self,
        mut reader: BufReader<File>,
        file_size: u64,
        num_frames: u64,
    ) -> Result<(String, Vec<u8>)> {
        let mut hasher = Sha256::new();
        let mut all_data = Vec::new();
        let mut buffer = vec![0u8; self.config.chunk_size];

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
                    debug!("Frame {}: processed {} bytes", frame_number, n);

                    // Update hash
                    hasher.update(chunk);

                    // Store data (in real implementation, would generate video frame here)
                    all_data.extend_from_slice(chunk);

                    frame_number += 1;

                    // Check for cancellation or timeouts
                    self.check_operation_health(frame_number)?;
                }
                Err(e) => {
                    return Err(F2V2FError::Io(e.to_string()).into());
                }
            }
        }

        progress.finish_with_message("Encoding complete!");

        let checksum = format!("{:x}", hasher.finalize());
        Ok((checksum, all_data))
    }

    fn check_operation_health(&self, frame_number: u64) -> Result<()> {
        // Check for memory pressure, cancellation signals, etc.
        // This would be expanded with actual monitoring
        if frame_number % 100 == 0 {
            debug!("Health check at frame {}", frame_number);
        }
        Ok(())
    }

    /// Estimate the video file size
    pub fn estimate_video_size(&self, file_size: u64) -> u64 {
        let num_frames = (file_size + self.config.chunk_size as u64 - 1) / self.config.chunk_size as u64;
        let bytes_per_frame = (self.config.width as u64) * (self.config.height as u64) * 3;
        
        // Estimate with video codec compression (assume ~50% compression)
        let raw_size = num_frames * bytes_per_frame;
        raw_size / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_encoder_creation() {
        let config = EncodeConfig::default();
        let encoder = Encoder::new(config).unwrap();
        assert!(encoder.config.validate().is_ok());
    }

    #[tokio::test]
    async fn test_estimate_video_size() {
        let config = EncodeConfig::default();
        let encoder = Encoder::new(config).unwrap();
        
        let estimated = encoder.estimate_video_size(1024 * 1024); // 1MB
        assert!(estimated > 0);
    }

    #[tokio::test]
    async fn test_encode_small_file() -> Result<()> {
        let config = EncodeConfig::default();
        let encoder = Encoder::new(config)?;

        let mut file = NamedTempFile::new()?;
        file.write_all(b"Hello, world!")?;
        file.flush()?;

        let (info, _data) = encoder.encode(file.path()).await?;
        
        assert_eq!(info.original_file_size, 13);
        assert!(!info.checksum.is_empty());
        assert!(info.num_frames > 0);

        Ok(())
    }
}
