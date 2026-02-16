# F2V2F - Complete Build & Run Guide

## 🏗️ Building the Rust Library

### Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install FFmpeg 7 (critical for compatibility)
brew install ffmpeg@7

# Install pkg-config
brew install pkg-config
```

### Step 1: Build Rust Core Library

```bash
# Navigate to lib directory
cd /Users/robin-hassan/Desktop/f2v2f/lib

# Export FFmpeg environment variables (IMPORTANT!)
export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"

# Build in release mode (optimized)
cargo build --release --lib

# Output will be in: lib/target/release/libf2v2f.dylib (13.5MB)
```

**Build Output:**
```
   Compiling f2v2f v0.1.0
   ...
    Finished `release` profile [optimized] in 4.67s
    
✓ lib/target/release/libf2v2f.a
✓ lib/target/release/libf2v2f.dylib  ← Main library (13.5MB)
✓ lib/target/release/libf2v2f.rlib
```

### What Happened During Build

The build process:
1. **Compiled Rust code** - All `.rs` files in `lib/src/`
2. **Linked FFmpeg** - Using FFmpeg 7.1.3 headers and libraries
3. **Generated FFI bindings** - C interface for Python to call
4. **Optimized release** - ~13.5MB optimized dylib for macOS
5. **15 minor warnings** - Non-critical (unused variables)

### Key Fixes Applied to Build

| Issue | Fix | File |
|-------|-----|------|
| FFmpeg 8.0 incompatible headers | Downgraded to FFmpeg 7.1.3 | Rust FFmpeg binding |
| `io::Error` not convertible to `F2V2FError` | Added `From<io::Error>` impl | `src/error.rs` |
| Progress bar template error handling | Used `.unwrap_or_else()` | `src/encoder.rs`, `src/decoder.rs` |
| C string literal with explicit `\0` | Removed extra null byte | `src/ffi.rs` |

---

## 🐍 Python Backend Setup

### Step 1: Create Virtual Environment
```bash
cd /Users/robin-hassan/Desktop/f2v2f
python3 -m venv venv
source venv/bin/activate
```

### Step 2: Install Dependencies
```bash
cd backend
pip install -r requirements.txt
```

**Dependencies:**
- Flask - REST API framework
- Flask-CORS - Cross-origin requests
- Werkzeug - Request utilities
- python-dotenv - Environment config

### Step 3: Fix Library Path (DONE ✓)
The `backend/f2v2f.py` wrapper now searches for the library in:
1. `lib/target/release/libf2v2f.dylib` ← New structure
2. `lib/target/debug/libf2v2f.dylib` ← New structure (fallback)
3. `target/release/libf2v2f.dylib` ← Old structure (fallback)
4. System paths `/usr/local/lib`, `/usr/lib`

### Step 4: Start Flask Backend
```bash
cd /Users/robin-hassan/Desktop/f2v2f/backend
python3 app.py
```

**Output:**
```
 * Running on http://127.0.0.1:5000
 * Debug mode: on
```

**Configuration (Fixed ✓):**
- `OUTPUT_FOLDER` - Now properly configured in Flask config
- `UPLOAD_FOLDER` - `backend/uploads/`
- Both directories auto-created on startup
- CORS enabled for port 3000 (Next.js frontend)

---

## ⚛️ Next.js Frontend Setup

### Step 1: Install Dependencies
```bash
cd /Users/robin-hassan/Desktop/f2v2f/frontend
npm install
```

**Dependencies (425 packages):**
- React 18
- Next.js 14
- TypeScript
- Tailwind CSS
- axios (for API calls)

### Step 2: Fixes Applied (✓ DONE)

| Issue | Fix | Impact |
|-------|-----|--------|
| `next.config.ts` not supported | Converted to `next.config.js` | Config now loads properly |
| API calls missing `/api` prefix | Fixed `lib/api.ts` to append `/api` | Endpoints now resolve correctly |
| `NEXT_PUBLIC_API_URL` not set | Auto-defaults to `http://localhost:5000` | Frontend connects to backend |

### Step 3: Start Next.js Frontend
```bash
cd /Users/robin-hassan/Desktop/f2v2f/frontend
NEXT_PUBLIC_API_URL=http://localhost:5000 npm run dev
```

**Output:**
```
  ▲ Next.js 14.0.0
  - Local:        http://localhost:3000
  - Environments: .env.local
```

---

## 🚀 Running the Complete System

### Terminal 1: Start Backend
```bash
cd /Users/robin-hassan/Desktop/f2v2f/backend
python3 app.py
```

