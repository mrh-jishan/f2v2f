#!/usr/bin/env python3
"""
Example: Using f2v2f Python binding directly
"""

import sys
from pathlib import Path

# Add bindings to path
sys.path.insert(0, str(Path(__file__).parent / "bindings"))

from python.f2v2f import Encoder, Decoder, version, EncodingError, DecodingError

def main():
    print(f"f2v2f version: {version()}")
    print()
    
    # Create a test file
    test_file = Path("example_input.txt")
    test_file.write_text("Hello, World! This is a test file for f2v2f encoding.")
    
    output_video = Path("example_output.mp4")
    recovered_file = Path("example_recovered.txt")
    
    try:
        # Encoding example
        print("📹 Encoding file to video...")
        encoder = Encoder(width=1920, height=1080, fps=30, chunk_size=65536)
        
        def progress_callback(total_bytes, total_frames, message):
            print(f"   Progress: {total_frames} frames, {total_bytes} bytes - {message}")
        
        encoder.encode(str(test_file), str(output_video), progress_callback)
        print(f"✅ Encoding complete! Video: {output_video}")
        print()
        
        # Decoding example
        print("🎬 Decoding video back to file...")
        decoder = Decoder()
        decoder.decode(str(output_video), str(recovered_file), progress_callback)
        print(f"✅ Decoding complete! File: {recovered_file}")
        print()
        
        # Verify
        original_content = test_file.read_text()
        recovered_content = recovered_file.read_text()
        
        if original_content == recovered_content:
            print("✅ Verification: Files match perfectly!")
        else:
            print("❌ Verification failed: Files don't match")
            return 1
        
    except EncodingError as e:
        print(f"❌ Encoding error: {e}")
        return 1
    except DecodingError as e:
        print(f"❌ Decoding error: {e}")
        return 1
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        return 1
    finally:
        # Cleanup test files
        test_file.unlink(missing_ok=True)
        output_video.unlink(missing_ok=True)
        recovered_file.unlink(missing_ok=True)
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
