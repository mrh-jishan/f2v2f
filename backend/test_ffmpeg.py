import subprocess
import os

def test_ffmpeg():
    print("Testing ffmpeg from python...")
    try:
        # Test basic version
        cmd = ["/usr/local/bin/ffmpeg", "-version"]
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode == 0:
            print("✅ ffmpeg -version success")
            print(result.stdout.split('\n')[0])
        else:
            print(f"❌ ffmpeg -version failed with return code {result.returncode}")
            print(result.stderr)
            
        # Test dylib issues by attempting a small encode
        print("\nTesting small encode...")
        cmd = [
            "/usr/local/bin/ffmpeg", "-y", "-f", "rawvideo", "-pix_fmt", "rgba", 
            "-video_size", "64x64", "-framerate", "30", "-i", "-", 
            "-c:v", "libx264", "-preset", "ultrafast", "-pix_fmt", "yuv420p", "/tmp/test.mp4"
        ]
        process = subprocess.Popen(cmd, stdin=subprocess.PIPE, stderr=subprocess.PIPE)
        # Send one frame
        frame = b'\x00' * (64 * 64 * 4)
        stdout, stderr = process.communicate(input=frame)
        
        if process.returncode == 0:
            print("✅ Small encode success")
        else:
            print(f"❌ Small encode failed with return code {process.returncode}")
            print(stderr.decode())

    except Exception as e:
        print(f"❌ Exception: {e}")

if __name__ == "__main__":
    test_ffmpeg()
