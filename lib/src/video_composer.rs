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
                "-c:v", "libx265",
                "-preset", "fast",
                "-crf", "28",
                "-pix_fmt", "yuv420p",
                "-movflags", "+faststart",
                output_path,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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
            let code = status.code().unwrap_or(-1);
            return Err(F2V2FError::EncodingError(
                format!("FFmpeg exited with code {}. This usually means: out of memory, invalid parameters, or disk full. For large files, try reducing chunk_size or lowering video resolution.", code)
            ));
        }

        Ok(())
    }

    /// Create video from geometric art frames based on file data (BLOCKING)
    pub fn compose_from_file_data_blocking<P: AsRef<Path>>(
        &self,
        file_data: Vec<u8>,
        chunk_size: usize,
        output_path: P,
    ) -> Result<()> {
        self.compose_from_file_data_blocking_with_original(file_data, chunk_size, 0, output_path)
    }

    /// Create video from geometric art frames based on file data (BLOCKING)
    /// With optional original file size tracking for metadata
    pub fn compose_from_file_data_blocking_with_original<P: AsRef<Path>>(
        &self,
        file_data: Vec<u8>,
        chunk_size: usize,
        original_size: u64,
        output_path: P,
    ) -> Result<()> {
        let output = output_path.as_ref();
        info!("Creating video from file data to {}", output.display());

        // Write metadata to sidecar file (.meta)
        let meta_path = if let Some(stem) = output.file_stem() {
            let mut meta = stem.to_os_string();
            meta.push(".mp4meta");
            output.parent().map(|p| p.join(&meta)).unwrap_or_else(|| Path::new(&meta).to_path_buf())
        } else {
            output.with_extension("mp4meta")
        };
        
        {
            let mut meta_file = std::fs::File::create(&meta_path)?;
            let original_or_encoded = if original_size > 0 { original_size } else { file_data.len() as u64 };
            let meta = format!("chunk_size={}\ncompressed_size={}\noriginal_size={}\n", 
                chunk_size, file_data.len(), original_or_encoded);
            meta_file.write_all(meta.as_bytes())?;
        }
        info!("📝 Metadata written to {}", meta_path.display());

        let num_chunks = (file_data.len() + chunk_size - 1) / chunk_size;
        let generator = GeometricArtGenerator::new(self.width, self.height, 42);

        // Use FFmpeg encoding without metadata
        let mut child = Self::ffmpeg_encode(self.width, self.height, self.fps, &output.to_string_lossy())?;
        let mut stdin = child.stdin.take().ok_or_else(|| F2V2FError::EncodingError("No stdin".to_string()))?;

        for (i, chunk) in file_data.chunks(chunk_size).enumerate() {
            if (i + 1) % 100 == 0 || (i + 1) == num_chunks {
                info!("  📹 Frame {}/{} ({:.1}%)", i + 1, num_chunks, 
                    ((i + 1) as f32 / num_chunks as f32) * 100.0);
            }

            // Pad the last chunk with zeros if it's smaller than chunk_size
            let mut padded_chunk = chunk.to_vec();
            if padded_chunk.len() < chunk_size {
                padded_chunk.resize(chunk_size, 0);
            }

            {
                let img = generator.generate_from_data(&padded_chunk)?;
                let frame_bytes = img.into_raw();
                
                match stdin.write_all(&frame_bytes) {
                    Ok(_) => {},
                    Err(e) if e.raw_os_error() == Some(32) => {
                        return Err(F2V2FError::EncodingError(
                            format!("FFmpeg pipe broken at frame {}/{} - FFmpeg crashed or ran out of memory. Error: {}", i + 1, num_chunks, e)
                        ));
                    },
                    Err(e) => return Err(F2V2FError::EncodingError(format!("Write failed at frame {}: {}", i + 1, e))),
                }
                // frame_bytes and img are dropped here explicitly
            }
            
            // Explicit cleanup
            padded_chunk.clear();
        }
        
        drop(stdin);

        // Read stderr to capture any FFmpeg errors
        let mut stderr_output = Vec::new();
        if let Some(mut stderr) = child.stderr.take() {
            let _ = stderr.read_to_end(&mut stderr_output);
        }

        let status = child.wait()
            .map_err(|e| F2V2FError::EncodingError(format!("Wait failed: {}", e)))?;

        if !status.success() {
            let err_msg = String::from_utf8_lossy(&stderr_output).to_string();
            debug!("FFmpeg stderr: {}", err_msg);
            return Err(F2V2FError::EncodingError(
                format!("FFmpeg exited with code {}. Details: {}", status.code().unwrap_or(-1), err_msg)
            ));
        }

        info!("Video composition complete");
        Ok(())
    }

    /// Create video from geometric art frames based on file data
    pub async fn compose_from_file_data<P: AsRef<Path>>(
        &self,
        file_data: Vec<u8>,
        chunk_size: usize,
        output_path: P,
    ) -> Result<()> {
        // Call the blocking version in a way that's safe for async
        let output_path_str = output_path.as_ref().to_string_lossy().to_string();
        self.compose_from_file_data_blocking(file_data, chunk_size, &output_path_str)
    }

    /// Extract frames from video
    pub async fn extract_frames<P: AsRef<Path>>(
        &self,
        video_path: P,
    ) -> Result<Vec<ImageBuffer<image::Rgba<u8>, Vec<u8>>>> {
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
        let mut frames = Vec::new();
        let frame_size = (self.width * self.height * 4) as usize;
        
        loop {
            let mut buffer = vec![0u8; frame_size];
            match stdout.read_exact(&mut buffer) {
                Ok(_) => {
                    let img = ImageBuffer::from_raw(self.width, self.height, buffer)
                        .ok_or_else(|| F2V2FError::DecodingError("Failed to create image from raw bytes".to_string()))?;
                    frames.push(img);
                }
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(F2V2FError::DecodingError(format!("Read failed: {}", e))),
            }
        }

        let status = child.wait()
            .map_err(|e| F2V2FError::DecodingError(format!("Wait failed: {}", e)))?;

        if !status.success() {
            // It might fail if we read all frames but ffmpeg has more to say, or if it's not a video
            warn!("ffmpeg exited with code {}", status.code().unwrap_or(-1));
        }

        info!("Extracted {} frames", frames.len());
        Ok(frames)
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
