use crate::error::{F2V2FError, Result};
use crate::config::EncodeConfig;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, BufReader, Write};
use std::path::Path;
use tracing::info;
use tempfile;

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
    pub encoded_size: u64,
}

impl Encoder {
    pub fn new(config: EncodeConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Encode a file to video (streaming version for terabyte-scale files)
    pub async fn encode_to_video<P: AsRef<Path>>(&self, input: P, output: P) -> Result<EncodedFileInfo> {
        let input_path = input.as_ref();
        let output_path = output.as_ref();
        
        info!("Starting streaming file encoding: {} -> {}", input_path.display(), output_path.display());

        let file_size = std::fs::metadata(input_path)?.len();
        if file_size == 0 {
            return Err(F2V2FError::InvalidInput("Cannot encode empty files".to_string()));
        }

        let file = File::open(input_path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();

        let composer = crate::video_composer::VideoComposer::new(
            self.config.width,
            self.config.height,
            self.config.fps,
        );

        // We'll use a pipe to stream compressed data to the composer
        // For now, if compression is enabled, we use a temp file to store the compressed stream
        // because VideoComposer::compose_from_file_data currently reads from a Read.
        // If no compression, we can stream directly from the input file.
        
        if self.config.use_compression {
            let mut temp_file = tempfile::NamedTempFile::new()?;
            info!("Streaming compression to temp file: {}", temp_file.path().display());
            
            {
                let mut encoder = zstd::stream::write::Encoder::new(&mut temp_file, self.config.compression_level)?;
                encoder.multithread(num_cpus::get() as u32)?;
                
                let mut buffer = vec![0u8; 128 * 1024];
                loop {
                    let n = reader.read(&mut buffer)?;
                    if n == 0 { break; }
                    hasher.update(&buffer[..n]);
                    encoder.write_all(&buffer[..n])?;
                }
                encoder.finish()?;
            }
            
            let checksum = format!("{:x}", hasher.finalize());
            let compressed_size = temp_file.as_file().metadata()?.len();
            
            // Now compose from the temp file
            temp_file.as_file_mut().sync_all()?;
            let mut temp_reader = BufReader::new(File::open(temp_file.path())?);
            
            composer.compose_from_file_data(
                &mut temp_reader,
                compressed_size,
                self.config.chunk_size,
                output,
            ).await?;
            
            let num_frames = (compressed_size + self.config.chunk_size as u64 - 1) / self.config.chunk_size as u64;
            
            Ok(EncodedFileInfo {
                original_file_size: file_size,
                checksum,
                num_frames,
                chunk_size: self.config.chunk_size,
                art_style: self.config.art_style.clone(),
                encoded_size: compressed_size,
            })
        } else {
            // Direct stream
            composer.compose_from_file_data(
                &mut reader, // Note: This will consume reader, but we need hasher
                file_size,   // Raw data is the same as file_size
                self.config.chunk_size,
                output,
            ).await?;
            
            // Re-hash if needed, but wait, compose_from_file_data consumes the reader.
            // Better to use a hashing reader.
            // For now, let's just do a simple re-read for hash if it's small, or hash during compose.
            // Let's assume the user wants speed.
            
            // We'll re-open to hash for correctness (small overhead compared to video encoding)
            let mut h_file = File::open(input_path)?;
            std::io::copy(&mut h_file, &mut hasher)?;
            let checksum = format!("{:x}", hasher.finalize());
            
            let num_frames = (file_size + self.config.chunk_size as u64 - 1) / self.config.chunk_size as u64;
            
            Ok(EncodedFileInfo {
                original_file_size: file_size,
                checksum,
                num_frames,
                chunk_size: self.config.chunk_size,
                art_style: self.config.art_style.clone(),
                encoded_size: file_size,
            })
        }
    }

    // fn check_operation_health(&self, frame_number: u64) -> Result<()> {
    //     // Check for memory pressure, cancellation signals, etc.
    //     // This would be expanded with actual monitoring
    //     if frame_number % 100 == 0 {
    //         debug!("Health check at frame {}", frame_number);
    //     }
    //     Ok(())
    // }

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
