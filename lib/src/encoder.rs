use crate::error::{F2V2FError, Result};
use crate::config::EncodeConfig;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tracing::info;
use zstd::stream::write::Encoder as ZstdEncoder;

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
    pub encoded_size: u64,  // Size after compression (if enabled)
    pub compression_ratio: f32,  // Original / Compressed
}

impl Encoder {
    pub fn new(config: EncodeConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Encode a file (BLOCKING, NO ASYNC) - Safe for FFI calls
    /// Returns (metadata, compressed_data)
    pub fn encode_blocking<P: AsRef<Path>>(&self, input: P) -> Result<(EncodedFileInfo, Vec<u8>)> {
        let input_path = input.as_ref();
        let file_size = std::fs::metadata(input_path)?.len();
        
        if file_size == 0 {
            return Err(F2V2FError::InvalidInput("Cannot encode empty files".to_string()));
        }

        info!("📁 Encoding file: {} ({} bytes)", input_path.display(), file_size);

        let mut file = File::open(input_path)?;
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data)?;

        // Calculate checksum of original data
        let mut hasher = Sha256::new();
        hasher.update(&file_data);
        let checksum = format!("{:x}", hasher.finalize());

        // Compress if enabled
        let encoded_data = if self.config.use_compression {
            info!("🗜️  Compressing with Zstd (compression_level={})", self.config.compression_level);
            let mut encoder = ZstdEncoder::new(Vec::new(), self.config.compression_level)?;
            encoder.multithread(num_cpus::get() as u32)?;
            encoder.write_all(&file_data)?;
            let compressed = encoder.finish()?;
            info!(
                "✅ Compression: {} bytes → {} bytes ({:.2}x ratio)", 
                file_size, 
                compressed.len(),
                (file_size as f32 / compressed.len() as f32)
            );
            compressed
        } else {
            info!("⏭️  Compression disabled, using raw data");
            file_data.clone()
        };

        let encoded_size = encoded_data.len() as u64;
        
        // Calculate optimal chunk size to limit frame count
        let max_frames = 1000;
        let optimal_chunk_size = std::cmp::max(
            self.config.chunk_size as u64,
            ((encoded_size + (max_frames - 1)) / max_frames) as u64,
        ) as usize;
        
        if optimal_chunk_size > self.config.chunk_size {
            info!("📊 Automatically adjusted chunk size: {} → {} bytes ({} frames)",
                self.config.chunk_size,
                optimal_chunk_size,
                (encoded_size + optimal_chunk_size as u64 - 1) / optimal_chunk_size as u64
            );
        }

        let compression_ratio = file_size as f32 / encoded_size as f32;
        let num_frames = (encoded_size + optimal_chunk_size as u64 - 1) / optimal_chunk_size as u64;

        let info = EncodedFileInfo {
            original_file_size: file_size,
            checksum,
            num_frames,
            chunk_size: optimal_chunk_size,
            art_style: self.config.art_style.clone(),
            encoded_size,
            compression_ratio,
        };

        info!("📊 Encoding complete: {} frames needed (ratio: {:.2}x)", num_frames, compression_ratio);

        Ok((info, encoded_data))
    }

    /// Encode a file: read, compress (optional), and return data
    /// Returns (metadata, compressed_data)
    /// 
    /// For large files, automatically adjusts chunk size to limit frame count:
    /// - Target: Keep frames under ~1000 to avoid excessive memory usage
    /// - Each frame requires 1920×1080×4 = 8.29MB raw data
    /// - Max frames: 1000 = 8.2GB max memory per encoding
    pub async fn encode<P: AsRef<Path>>(&self, input: P) -> Result<(EncodedFileInfo, Vec<u8>)> {
        // Simply call the blocking version
        self.encode_blocking(input)
    }

    /// Estimate the video file size based on input, accounting for compression
    /// 
    /// **Calculation:**
    /// 1. Estimate Zstd compression (2-4x for typical data)
    /// 2. Calculate frames needed from compressed size
    /// 3. Estimate H.265 video compression (~50% of raw frame data)
    /// 
    /// **Typical ratios:**
    /// - Text/JSON: 3-4x compression
    /// - Binary data: 1.5-2x compression  
    /// - Video codec: ~50% additional compression
    pub fn estimate_video_size(&self, file_size: u64) -> u64 {
        // Estimate compressed size
        let estimated_compressed = if self.config.use_compression {
            // Zstd typically achieves 2-4x compression for text/data, 1.1-1.5x for binary
            (file_size as f32 * 0.3) as u64  // Conservative: 30% of original
        } else {
            file_size
        };
        
        // Calculate frames needed
        let num_frames = (estimated_compressed + self.config.chunk_size as u64 - 1) / self.config.chunk_size as u64;
        let bytes_per_frame = (self.config.width as u64) * (self.config.height as u64) * 4;  // RGBA
        
        // Estimate with H.264 video codec compression (assume ~50% compression)
        let raw_size = num_frames * bytes_per_frame;
        let estimated_video_size = raw_size / 2;
        
        info!(
            "📈 Size estimate: {} bytes → ~{} bytes (compressed) → {} frames → ~{} MB video",
            file_size,
            estimated_compressed,
            num_frames,
            estimated_video_size / (1024 * 1024)
        );
        
        estimated_video_size
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
    async fn test_encode_small_file_with_compression() -> Result<()> {
        let config = EncodeConfig {
            use_compression: true,
            compression_level: 11,
            ..EncodeConfig::default()
        };
        let encoder = Encoder::new(config)?;

        let mut file = NamedTempFile::new()?;
        file.write_all(b"Hello, world! This is a test.")?;
        file.flush()?;

        let (info, data) = encoder.encode(file.path()).await?;
        
        assert_eq!(info.original_file_size, 29);
        assert!(!info.checksum.is_empty());
        assert!(info.num_frames > 0);
        assert!(data.len() < 29);  // Compression should make it smaller
        assert!(info.compression_ratio > 1.0);

        Ok(())
    }

    #[tokio::test]
    async fn test_encode_without_compression() -> Result<()> {
        let config = EncodeConfig {
            use_compression: false,
            ..EncodeConfig::default()
        };
        let encoder = Encoder::new(config)?;

        let mut file = NamedTempFile::new()?;
        file.write_all(b"test data")?;
        file.flush()?;

        let (info, data) = encoder.encode(file.path()).await?;
        
        assert_eq!(info.original_file_size, 9);
        assert_eq!(data.len() as u64, 9);  // No compression
        assert_eq!(info.compression_ratio, 1.0);

        Ok(())
    }
}
