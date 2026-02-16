import os
import sys
from pathlib import Path
import hashlib

# Add backend to path to import f2v2f
sys.path.append(str(Path(__file__).parent))

from f2v2f import Encoder, Decoder

def calculate_checksum(filepath):
    sha256_hash = hashlib.sha256()
    with open(filepath, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()

def test_full_cycle():
    # Setup paths
    input_file = Path("/tmp/test_f2v2f_cycle_input.bin")
    video_file = Path("/tmp/test_f2v2f_cycle_video.mp4")
    output_file = Path("/tmp/test_f2v2f_cycle_output.bin")

    # Create test data (small enough to be fast, but with varied content)
    test_data = os.urandom(1024 * 10) # 10KB
    with open(input_file, "wb") as f:
        f.write(test_data)
    
    input_checksum = calculate_checksum(input_file)
    print(f"Input file created: {input_file} ({len(test_data)} bytes)")
    print(f"Input checksum: {input_checksum}")

    # Encode
    print("\n--- Encoding ---")
    encoder = Encoder(width=1280, height=720, fps=30, chunk_size=4096)
    encoder.encode(str(input_file), str(video_file))
    print(f"Video created: {video_file} ({video_file.stat().st_size} bytes)")

    # Decode
    print("\n--- Decoding ---")
    decoder = Decoder(width=1280, height=720, chunk_size=4096)
    decoder.decode(str(video_file), str(output_file))
    print(f"Decoded file created: {output_file} ({output_file.stat().st_size} bytes)")

    # Verify
    print("\n--- Verification ---")
    output_checksum = calculate_checksum(output_file)
    print(f"Output checksum: {output_checksum}")

    if input_checksum == output_checksum:
        print("\n✅ SUCCESS: Full cycle completed. Files are identical.")
    else:
        print("\n❌ FAILURE: Checksum mismatch.")
        print(f"Expected: {input_checksum}")
        print(f"Actual:   {output_checksum}")

    # Cleanup
    for f in [input_file, video_file, output_file]:
        if f.exists():
            f.unlink()

if __name__ == "__main__":
    try:
        test_full_cycle()
    except Exception as e:
        print(f"\n❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
