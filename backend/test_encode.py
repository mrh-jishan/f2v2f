#!/usr/bin/env python3
"""Test encoding directly to see FFmpeg output"""

import sys
sys.path.insert(0, '/Users/robin-hassan/Desktop/f2v2f/backend')

from f2v2f import Encoder
import os

# Create a small test file
test_file = "/tmp/test_f2v2f_input.txt"
output_file = "/tmp/test_f2v2f_output.mp4"

with open(test_file, 'w') as f:
    f.write("Hello, World! This is a test file for f2v2f encoding.\n" * 100)

print(f"Test file created: {test_file} ({os.path.getsize(test_file)} bytes)")
print(f"Output file: {output_file}")

try:
    encoder = Encoder(width=1920, height=1080, fps=30, chunk_size=4096)
    print("Encoder created successfully")
    
    print("\nStarting encoding...")
    encoder.encode(test_file, output_file)
    
    print(f"\n✅ Encoding successful!")
    print(f"Output file size: {os.path.getsize(output_file)} bytes")
    
except Exception as e:
    print(f"\n❌ Encoding failed: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)
