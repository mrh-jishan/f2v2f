# File to Video to File (f2v2f) 🎬

## Overview

**f2v2f** is an innovative, high-performance file encoding system that converts any file into a beautiful video with procedurally-generated geometric art, and decodes it back perfectly. 

The system is powered by a high-performance **Rust core** and a high-concurrency **Golang web service**.

**Key Features:**
- ✨ **Beautiful geometric art generation**: Data is mapped to visual patterns.
- 🚀 **High-performance Rust core**: FFI based for maximum speed.
- 🐹 **Golang Backend**: High-concurrency primary web service using Fiber.
- 🎯 **Perfect reconstruction**: Byte-for-byte identical data recovery.
- 💾 **Terabyte-scale support**: Zero-copy streaming and multithreaded zstd compression.
- 🌐 **Modern Web UI**: Beautiful Next.js frontend with live progress tracking.
- 🐳 **Docker Ready**: Fully containerized for easy deployment.

## Architecture

```
Input File
    ↓
[Go Backend] → Spawns job & manages state
    ↓
[Rust Core] → Reads file in chunks & generates art
    ↓
[FFmpeg] → Assembles frames into H.264 video
    ↓
Output Video
    ↓
[Decoder] → Extracts data from video frames back to file
```

## Quick Start

### 1. Build Everything
```bash
make all
```

### 2. Run the System
```bash
# Terminal 1
make backend-run

# Terminal 2
make frontend-dev
```
Visit http://localhost:3000 to start encoding!

## API Usage (Go/Fiber)

The Go backend exposes a REST API at `http://localhost:5001/api/`.

```bash
# Encode a file
curl -X POST -F "file=@data.zip" http://localhost:5001/api/encode

# Check status
curl http://localhost:5001/api/status/{job_id}
```

## Project Structure

- `lib/`: Rust Core Engine
- `backend/`: Golang Web Service (Primary)
- `frontend/`: Next.js Web Application
- `Makefile.new`: Main build system

## Why Golang?
We shifted our primary web service to Go to leverage its superior concurrency model (goroutines) and excellent performance as a web server, while keeping the heavy computational logic in Rust.

## Documentation
- [QUICK_START.md](QUICK_START.md) - Get started in 3 minutes
- [ARCHITECTURE.md](ARCHITECTURE.md) - Deep dive into design
- [BUILD_AND_RUN.md](BUILD_AND_RUN.md) - Build instructions
- [DEPLOYMENT.md](DEPLOYMENT.md) - Production deployment

---

**Happy encoding! 🎬🚀**
