# F2V2F - Build & Run Guide (Golang Edition)

This guide covers building the Rust core library and running the new Go-based web service.

## 🏗️ Building the Rust Library

The Rust library is the engine that powers f2v2f.

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install FFmpeg 7
brew install ffmpeg@7
brew install pkg-config
```

### Step 1: Build Rust Core
```bash
cd lib
# Optional: Set FFmpeg paths if brew is not in default path
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"

cargo build --release --lib
```
The output will be in `lib/target/release/libf2v2f.dylib` (macOS) or `.so` (Linux).

---

## 🐹 Golang Backend Setup

The Go backend is a high-performance Fiber server that replaces the previous Flask implementation.

### Step 1: Install Go
Ensure you have Go 1.20+ installed.
```bash
go version
```

### Step 2: Build the Backend
```bash
cd backend
go build -o server main.go
```

### Step 3: Start Go Server
```bash
./server
```
The server runs on **http://localhost:5001** by default.

---

## ⚛️ Next.js Frontend Setup

### Step 1: Install Dependencies
```bash
cd frontend
npm install
```

### Step 2: Start Development Server
```bash
# Point to the Go backend
NEXT_PUBLIC_API_URL=http://localhost:5001 npm run dev
```
The frontend runs on **http://localhost:3000**.

---

## 🚀 One-Command Startup

You can use the provided `Makefile` to start everything:

```bash
# Terminal 1: Build and Run Backend
make backend-run

# Terminal 2: Run Frontend
make frontend-dev
```

---

## ✅ System Verification

### 1. Health Check
```bash
curl http://localhost:5001/health
# Expected: {"status": "healthy", "engine": "golang"}
```

### 2. Version Check
```bash
curl http://localhost:5001/api/version
# Expected: {"version": "0.1.0"}
```

---

## 🧪 Testing Workflow

1. Open **http://localhost:3000**.
2. Upload a small file (e.g., a PDF or image).
3. The Go backend will spawn a worker to encode it into a video.
4. Once completed, the video will appear in the History tab.
5. Download and watch your file-transformed-to-art!
6. Go to the "Decode" tab and upload the video to get your original file back.

---

## 📁 Project Directory Structure

```
f2v2f/
├── lib/           # Rust Engine
├── backend/       # Go Web Service (Primary)
│   ├── f2v2f/     # Go-Rust Bindings
│   ├── main.go    # Fiber Server
│   └── outputs/   # Encoded videos
├── frontend/      # Next.js UI
└── Makefile   # Build automation
```

---

## 🔧 Troubleshooting

### "Library not found"
Go needs to know where the Rust shared library is.
- On macOS: `export DYLD_LIBRARY_PATH=$PWD/lib/target/release:$DYLD_LIBRARY_PATH`
- On Linux: `export LD_LIBRARY_PATH=$PWD/lib/target/release:$LD_LIBRARY_PATH`

### Port 5001 in use
Change it in `backend/main.go` or via environment variable:
```bash
PORT=5002 ./server
```

---

**Happy encoding with Go! 🐹🚀**
