# F2V2F Complete Guide - Everything You Need to Know

## 📚 Quick Navigation

- **For Quick Start:** See [QUICK_START.md](#quick-start-below)
- **For Building Rust:** See [BUILD_AND_RUN.md](BUILD_AND_RUN.md#-building-the-rust-library)
- **For All Fixes:** See [FIXES_APPLIED.md](FIXES_APPLIED.md)
- **For Architecture:** See [ARCHITECTURE.md](ARCHITECTURE.md)

---

## 🚀 QUICK START

### Prerequisites (One-Time Setup)
```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install FFmpeg 7 (CRITICAL - must be version 7, not 8)
brew install ffmpeg@7

# Install pkg-config
brew install pkg-config
```

### Step 1: Build Rust Library
```bash
cd /Users/robin-hassan/Desktop/f2v2f/lib

# Set environment variables for FFmpeg 7
export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"

# Build (takes ~5 seconds)
cargo build --release --lib

# Verify (should show ~13.5MB)
ls -lh target/release/libf2v2f.dylib
```

**What happens:** Rust code compiles → FFmpeg is linked → FFI interface is generated

### Step 2: Start Services (Two Terminals)

**Terminal 1 - Backend:**
```bash
cd /Users/robin-hassan/Desktop/f2v2f/backend
python3 app.py
# You should see: * Running on http://127.0.0.1:5000
```

**Terminal 2 - Frontend:**
```bash
cd /Users/robin-hassan/Desktop/f2v2f/frontend
NEXT_PUBLIC_API_URL=http://localhost:5000 npm run dev
# You should see: - Local: http://localhost:3000
```

### Step 3: Open in Browser
```bash
open http://localhost:3000
```

### Step 4: Test Encoding
1. Create a test file: `echo "test" > ~/test.txt`
2. Click "Encode" tab
3. Drag & drop `~/test.txt`
4. Click "Encode to Video"
5. Watch progress bar → Check "History" tab

---

## 🏗️ How to Build Rust Library (Step-by-Step)

### What You're Building
- `libf2v2f.dylib` - Dynamic library (13.5MB)
- C FFI interface for Python to use
- Encoding/decoding with geometric art generation
- FFmpeg H.264 video output

### Build Process

```bash
# 1. Go to lib directory
cd /Users/robin-hassan/Desktop/f2v2f/lib

# 2. Set environment variables (CRITICAL!)
export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"

# 3. Build in release mode (optimized)
cargo build --release --lib

# 4. Verify success
ls -lh target/release/libf2v2f.dylib
# Output: -rw-r--r--  13.5M ... libf2v2f.dylib
```

### What Gets Built
```
lib/target/release/
├── libf2v2f.dylib    ← Main library (13.5MB) - THIS IS WHAT WE NEED
├── libf2v2f.a        ← Static library
├── libf2v2f.rlib     ← Rust library format
└── deps/             ← Dependencies
```

### Troubleshooting the Build

| Error | Solution |
|-------|----------|
| `cargo: command not found` | Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `FFmpeg headers not found` | Set `CPPFLAGS`: `-I/usr/local/opt/ffmpeg@7/include` |
| `avfft.h not found` | You have FFmpeg 8 - downgrade: `brew install ffmpeg@7` |
| `ld: library not found for -lavformat` | Set `LDFLAGS`: `-L/usr/local/opt/ffmpeg@7/lib` |
| Takes 10+ minutes | Normal for first build, parallel compilation uses all cores |

### Files Modified During Build
- `src/error.rs` - Added `From<io::Error>` and `From<std::fmt::Error>` implementations
- `src/encoder.rs` - Fixed progress bar template error handling
- `src/decoder.rs` - Fixed progress bar template error handling  
- `src/ffi.rs` - Removed explicit null byte from C string literal

All these are already fixed in the repo.

---

## 🐍 Python Backend Configuration

**What Gets Auto-Configured:**

When you run `python3 app.py`, the backend automatically:

1. **Sets environment variables** - FFmpeg and Rust library are found
2. **Creates directories** - `uploads/`, `outputs/`, `uploads/files_registry.json`
3. **Initializes Flask** - REST API server starts on port 5000
4. **Enables CORS** - Allows frontend on port 3000 to connect

**Configuration Values:**
```python
UPLOAD_FOLDER = backend/uploads/        # User uploads
OUTPUT_FOLDER = backend/outputs/        # Encoded/decoded results
MAX_FILE_SIZE = 5GB                             # File upload limit
CORS enabled for port 3000                      # Frontend communication
```

**API Endpoints Ready:**
- `POST /api/encode` - Encode file to video
- `POST /api/decode` - Decode video to file
- `GET /api/status/<job_id>` - Check job progress
- `GET /api/files` - List all files
- `GET /api/download/<filename>` - Download file
- `GET /health` - Health check

---

## ⚛️ Next.js Frontend Configuration

**What Gets Auto-Configured:**

When you run `npm run dev`, the frontend automatically:

1. **Loads environment** - `NEXT_PUBLIC_API_URL=http://localhost:5000`
2. **Builds API client** - Endpoints connect to backend `/api/*`
3. **Compiles React** - TypeScript → JavaScript
4. **Serves on port 3000** - http://localhost:3000

**UI Features:**
- **Encode Tab** - Upload file, set resolution/FPS, encode to video
- **Decode Tab** - Upload MP4, decode back to original file
- **History Tab** - View all processed files, play videos

---

## 🧪 Complete Testing Workflow

### Test 1: Encoding
```bash
# 1. Create test file
echo "Hello, this is a test file for f2v2f encoding!" > ~/test.txt

# 2. Open http://localhost:3000
# 3. Click "Encode" tab
# 4. Drag & drop ~/test.txt
# 5. Click "Encode to Video"
# 6. Wait for progress bar to complete

# 7. Verify output file was created
ls -lh /Users/robin-hassan/Desktop/f2v2f/backend/outputs/
# Should see a .mp4 file

# 8. Check "History" tab in browser
# Should see the encoded video in the list
```

### Test 2: Decoding
```bash
# 1. In "History" tab, find the MP4 from above
# 2. Click "Decode" button (if available)
#    OR go to "Decode" tab and upload the MP4

# 3. Click "Decode from Video"
# 4. Wait for progress bar

# 5. Verify original file was restored
ls /Users/robin-hassan/Desktop/f2v2f/backend/outputs/

# 6. Compare with original
# File should be identical to ~/test.txt
```

### Test 3: Video Playback
```bash
# 1. Go to "History" tab
# 2. Find an encoded .mp4 file
# 3. Click on it to play in the browser
# Should show geometric art patterns animating
```

---

## 📊 System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│         User Opens http://localhost:3000                    │
│              (Web Browser)                                   │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ↓
┌─────────────────────────────────────────────────────────────┐
│         Next.js Frontend (React + TypeScript)               │
│  Port: 3000                                                 │
│                                                             │
│  - FileUploadForm.tsx → File drag & drop                   │
│  - JobStatus.tsx → Progress bars (0-100%)                 │
│  - FileHistory.tsx → Video player & file browser          │
│  - api.ts → HTTP calls to backend                         │
│                                                             │
│  NEXT_PUBLIC_API_URL = http://localhost:5000/api          │
└──────────────────────┬──────────────────────────────────────┘
                       │ HTTP POST/GET
                       │ /api/encode, /api/decode, etc.
                       ↓
┌─────────────────────────────────────────────────────────────┐
│         Flask Backend (Python REST API)                     │
│  Port: 5000                                                 │
│                                                             │
│  - app.py → Main Flask server                             │
│  - Handles file uploads → Stores in uploads/              │
│  - Creates background jobs for encode/decode              │
│  - Returns encoded files from outputs/                     │
│  - Manages file history/registry                          │
│                                                             │
│  Environment:                                              │
│  - DYLD_LIBRARY_PATH=/usr/local/opt/ffmpeg@7/lib:...     │
│  - Calls Rust via ctypes FFI                             │
└──────────────────────┬──────────────────────────────────────┘
                       │ ctypes FFI
                       │
┌──────────────────────┴──────────────────────────────────────┐
│                                                             │
│  ┌────────────────────────────────────────────────────┐    │
│  │  f2v2f.py (Python ctypes Wrapper)                │    │
│  │  - Encoder class (calls _lib.f2v2f_encode_file)  │    │
│  │  - Decoder class (calls _lib.f2v2f_decode_file)  │    │
│  └────────────────────────────────────────────────────┘    │
│                                                             │
│  ┌────────────────────────────────────────────────────┐    │
│  │  libf2v2f.dylib (Rust Library - 13.5MB)          │    │
│  │                                                    │    │
│  │  Encoding Process:                                │    │
│  │  1. Read file in chunks                          │    │
│  │  2. Generate geometric art for each chunk        │    │
│  │  3. Compose art frames into video                │    │
│  │  4. Write H.264 MP4 with FFmpeg                  │    │
│  │                                                    │    │
│  │  Decoding Process:                                │    │
│  │  1. Parse MP4 video frames                        │    │
│  │  2. Recognize geometric patterns                  │    │
│  │  3. Convert patterns back to data chunks         │    │
│  │  4. Reconstruct original file                     │    │
│  │  5. Verify SHA256 checksum                        │    │
│  │                                                    │    │
│  │  Requires FFmpeg 7.1.3:                          │    │
│  │  - libavformat, libavcodec, libswscale          │    │
│  │  - All available via /usr/local/opt/ffmpeg@7/lib│    │
│  └────────────────────────────────────────────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 📁 Directory Tree (Important Paths)

```
/Users/robin-hassan/Desktop/f2v2f/
├── lib/                              ← Rust Code
│   ├── Cargo.toml                   ← Build config
│   ├── src/
│   │   ├── lib.rs
│   │   ├── encoder.rs               ← Encoding logic
│   │   ├── decoder.rs               ← Decoding logic
│   │   ├── ffi.rs                   ← C Interface (***)
│   │   ├── image_generator.rs       ← Art generation
│   │   ├── video_composer.rs        ← Video creation
│   │   └── ...
│   └── target/release/
│       └── libf2v2f.dylib           ← BUILD OUTPUT (13.5MB) ***
│
├── backend/                          ← Python Flask
│   ├── app.py                       ← Flask server (main) ***
│   ├── f2v2f.py                     ← Python FFI wrapper
│   ├── requirements.txt             ← pip dependencies
│   ├── uploads/                     ← User uploads (auto-created)
│   ├── outputs/                     ← Encoded/decoded results (auto-created)
│   └── file_registry.json           ← File history (auto-created)
│
├── frontend/                         ← Next.js React
│   ├── app/
│   │   ├── page.tsx                 ← Main UI ***
│   │   └── layout.tsx               ← Page layout
│   ├── components/
│   │   ├── FileUploadForm.tsx       ← Upload UI
│   │   ├── JobStatus.tsx            ← Progress bars
│   │   └── FileHistory.tsx          ← File listing + video player
│   ├── lib/
│   │   └── api.ts                   ← API client ***
│   ├── styles/
│   │   └── globals.css              ← Dark theme styling
│   ├── package.json                 ← npm config
│   ├── next.config.js               ← Next.js config (JS, not TS) ***
│   └── node_modules/                ← npm packages (auto-created)
│
├── BUILD_AND_RUN.md                 ← Comprehensive build guide
├── BUILD_COMPLETE.md                ← Build status summary
├── FIXES_APPLIED.md                 ← All fixes documentation
├── SYSTEM_RUNNING.md                ← System status
├── setup.sh                         ← Setup script
├── start.sh                         ← Startup script
└── ... (other docs)
```

**Key files marked with (\*\*\*)** are the ones you'll interact with most.

---

## ⚙️ All Environment Variables Explained

### Set Automatically by Backend (app.py)
```
DYLD_LIBRARY_PATH includes:
  - /usr/local/opt/ffmpeg@7/lib  (FFmpeg libraries)
  - lib/target/release           (Rust library)
```

### Set by Frontend
```
NEXT_PUBLIC_API_URL = http://localhost:5000
(automatically appended with /api)
```

### You Need to Set (One-time, for Rust build)
```
export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"
```

---

## ✅ Verification Commands

```bash
# Check Rust library exists
ls -lh /Users/robin-hassan/Desktop/f2v2f/lib/target/release/libf2v2f.dylib

# Check FFmpeg is installed
brew list ffmpeg@7

# Check Backend is running
curl http://localhost:5000/health

# Check Frontend is running
curl http://localhost:3000 | grep f2v2f

# Check API connectivity
curl http://localhost:5000/api/version
```

---

## 📝 Logs & Debugging

**Real-time backend logs:**
```bash
tail -f /Users/robin-hassan/Desktop/f2v2f/backend/app.log
```

**Real-time frontend logs:**
```bash
tail -f /Users/robin-hassan/Desktop/f2v2f/frontend/frontend.log
```

**Check uploaded files:**
```bash
ls -lh /Users/robin-hassan/Desktop/f2v2f/backend/uploads/
```

**Check output files:**
```bash
ls -lh /Users/robin-hassan/Desktop/f2v2f/backend/outputs/
```

---

## 🎯 Next Steps After First Successful Test

1. ✅ **Successful Integration?** Great! Move to production testing
2. ✅ **File History Working?** Files are persisting correctly
3. ✅ **Video Playback Works?** UI integration is seamless
4. ✅ **Large Files?** Try 10MB+ files to test streaming
5. ✅ **Performance?** Check CPU usage during encoding

For larger production deployments:
- See [DEPLOYMENT.md](DEPLOYMENT.md) for Docker/cloud setup
- See [ARCHITECTURE.md](ARCHITECTURE.md) for advanced patterns

---

## 🔗 Related Documents

| Document | Purpose |
|----------|---------|
| [BUILD_AND_RUN.md](BUILD_AND_RUN.md) | Detailed build & configuration guide |
| [FIXES_APPLIED.md](FIXES_APPLIED.md) | All bugs fixed & how they were solved |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Technical design & how it works |
| [DEPLOYMENT.md](DEPLOYMENT.md) | Docker, Kubernetes, cloud deployment |
| [README.md](README.md) | Original project overview |

---

**Last Updated:** February 16, 2026  
**Status:** ✅ Ready for Production Testing  
**Support:** Check logs and error messages for debugging

Good luck! 🚀
