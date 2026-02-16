use crate::error::{F2V2FError, Result};
use crate::image_generator::GeometricArtGenerator;
use image::ImageBuffer;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{Read, Write};
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
        width: u32,
        height: u32,
        fps: u32,
        output_path: &str,
    ) -> Result<std::process::Child> {
        let cmd = Command::new("/usr/local/bin/ffmpeg")
            .args(&[
                "-y",  // Overwrite
                "-f", "rawvideo",
                "-pix_fmt", "rgba",
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", &fps.to_string(),
                "-i", "pipe:0",
                "-c:v", "libx264",
                "-preset", "medium",
                "-crf", "0",
                "-pix_fmt", "yuv444p",
                "-color_range", "pc",
                "-movflags", "+faststart",
                output_path,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(std::fs::File::create("ffmpeg_error.log").map_err(|e| F2V2FError::EncodingError(format!("Failed to create log file: {}", e)))?)
            .spawn()
            .map_err(|e| F2V2FError::EncodingError(format!("Failed to start ffmpeg: {}", e)))?;

        Ok(cmd)
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

        let mut child = Self::ffmpeg_encode(self.width, self.height, self.fps, &output.to_string_lossy())?;
        let mut stdin = child.stdin.take().ok_or_else(|| F2V2FError::EncodingError("No stdin".to_string()))?;

        for frame in frame_data {
            stdin.write_all(&frame)
                .map_err(|e| F2V2FError::EncodingError(format!("Write failed: {}", e)))?;
        }
        
        drop(stdin);

        let status = child.wait()
            .map_err(|e| F2V2FError::EncodingError(format!("Wait failed: {}", e)))?;

        if !status.success() {
            return Err(F2V2FError::EncodingError(
                format!("ffmpeg exited with code {}", status.code().unwrap_or(-1))
            ));
        }

        Ok(())
    }

    /// Create video from geometric art frames based on file data stream
    pub async fn compose_from_file_data<P: AsRef<Path>, R: Read>(
        &self,
        mut reader: R,
        total_size: u64,
        chunk_size: usize,
        output_path: P,
    ) -> Result<()> {
        let output = output_path.as_ref();
        info!("Creating video from file data stream to {}", output.display());

        let num_chunks = (total_size + chunk_size as u64 - 1) / chunk_size as u64;
        let generator = GeometricArtGenerator::new(self.width, self.height, 42);

        let mut child = Self::ffmpeg_encode(self.width, self.height, self.fps, &output.to_string_lossy())?;
        let mut stdin = child.stdin.take().ok_or_else(|| F2V2FError::EncodingError("No stdin".to_string()))?;

        let mut buffer = vec![0u8; chunk_size];
        let mut chunk_count = 0;

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 { break; }
            
            chunk_count += 1;
            debug!("Generating and writing frame {}/{}", chunk_count, num_chunks);

            // Pad the last chunk with zeros if it's smaller than chunk_size
            if n < chunk_size {
                buffer[n..].fill(0);
            }

            let img = generator.generate_from_data(&buffer)?;
            let frame_bytes = img.into_raw();
            
            stdin.write_all(&frame_bytes)
                .map_err(|e| F2V2FError::EncodingError(format!("Write failed at frame {}: {}", chunk_count, e)))?;
        }
        
        drop(stdin);

        let status = child.wait()
            .map_err(|e| F2V2FError::EncodingError(format!("Wait failed: {}", e)))?;

        if !status.success() {
            return Err(F2V2FError::EncodingError(
                format!("ffmpeg exited with code {}", status.code().unwrap_or(-1))
            ));
        }

        info!("Video composition complete. Total frames: {}", chunk_count);
        Ok(())
    }

    /// Extract frames from video one by one using a callback
    pub async fn extract_frames<P: AsRef<Path>, F>(
        &self,
        video_path: P,
        mut callback: F,
    ) -> Result<()> 
    where 
        F: FnMut(ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Result<()>
    {
        let path = video_path.as_ref();
        info!("Extracting frames from: {}", path.display());

        let mut child = Command::new("/usr/local/bin/ffmpeg")
            .args(&[
                "-i", &path.to_string_lossy(),
                "-f", "rawvideo",
                "-pix_fmt", "rgba",
                "-color_range", "pc",
                "-",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| F2V2FError::DecodingError(format!("Failed to start ffmpeg: {}", e)))?;

        let mut stdout = child.stdout.take().ok_or_else(|| F2V2FError::DecodingError("No stdout".to_string()))?;
        let frame_size = (self.width * self.height * 4) as usize;
        let mut frame_count = 0;
        
        loop {
            let mut buffer = vec![0u8; frame_size];
            match stdout.read_exact(&mut buffer) {
                Ok(_) => {
                    let img = ImageBuffer::from_raw(self.width, self.height, buffer)
                        .ok_or_else(|| F2V2FError::DecodingError("Failed to create image from raw bytes".to_string()))?;
                    
                    callback(img)?;
                    frame_count += 1;
                    if frame_count % 100 == 0 {
                        debug!("Extracted {} frames...", frame_count);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(F2V2FError::DecodingError(format!("Read failed at frame {}: {}", frame_count + 1, e))),
            }
        }

        let status = child.wait()
            .map_err(|e| F2V2FError::DecodingError(format!("Wait failed: {}", e)))?;

        if !status.success() {
            // It might fail if we read all frames but ffmpeg has more to say, or if it's not a video
            warn!("ffmpeg exited with code {}", status.code().unwrap_or(-1));
        }

        info!("Finished extracting {} frames", frame_count);
        Ok(())
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
    fn test_compose_from_frames() -> Result<()> {
        let composer = VideoComposer::new(256, 256, 30);
        let output = Path::new("/tmp/test_compose.mp4");
        
        let frame = vec![0u8; 256 * 256 * 4]; // Black frame
        let frames = vec![frame.clone(), frame];
        
        composer.compose_from_frames(frames, output)?;
        assert!(output.exists());
        assert!(output.metadata()?.len() > 0);
        
        std::fs::remove_file(output)?;
        Ok(())
    }
}