### Terminal 2: Start Frontend
```bash
cd /Users/robin-hassan/Desktop/f2v2f/frontend
NEXT_PUBLIC_API_URL=http://localhost:5000 npm run dev
```

### Terminal 3: Test the System
```bash
# Create a test file
echo "Hello f2v2f! Test encoding/decoding." > ~/test.txt

# Open browser
open http://localhost:3000
```

---

## ✅ System Verification

### Health Checks

**Backend Health:**
```bash
curl http://localhost:5000/health
# Response: {"status": "healthy"}
```

**Frontend Loading:**
```bash
curl http://localhost:3000 | grep -i "f2v2f"
# Should return HTML with "f2v2f" title
```

**API Connectivity:**
```bash
# Check if frontend can reach backend
curl http://localhost:5000/api/version
# Response: {"version": "0.1.0"}
```

---

## 🧪 Testing Workflow

### 1. Encoding Test
```
Frontend:
- Upload file (test.txt)
- Set resolution: 1920x1080
- Set FPS: 30
- Click "Encode to Video"

Backend:
- Receives upload
- Calls Rust encoder
- Generates MP4 video

Expected:
- Progress bar shows 0-100%
- MP4 file appears in outputs/
- History tab shows result
```

### 2. Decoding Test
```
Frontend:
- Go to "Decode" tab
- Upload the MP4 from encoding
- Click "Decode from Video"

Backend:
- Receives MP4
- Calls Rust decoder
- Restores original file

Expected:
- Original file restored
- SHA256 checksum matches
- File integrity verified
```

### 3. File History
```
Frontend:
- Go to "History" tab
- See all processed files
- Can play encoded videos
- Can download files

Expected:
- All files visible
- Timestamps correct
- File sizes accurate
```

---

## 📁 Directory Structure

```
/Users/robin-hassan/Desktop/f2v2f/
├── lib/                          # ✓ Rust core
│   ├── Cargo.toml               # Rust manifest
│   ├── src/                      # Source code
│   │   ├── lib.rs              # Library root
│   │   ├── encoder.rs          # Encoding logic
│   │   ├── decoder.rs          # Decoding logic
│   │   ├── ffi.rs              # C interface (FFI)
│   │   ├── error.rs            # Error types
│   │   ├── image_generator.rs  # Art generation
│   │   ├── video_composer.rs   # Video creation
│   │   ├── config.rs           # Configuration
│   │   └── checkpoint_manager.rs # Progress tracking
│   └── target/release/
│       ├── libf2v2f.dylib      # ✓ COMPILED LIBRARY
│       ├── libf2v2f.a
│       └── libf2v2f.rlib
│
├── backend/                      # ✓ Python Flask
│   ├── app.py                  # Flask server
│   ├── f2v2f.py                # Python FFI wrapper (updated paths)
│   ├── requirements.txt         # pip dependencies
│   ├── setup.py                # Package config
│   ├── uploads/                # ✓ User uploads
│   ├── outputs/                # ✓ Encoded/decoded files
│   └── file_registry.json      # File history
│
├── frontend/                     # ✓ Next.js UI
│   ├── app/
│   │   ├── page.tsx            # Main page
│   │   └── layout.tsx          # Layout wrapper
│   ├── components/              # React components
│   │   ├── FileUploadForm.tsx
│   │   ├── JobStatus.tsx
│   │   └── FileHistory.tsx
│   ├── lib/
│   │   └── api.ts              # API client (fixed with /api)
│   ├── styles/
│   │   └── globals.css         # Dark theme CSS
│   ├── package.json            # npm dependencies
│   ├── tsconfig.json           # TypeScript config
│   ├── tailwind.config.ts      # Tailwind config
│   ├── next.config.js          # ✓ Next.js config (was .ts)
│   └── postcss.config.js       # PostCSS config
│
├── Makefile.new                # Build automation (optional)
├── docker-compose.new.yml      # Docker setup (optional)
├── Dockerfile.compose-backend   # Flask Docker image
├── Dockerfile.compose-frontend  # Next.js Docker image
├── .env.example                # Environment template
└── README files                # Documentation
```

---

## ⚙️ Configuration Reference

### Environment Variables

**Backend (Flask):**
```bash
# Automatic - created on app startup
UPLOAD_FOLDER = backend/uploads/
OUTPUT_FOLDER = backend/outputs/
FILE_REGISTRY = backend/file_registry.json
MAX_FILE_SIZE = 5GB
```

**Frontend (Next.js):**
```bash
NEXT_PUBLIC_API_URL=http://localhost:5000
# If not set, defaults to http://localhost:5000
```

