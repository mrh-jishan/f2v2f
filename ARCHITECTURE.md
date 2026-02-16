# f2v2f - High-Performance Multi-Language Architecture

## Overview

**f2v2f** is a high-performance file-to-video encoding system. It is structured as a monorepo with a high-performance Rust core and a language binding for Go (Primary).

1. **Rust Core** - High-performance encoding/decoding engine (FFI based)
2. **Golang Backend** - High-concurrency web service using Fiber (Primary)
3. **Next.js Frontend** - Modern web interface for the service

## Project Structure

```
f2v2f/
├── lib/                      # Rust source code
│   ├── src/
│   │   ├── ffi.rs            # C FFI exports
│   │   ├── encoder.rs        # Encoding engine
│   │   ├── decoder.rs        # Decoding engine
│   │   └── ...
├── backend/                  # Golang Backend (Primary)
│   ├── main.go               # Fiber web server
│   ├── f2v2f/                # Go bindings (cgo)
│   └── uploads/              # Temporal storage
├── frontend/                 # Next.js Frontend
│   ├── app/                  # App Router components
│   ├── components/           # UI components
│   └── lib/api.ts            # API Client
├── Makefile.new              # Main build system
└── docker-compose.new.yml    # Container orchestration
```

## Architecture: How It All Works

### 1. **Rust Core (lib/)**

The Rust core is the heart of the system, providing:
- Deterministic geometric art generation
- Pixel-perfect data mapping
- Multi-threaded H.264 video encoding via FFmpeg
- Streaming processing for terabyte-scale files

### 2. **Golang Backend (backend/)**

The Go backend is the primary web service. It uses the **Fiber** framework for high performance and **cgo** to call the Rust core.

**Capabilities:**
- Asynchronous job management (Goroutines)
- SQLite for persistent file history
- High-speed file streaming
- Concurrent processing of multiple encodes/decodes

**Processing Flow:**
1. Client uploads file to `/api/encode`.
2. Go server saves file and spawns a Goroutine.
3. Goroutine calls the Rust `f2v2f_encode_file` via CGO.
4. Rust core processes the file, generating a video.
5. Go server updates SQLite with job completion and metadata.

### 3. **FFI Layer & Bindings**

- **Go (Primary):** Uses CGO to link directly to `libf2v2f.so` / `libf2v2f.dylib`.

```
┌─────────────────────────────┐
│        Rust Core            │
│  (Engine, Art, Encoding)    │
└────────────┬────────────────┘
             │
             ↓
┌─────────────────────────────┐
│     FFI Layer (C API)       │
└─────────────────────────────┘
      │
      ↓
   Golang
    (cgo)
```

## Data Flow: Encoding

1. **Frontend:** User selects a 10GB ISO file and clicks "Encode".
2. **API:** `POST /api/encode` sends the file chunk to the Go Backend.
3. **Backend:** Saves the file to `uploads/` and returns `202 Accepted` with a `job_id`.
4. **Processing:** A Go worker (Goroutine) starts the Rust encoder.
5. **Rust:** Reads the 10GB file in 64KB chunks, generates beautiful geometric frames, and pipes them to FFmpeg.
6. **Completion:** The output `.mp4` is saved to `outputs/`, and the job status is marked "completed" in SQLite.
7. **Frontend:** Polls status until "completed", then displays the video and download link.

## Deployment Architecture

### Docker Compose
- **Backend Service:** Compiled Go binary running with FFmpeg and the Rust shared library.
- **Frontend Service:** Next.js application served via Node.js (or exported to static).
- **Network:** Private bridge network for inter-service communication.

### Scaling
- The Go backend is designed to be stateless (using a mounted volume for files/history).
- Multiple backend instances can be run behind a load balancer.

## Performance Considerations

- **Memory:** Both Go and Rust use streaming, keeping memory usage constant (~100MB-200MB) even for large files.
- **CPU:** Encoding is CPU-intensive. Go's concurrency model allows it to manage multiple encoding jobs while remaining responsive.
- **IO:** Go's efficient IO handling minimizes bottlenecks during file uploads and downloads.

## Security

- **File Validation:** All uploads are validated for size and type.
- **Path Sanitization:** Prevent directory traversal.
- **CGO Safety:** Proper pointer management between Go and Rust.

---

**Happy developing with f2v2f and Go! 🐹🚀**
