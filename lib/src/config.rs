use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::error::{F2V2FError, Result};

/// Configuration for encoding operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodeConfig {
    /// Resolution width
    pub width: u32,
    /// Resolution height
    pub height: u32,
    /// Frames per second
    pub fps: u32,
    /// Chunk size in bytes for processing
    pub chunk_size: usize,
    /// Art style (geometric, fractal, noise)
    pub art_style: String,
    /// Number of worker threads
    pub num_threads: usize,
    /// Buffer size for reading file
    pub buffer_size: usize,
}

impl Default for EncodeConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 30,
            chunk_size: 4096,         // 4KB - ensures multiple frames even for small files
            art_style: "geometric".to_string(),
            num_threads: num_cpus::get(),
            buffer_size: 1024 * 1024, // 1MB
        }
    }
}

impl EncodeConfig {
    /// Parse resolution string (format: WIDTHxHEIGHT)
    pub fn parse_resolution(resolution: &str) -> Result<(u32, u32)> {
        let parts: Vec<&str> = resolution.split('x').collect();
        if parts.len() != 2 {
            return Err(F2V2FError::InvalidInput(
                "Resolution must be in format WIDTHxHEIGHT (e.g., 1920x1080)".to_string(),
            ));
        }

        let width = u32::from_str(parts[0])
            .map_err(|_| F2V2FError::InvalidInput("Invalid width".to_string()))?;
        let height = u32::from_str(parts[1])
            .map_err(|_| F2V2FError::InvalidInput("Invalid height".to_string()))?;

        if width < 256 || height < 256 {
            return Err(F2V2FError::InvalidInput(
                "Minimum resolution is 256x256".to_string(),
            ));
        }

        if width > 7680 || height > 4320 {
            return Err(F2V2FError::InvalidInput(
                "Maximum resolution is 7680x4320 (8K)".to_string(),
            ));
        }

        Ok((width, height))
    }

    pub fn validate(&self) -> Result<()> {
        if self.fps == 0 || self.fps > 120 {
            return Err(F2V2FError::ConfigError(
                "FPS must be between 1 and 120".to_string(),
            ));
        }

        if self.chunk_size == 0 || self.chunk_size > 10 * 1024 * 1024 {
            // Max 10MB chunks
            return Err(F2V2FError::ConfigError(
                "Chunk size must be between 1 and 10485760 bytes".to_string(),
            ));
        }

        if self.num_threads == 0 {
            return Err(F2V2FError::ConfigError(
                "Number of threads must be at least 1".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeConfig {
    /// Resolution width
    pub width: u32,
    /// Resolution height
    pub height: u32,
    /// Chunk size in bytes
    pub chunk_size: usize,
    /// Number of worker threads
    pub num_threads: usize,
    /// Buffer size for processing
    pub buffer_size: usize,
    /// Verify checksum after decoding
    pub verify_checksum: bool,
}

impl Default for DecodeConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            chunk_size: 4096,
            num_threads: num_cpus::get(),
            buffer_size: 1024 * 1024, // 1MB
            verify_checksum: true,
        }
    }
}

impl DecodeConfig {
    pub fn validate(&self) -> Result<()> {
        if self.num_threads == 0 {
            return Err(F2V2FError::ConfigError(
                "Number of threads must be at least 1".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_resolution() {
        assert_eq!(EncodeConfig::parse_resolution("1920x1080").unwrap(), (1920, 1080));
        assert_eq!(EncodeConfig::parse_resolution("3840x2160").unwrap(), (3840, 2160));
        assert!(EncodeConfig::parse_resolution("invalid").is_err());
        assert!(EncodeConfig::parse_resolution("100x100").is_err()); // Too small
    }

    #[test]
    fn test_validate_config() {
        let config = EncodeConfig::default();
        assert!(config.validate().is_ok());

        let mut bad_config = EncodeConfig::default();
        bad_config.fps = 0;
        assert!(bad_config.validate().is_err());
    }
}
