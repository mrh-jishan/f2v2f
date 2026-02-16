/// Example: Encoding a file to video
use f2v2f::config::EncodeConfig;
use f2v2f::encoder::Encoder;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Create configuration
    let mut config = EncodeConfig::default();
    config.width = 1920;
    config.height = 1080;
    config.fps = 30;
    config.chunk_size = 65536; // 64KB
    config.art_style = "geometric".to_string();

    // Create encoder
    let encoder = match Encoder::new(config) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to create encoder: {}", e);
            return;
        }
    };

    // Encode file
    let input_file = "input.bin";
    let (file_info, _encoded_data) = match encoder.encode(input_file).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Encoding failed: {}", e);
            return;
        }
    };

    // Print results
    println!("Encoding successful!");
    println!("  Original size: {} bytes", file_info.original_file_size);
    println!("  Number of frames: {}", file_info.num_frames);
    println!("  Checksum: {}", file_info.checksum);

    // Estimate video size
    let estimated_video_size = encoder.estimate_video_size(file_info.original_file_size);
    println!("  Estimated video size: {} MB", estimated_video_size / 1024 / 1024);
}