**Rust Build:**
```bash
FFMPEG_DIR=/usr/local/opt/ffmpeg@7
PKG_CONFIG_PATH=/usr/local/opt/ffmpeg@7/lib/pkgconfig
```

---

## 🔧 Troubleshooting

### Error: Could not find f2v2f library
**Solution:**
```bash
# Ensure lib is built
cd /Users/robin-hassan/Desktop/f2v2f/lib
cargo build --release --lib

# Verify dylib exists
ls -lh lib/target/release/libf2v2f.dylib
```

### Error: OUTPUT_FOLDER not found
**Status:** ✓ FIXED in app.py
- Flask config now includes `OUTPUT_FOLDER`
- Directories auto-created on encode/decode

### Error: API endpoints not found (404)
**Solution:**
```bash
# Verify api.ts has correct prefix
grep "API_BASE" /Users/robin-hassan/Desktop/f2v2f/frontend/lib/api.ts
# Should append '/api' to base URL
```

### Error: next.config.ts not supported
**Status:** ✓ FIXED
- Converted to `next.config.js`
- Next.js now loads config properly

### Encoding fails with no output file
**Solution:**
```bash
# Verify output directory exists and is writable
ls -ld /Users/robin-hassan/Desktop/f2v2f/backend/outputs/
chmod 755 /Users/robin-hassan/Desktop/f2v2f/backend/outputs/
```

---

## 📊 System Architecture

```
┌────────────────────────────────────────────────────────┐
│          Browser (http://localhost:3000)              │
│  ┌──────────────────────────────────────────────────┐ │
│  │  Next.js Frontend (TypeScript/React)             │ │
│  │  - File Upload Form                              │ │
│  │  - Job Status Display                            │ │
│  │  - File History & Video Player                   │ │
│  └──────────────────────────────────────────────────┘ │
└────────────────┬─────────────────────────────────────┘
                 │ HTTP REST API
                 ├─ /api/encode (POST)
                 ├─ /api/decode (POST)
                 ├─ /api/status/<id> (GET)
                 ├─ /api/files (GET)
                 └─ /api/download/<file> (GET)
                 │
┌────────────────▼─────────────────────────────────────┐
│    Flask Backend (http://localhost:5000)              │
│  ┌──────────────────────────────────────────────────┐ │
│  │  Python Application Layer                        │ │
│  │  - Job Queue Management                          │ │
│  │  - File Upload/Download Handling                 │ │
│  │  - Progress Tracking                             │ │
│  └──────┬─────────────────────────────────────────┘ │
│         │ ctypes FFI Bindings
│  ┌──────▼─────────────────────────────────────────┐ │
│  │  f2v2f.py (ctypes wrapper)                     │ │
│  │  - Encoder class                               │ │
│  │  - Decoder class                               │ │
│  └──────┬─────────────────────────────────────────┘ │
└────────┼────────────────────────────────────────────┘
         │ C FFI Calls
         │
┌────────▼────────────────────────────────────────────┐
│  Rust Core Library (libf2v2f.dylib)                 │
│  ┌──────────────────────────────────────────────┐  │
│  │  Encoding Engine                             │  │
│  │  - File chunking                             │  │
│  │  - Geometric art generation                  │  │
│  │  - Video composition                         │  │
│  └──────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────┐  │
│  │  Decoding Engine                             │  │
│  │  - Video parsing                             │  │
│  │  - Pattern recognition                       │  │
│  │  - File reconstruction                       │  │
│  └──────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────┐  │
│  │  FFmpeg Integration (v7.1.3)                 │  │
│  │  - MP4 encoding                              │  │
│  │  - H.264 video codec                         │  │
│  │  - Metadata handling                         │  │
│  └──────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

---

## 📝 Build & Run Summary

### Quick Start Script
```bash
#!/bin/bash
# f2v2f - Complete startup

cd /Users/robin-hassan/Desktop/f2v2f

# Terminal 1: Backend
echo "Starting Backend..."
cd backend && python3 app.py &

# Terminal 2: Frontend
echo "Starting Frontend..."
cd ../frontend
NEXT_PUBLIC_API_URL=http://localhost:5000 npm run dev &

# Wait for startup
sleep 3

# Terminal 3: Open browser
open http://localhost:3000

echo "✓ System Running!"
echo "  Frontend: http://localhost:3000"
echo "  Backend:  http://localhost:5000"
```

### Save as startup script:
```bash
chmod +x /Users/robin-hassan/Desktop/f2v2f/start.sh
./start.sh
```

---

**✓ System Ready for Testing!**

All components are properly configured and ready to use. Follow the testing workflow above to verify everything works correctly.
