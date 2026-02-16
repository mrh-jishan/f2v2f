use crate::error::{F2V2FError, Result};
use crate::config::DecodeConfig;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Write, Read, Cursor};
use std::path::Path;
use tracing::info;

/// Decodes a video back to the original file
pub struct Decoder {
    config: DecodeConfig,
}

/// Metadata extracted from encoded video
#[derive(Debug, Clone)]
pub struct DecodedFileInfo {
    pub extracted_size: u64,
    pub checksum: String,
    pub was_compressed: bool,
}

// Zstd magic number: 0x28, 0xB5, 0x2F, 0xFD
const ZSTD_MAGIC: &[u8] = &[0x28, 0xB5, 0x2F, 0xFD];

impl Decoder {
    pub fn new(config: DecodeConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Detect if data is zstd compressed by checking magic bytes
    fn is_zstd_compressed(data: &[u8]) -> bool {
        data.len() >= 4 && &data[0..4] == ZSTD_MAGIC
    }

    /// Decode a video back to file with automatic decompression
    /// 
    /// Process:
    /// 1. Extract all data from video frames
    /// 2. Detect if it's zstd compressed
    /// 3. Decompress if needed
    /// 4. Write original file
    /// 5. Verify checksum
    pub async fn decode<P: AsRef<Path>>(&self, input: P, output: P) -> Result<DecodedFileInfo> {
        let input_path = input.as_ref();
        let output_path = output.as_ref();

        info!("🎬 Starting video extraction from: {}", input_path.display());

        // Extract all frame data from video
        let extracted_data = self.extract_frame_data(input_path).await?;
        info!("✅ Extracted {} bytes from video", extracted_data.len());

        // Detect compression
        let was_compressed = Self::is_zstd_compressed(&extracted_data);
        info!("🔍 Data format: {}", 
            if was_compressed { "Zstd compressed" } else { "Raw" });

        // Decompress if needed
        let final_data = if was_compressed {
            info!("🗜️  Decompressing with Zstd...");
            let mut decoder = zstd::stream::read::Decoder::new(Cursor::new(&extracted_data))?;
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            info!("✅ Decompressed: {} bytes → {} bytes", 
                extracted_data.len(), decompressed.len());
            decompressed
        } else {
            extracted_data.clone()
        };

        // Calculate checksum and write file
        let mut hasher = Sha256::new();
        hasher.update(&final_data);
        let checksum = format!("{:x}", hasher.finalize());

        let mut output_file = File::create(output_path)?;
        output_file.write_all(&final_data)?;
        output_file.sync_all()?;

        info!("💾 Wrote {} bytes to {}", final_data.len(), output_path.display());
        info!("📋 Checksum: {}", checksum);

        Ok(DecodedFileInfo {
            extracted_size: final_data.len() as u64,
            checksum,
            was_compressed,
        })
    }

    /// Extract all data from video frames
    async fn extract_frame_data<P: AsRef<Path>>(&self, video_path: P) -> Result<Vec<u8>> {
        let path = video_path.as_ref();
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

        // Extract frames from video
        let frames = composer.extract_frames(path).await?;
        info!("📸 Extracted {} frames from video", frames.len());

        let mut all_data = Vec::new();
        for (i, frame) in frames.iter().enumerate() {
            let frame_data = generator.decode_from_image(frame, self.config.chunk_size)?;
            all_data.extend_from_slice(&frame_data);
            if (i + 1) % 10 == 0 {
                info!("  Processed {} frames...", i + 1);
            }
        }

        Ok(all_data)
    }

    /// Verify that decoded file matches expected checksum
    pub fn verify_checksum<P: AsRef<Path>>(
        &self,
        file_path: P,
        expected_checksum: &str,
    ) -> Result<bool> {
        let path = file_path.as_ref();
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 1024 * 1024]; // 1MB chunks

        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(e) => {
                    return Err(F2V2FError::Io(e.to_string()));
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

    #[test]
    fn test_decoder_creation() {
        let config = DecodeConfig::default();
        let decoder = Decoder::new(config).unwrap();
        assert!(decoder.config.validate().is_ok());
    }

    #[test]
    fn test_zstd_magic_detection() {
        let zstd_data = vec![0x28, 0xB5, 0x2F, 0xFD, 0x00, 0x00];
        assert!(Decoder::is_zstd_compressed(&zstd_data));

        let raw_data = vec![0x00, 0x01, 0x02, 0x03];
        assert!(!Decoder::is_zstd_compressed(&raw_data));

        let empty = vec![];
        assert!(!Decoder::is_zstd_compressed(&empty));
    }

    #[tokio::test]
    async fn test_verify_checksum() -> Result<()> {
        use tempfile::NamedTempFile;
        use std::io::Write;

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
