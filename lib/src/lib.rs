//! # f2v2f - File to Video to File
//!
//! A creative and innovative tool that converts any file into a beautiful video
//! with geometric art frames, and decodes the video back to the original file.
//!
//! ## Quick Start
//!
//! ### Encoding a file:
//! ```ignore
//! use f2v2f::config::EncodeConfig;
//! use f2v2f::encoder::Encoder;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = EncodeConfig::default();
//!     let encoder = Encoder::new(config)?;
//!     let (info, data) = encoder.encode("file.bin").await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Decoding a video:
//! ```ignore
//! use f2v2f::config::DecodeConfig;
//! use f2v2f::decoder::Decoder;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = DecodeConfig::default();
//!     let decoder = Decoder::new(config)?;
//!     let info = decoder.decode("video.mp4", "output.bin").await?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod decoder;
pub mod encoder;
pub mod error;
pub mod image_generator;
pub mod video_composer;
pub mod ffi;

pub use error::Result;
pub use encoder::Encoder;
pub use decoder::Decoder;
pub use config::{EncodeConfig, DecodeConfig};
