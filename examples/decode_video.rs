/// Example: Decoding a video back to a file
use f2v2f::config::DecodeConfig;
use f2v2f::decoder::Decoder;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Create configuration
    let config = DecodeConfig::default();

    // Create decoder
    let decoder = match Decoder::new(config) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to create decoder: {}", e);
            return;
        }
    };

    // Decode video
    let video_file = "output.mp4";
    let output_file = "reconstructed.bin";

    let decode_info = match decoder.decode(video_file, output_file).await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Decoding failed: {}", e);
            return;
        }
    };

    // Print results
    println!("Decoding successful!");
    println!("  Extracted size: {} bytes", decode_info.extracted_size);
    println!("  Checksum: {}", decode_info.checksum);

    // Verify with original checksum (example)
    let original_checksum = "12345abcde67890"; // Would come from encode operation
    match decoder.verify_checksum(output_file, original_checksum) {
        Ok(true) => println!("  ✓ Checksum matches! File integrity verified."),
        Ok(false) => println!("  ✗ Checksum mismatch! File may be corrupted."),
        Err(e) => eprintln!("  Error verifying checksum: {}", e),
    }
}
