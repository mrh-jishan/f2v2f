#!/usr/bin/env python3
"""Test encoding from Flask-like threaded context"""

import sys
sys.path.insert(0, '/Users/robin-hassan/Desktop/f2v2f/backend')

from f2v2f import Encoder
import os
import threading
from pathlib import Path

# Create a test file
test_file = "/Users/robin-hassan/Desktop/f2v2f/backend/uploads/test_input.txt"
output_file = "/Users/robin-hassan/Desktop/f2v2f/backend/outputs/test_output.mp4"

# Ensure directories exist
Path(test_file).parent.mkdir(parents=True, exist_ok=True)
Path(output_file).parent.mkdir(parents=True, exist_ok=True)

with open(test_file, 'w') as f:
    f.write("Hello, World! This is a test file for f2v2f encoding.\n" * 100)

print(f"Test file created: {test_file} ({os.path.getsize(test_file)} bytes)")
print(f"Output file: {output_file}")

def encode_in_thread():
    try:
        encoder = Encoder(width=1920, height=1080, fps=30, chunk_size=4096)
        print("Encoder created in thread")
        
        print("Starting encoding in thread...")
        encoder.encode(test_file, output_file)
        
        print(f"✅ Encoding successful in thread!")
        print(f"Output file size: {os.path.getsize(output_file)} bytes")
        
    except Exception as e:
        print(f"❌ Encoding failed in thread: {e}")
        import traceback
        traceback.print_exc()

# Run in thread (like Flask does)
print("\n--- Testing in main thread ---")
try:
    encoder = Encoder(width=1920, height=1080, fps=30, chunk_size=4096)
    encoder.encode(test_file, output_file)
    print("✅ Main thread encoding successful!")
except Exception as e:
    print(f"❌ Main thread encoding failed: {e}")

print("\n--- Testing in background thread (Flask-like) ---")
thread = threading.Thread(target=encode_in_thread)
thread.start()
thread.join()

if os.path.exists(output_file):
    size = os.path.getsize(output_file)
    print(f"\n✅ Final output exists: {size} bytes")
else:
    print("\n❌ Final output does not exist")
