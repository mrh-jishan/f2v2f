# F2V2F Complete Guide - Everything You Need to Know (Golang Edition)

## 📚 Quick Navigation

- **For Quick Start:** See [QUICK_START.md](QUICK_START.md)
- **For Building Rust:** See [BUILD_AND_RUN.md](BUILD_AND_RUN.md)
- **For Architecture:** See [ARCHITECTURE.md](ARCHITECTURE.md)

---

## 🚀 QUICK START

### Prerequisites (One-Time Setup)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Go
brew install go

# Install FFmpeg 7
brew install ffmpeg@7
brew install pkg-config
```

### Step 1: Build Core and Backend
```bash
cd f2v2f
make all           # Builds Rust library, Go backend, and installs Frontend
```

### Step 2: Start Services (Two Terminals)

**Terminal 1 - Backend (Go):**
```bash
make backend-run
# You should see: Starting Go server on http://localhost:5000
```

**Terminal 2 - Frontend (Next.js):**
```bash
make frontend-dev
# You should see: - Local: http://localhost:3000
```

### Step 3: Open in Browser
```bash
open http://localhost:3000
```

---

## 🏗️ How to Build Rust Library

The Rust library (`libf2v2f.dylib` or `.so`) is the core engine.

### Build Process
```bash
cd lib
# Optional: Set FFmpeg environment variables
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"

cargo build --release --lib
```

### What Gets Built
- `lib/target/release/libf2v2f.dylib`: The shared library that Go calls via cgo.

---

## 🐹 Golang Backend Configuration

The new Go backend is powered by **Fiber**.

**Auto-Configured Features:**
- **Concurrent Workers:** Uses Goroutines for parallel encoding/decoding.
- **SQLite Database:** Stores file history in `backend/f2v2f.db`.
- **Automatic Directories:** Creates `uploads/` and `outputs/` folders.
- **CORS:** Enabled for port 3000.

**Configuration Values:**
- Port: 5000
- Max Upload: 50GB
- Database: SQLite

**API Endpoints:**
- `POST /api/encode` - Start encoding job
- `POST /api/decode` - Start decoding job
- `GET /api/status/:job_id` - Check progress
- `GET /api/files` - List job history
- `GET /api/download/:filename` - Download result
- `GET /api/health` - Health check

---

## ⚛️ Next.js Frontend Configuration

**UI Features:**
- **Modern Interface:** Built with Tailwind CSS and Framer Motion.
- **Real-time Progress:** Polls the Go API for encode/decode status.
- **Live Video Player:** Preview encoded videos directly in your browser.
- **History Browser:** See all past operations and their details.

---

## 🧪 Complete Testing Workflow

1. **Encode:** Upload any file (PDF, TXT, etc.) and convert it to a video.
2. **Verify:** Check the `backend/outputs/` folder for the generated `.mp4`.
3. **Decode:** Upload the video back to the "Decode" tab.
4. **Compare:** The restored file should be byte-for-byte identical to the original.

---

## 📁 Project Directory Structure

```
f2v2f/
├── lib/           # Rust Core Engine
├── backend/       # Go Web Service (Primary)
│   ├── main.go    # Entry point
│   ├── f2v2f/     # Go-Rust bridge
│   └── f2v2f.db   # File Registry
├── frontend/      # Next.js Web App
└── Makefile   # Build automation
```

---

## ⚙️ Environment Variables

**Backend (Go):**
```bash
PORT=5000
ENCODING_WIDTH=1920
ENCODING_HEIGHT=1080
ENCODING_FPS=30
```

**Frontend:**
```bash
NEXT_PUBLIC_API_URL=http://localhost:5000
```

---

**Happy encoding with f2v2f and Go! 🐹🚀**
