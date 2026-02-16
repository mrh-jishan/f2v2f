# F2V2F Golang Backend (High Performance) 🐹

High-performance REST API server built with Go and Fiber, utilizing the Rust `f2v2f` library via `cgo`. Designed for terabyte-scale file processing with high concurrency.

## 🚀 Features
- **Fastest Processing**: Uses Go's efficient concurrency model and low FFI overhead.
- **Terabyte Support**: Streaming architecture ensures constant memory usage even for huge files.
- **Fiber Web Framework**: Extremely low latency and high throughput.
- **SQLite Persistence**: Robust job history and file registry.
- **Multithreaded Zstd**: Hardware-accelerated compression via Rust core.

## 📦 What's Here
- `main.go`: Principal server logic and API routing.
- `f2v2f/`: Go bindings for the Rust shared library.
- `f2v2f.db`: Persistence layer.
- `uploads/`: Temporary storage for incoming files.
- `outputs/`: Final results (videos and restored files).

## 🔧 Installation

### Prerequisites
- Go 1.20+
- Rust library built (`../lib/target/release/libf2v2f.dylib`)
- FFmpeg 7.0+

### Build and Run
```bash
cd backend
go build -o gserver main.go
./gserver
```

The server starts on `http://localhost:5000` by default.

## 🔌 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | `/health` | Health check (returns engine info) |
| GET    | `/api/version` | Rust core version |
| POST   | `/api/encode` | Start encoding job |
| POST   | `/api/decode` | Start decoding job |
| GET    | `/api/status/:id` | Get job progress and results |
| GET    | `/api/files` | List history of operations |
| GET    | `/api/download/:fn` | Download results |

## 🛠️ Configuration
Configuration is currently hardcoded in `main.go` for simplicity but can be moved to `.env`:
- `BodyLimit`: 50GB (configurable)
- `Port`: 5000

## ⚖️ License
MIT
