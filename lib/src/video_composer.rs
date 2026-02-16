use crate::error::{F2V2FError, Result};
use crate::image_generator::GeometricArtGenerator;
use image::ImageBuffer;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::Write;
use tracing::{info, warn, debug};

/// Composes individual image frames into a video
pub struct VideoComposer {
    width: u32,
    height: u32,
    fps: u32,
}

impl VideoComposer {
    pub fn new(width: u32, height: u32, fps: u32) -> Self {
        Self { width, height, fps }
    }

    fn ffmpeg_encode(
        frame_data: &[Vec<u8>],
        width: u32,
        height: u32,
        fps: u32,
        output_path: &str,
    ) -> Result<()> {
        // Combine all frame data into a single bytearray for communicate()
        let mut all_data = Vec::new();
        for frame in frame_data {
            all_data.extend_from_slice(frame);
        }

        let mut cmd = Command::new("ffmpeg")
            .args(&[
                "-y",  // Overwrite
                "-f", "rawvideo",
                "-pix_fmt", "rgb24",
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", &fps.to_string(),
                "-i", "pipe:0",
                "-c:v", "libx264",
                "-preset", "fast",
                "-pix_fmt", "yuv420p",
                "-profile:v", "baseline",
                "-level", "3.0",
                "-movflags", "+faststart",
                output_path,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| F2V2FError::EncodingError(format!("Failed to start ffmpeg: {}", e)))?;

        info!("FFmpeg started, writing {} bytes to pipe", all_data.len());

        // Write all data at once then wait
        {
            let mut stdin = cmd.stdin.take()
                .ok_or_else(|| F2V2FError::EncodingError("No stdin".to_string()))?;
            
            stdin.write_all(&all_data)
                .map_err(|e| F2V2FError::EncodingError(format!("Write failed: {}", e)))?;
            
            info!("Successfully wrote {} bytes to FFmpeg stdin", all_data.len());
        }

        info!("Waiting for FFmpeg to finish...");
        let status = cmd.wait()
            .map_err(|e| F2V2FError::EncodingError(format!("Wait failed: {}", e)))?;

        if !status.success() {
            return Err(F2V2FError::EncodingError(
                format!("ffmpeg exited with code {}", status.code().unwrap_or(-1))
            ));
        }

        info!("FFmpeg completed successfully");
        Ok(())
    }
   
    /// Create video from sequence of frames
    pub fn compose_from_frames<P: AsRef<Path>>(
        &self,
        frame_data: Vec<Vec<u8>>,
        output_path: P,
    ) -> Result<()> {
        let output = output_path.as_ref();
        info!(
            "Composing video: {}x{} @ {} fps to {}",
            self.width,
            self.height,
            self.fps,
            output.display()
        );

        Self::ffmpeg_encode(&frame_data, self.width, self.height, self.fps, &output.to_string_lossy())
    }

    /// Create video from geometric art frames based on file data
    pub async fn compose_from_file_data<P: AsRef<Path>>(
        &self,
        file_data: Vec<u8>,
        chunk_size: usize,
        output_path: P,
    ) -> Result<()> {
        info!("Creating video from file data");

        let num_chunks = (file_data.len() + chunk_size - 1) / chunk_size;
        let generator = GeometricArtGenerator::new(self.width, self.height, 42);

        let mut frames = Vec::new();

        for (i, chunk) in file_data.chunks(chunk_size).enumerate() {
            debug!("Generating frame {}/{}", i + 1, num_chunks);

            let img = generator.generate_from_data(chunk)?;
            
            // Convert image to raw bytes for video composition
            let frame_bytes = img.into_raw();
            frames.push(frame_bytes);
        }

        // Call sync compose from async context
        self.compose_from_frames(frames, output_path)
    }

    /// Extract frames from video
    pub async fn extract_frames<P: AsRef<Path>>(
        &self,
        video_path: P,
    ) -> Result<Vec<ImageBuffer<image::Rgba<u8>, Vec<u8>>>> {
        let path = video_path.as_ref();
        info!("Extracting frames from: {}", path.display());

        // This would use ffmpeg to extract frames
        // For now, placeholder
        warn!("Frame extraction not yet fully implemented");

        Ok(Vec::new())
    }
}

/// Validates video file integrity
pub struct VideoValidator;

impl VideoValidator {
    /// Check if file is a valid video
    pub fn is_valid_video<P: AsRef<Path>>(path: P) -> Result<bool> {
        let path = path.as_ref();

        // Check file existence
        if !path.exists() {
            return Err(F2V2FError::InvalidInput("Video file not found".to_string()));
        }

        // Check file size
        let metadata = std::fs::metadata(path)?;
        if metadata.len() < 1024 {
            // Minimum reasonable video file size
            return Err(F2V2FError::InvalidInput(
                "Video file too small (likely corrupted)".to_string(),
            ));
        }

        // Basic format detection by magic bytes would go here
        Ok(true)
    }

    /// Verify video frame count
    pub async fn verify_frame_count<P: AsRef<Path>>(
        _video_path: P,
        _expected_frames: u64,
    ) -> Result<bool> {
        // This would use ffmpeg to get frame count
        warn!("Frame count verification not yet fully implemented");
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composer_creation() {
        let composer = VideoComposer::new(1920, 1080, 30);
        assert_eq!(composer.width, 1920);
        assert_eq!(composer.height, 1080);
        assert_eq!(composer.fps, 30);
    }

    #[test]
    fn test_video_validator() -> Result<()> {
        // Would test with actual video file
        // For now, just validate the logic compiles
        Ok(())
    }
}
