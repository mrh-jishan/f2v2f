# f2v2f - Multi-Language Architecture & Integration Guide

## Overview

This is a **multi-language library project** with a Rust core and FFI bindings for Python and TypeScript/Node.js. It's structured as a monorepo that supports:

1. **Rust Core** - High-performance encoding/decoding engine
2. **Python Bindings** - ctypes-based FFI wrapper + Flask backend
3. **TypeScript/Node.js Bindings** - FFI wrapper + Express backend
4. **CLI Tool** - Standalone command-line interface
5. **Web Frontends** - React/Vue.js applications

## Project Structure

```
f2v2f/
├── src/                    # Rust source code
│   ├── main.rs            # CLI binary
│   ├── lib.rs             # Library root
│   ├── ffi.rs             # C FFI exports (NEW)
│   ├── encoder.rs
│   ├── decoder.rs
│   ├── image_generator.rs
│   ├── video_composer.rs
│   ├── config.rs
│   └── error.rs
├── bindings/              # Language bindings (NEW)
│   ├── python/
│   │   ├── f2v2f.py       # ctypes wrapper (NEW)
│   │   ├── app.py         # Flask backend (NEW)
│   │   ├── setup.py       # Python package setup (NEW)
│   │   └── requirements.txt
│   ├── typescript/
│   │   ├── index.ts       # FFI wrapper (NEW)
│   │   ├── app.ts         # Express backend (NEW)
│   │   ├── package.json   # Node.js config (NEW)
│   │   └── dist/          # Compiled JS (generated)
│   └── lib/               # Compiled shared libraries
│       ├── libf2v2f.so
│       ├── libf2v2f.dylib
│       └── f2v2f.dll
├── examples/
│   ├── encode_file.rs
│   └── decode_video.rs
├── build.sh               # Cross-platform build script (NEW)
├── build.bat              # Windows build script (NEW)
├── Cargo.toml             # Rust configuration
├── Makefile               # Build targets (optional)
└── README.md
```

## Building the Project

### Prerequisites

- Rust 1.70+ ([rustup.rs](https://rustup.rs/))
- Python 3.8+ (for Python bindings)
- Node.js 16+ (for TypeScript bindings)
- Cargo (included with Rust)

### Quick Build (All Platforms)

**macOS/Linux:**
```bash
chmod +x build.sh
./build.sh
```

**Windows:**
```cmd
build.bat
```

This automatically:
1. ✅ Builds Rust library (release mode)
2. ✅ Copies library to bindings folder
3. ✅ Sets up Python virtual environment
4. ✅ Installs Python dependencies
5. ✅ Builds TypeScript with npm
6. ✅ Creates shared library accessible to all languages

### Manual Build Steps

If you prefer step-by-step:

```bash
# Build Rust library
cargo build --release --lib

# Copy library to accessible location
mkdir -p bindings/lib
cp target/release/libf2v2f.* bindings/lib/

# Python setup
cd bindings/python
python3 -m venv venv
source venv/bin/activate  # or: . venv/Scripts/activate on Windows
pip install -r requirements.txt
pip install -e .

# TypeScript setup
cd ../typescript
npm install
npm run build
```

## Architecture: How It All Works

### 1. **Rust Core (src/)**

The Rust core provides:
- High-performance encoding/decoding
- Geometric art generation
- Video composition
- Async/await for long operations
- Error handling with type safety

**Key responsibility:** Do the heavy lifting efficiently

### 2. **FFI Layer (src/ffi.rs)**

C-compatible function exports:
- `f2v2f_encode_create()` - Create encoder with config
- `f2v2f_encode_file()` - Perform encoding
- `f2v2f_encode_free()` - Cleanup
- `f2v2f_decode_create()` - Create decoder
- `f2v2f_decode_file()` - Perform decoding
- `f2v2f_decode_free()` - Cleanup

**Key responsibility:** Bridge between Rust and other languages

```
┌─────────────────────────────┐
│        Rust Code            │
│   (Type-safe, fast)         │
└────────────┬────────────────┘
             │
             ↓
┌─────────────────────────────┐
│     FFI Layer (C API)       │
│   (Language-agnostic)       │
└─────────────────────────────┘
      │        │        │
      ↓        ↓        ↓
   Python   Node.js   Other
   ctypes    ffi-napi  Languages
```

### 3. **Python Wrapper (bindings/python/)**

**f2v2f.py:**
- Uses ctypes to call C FFI functions
- Provides Pythonic API with Encoder/Decoder classes
- Exception handling converted to Python exceptions
- String encoding/decoding handled automatically

**Example usage:**
```python
from f2v2f import Encoder

encoder = Encoder(width=1920, height=1080, fps=30)
encoder.encode("input.pdf", "output.mp4")
```

**app.py:**
- Flask REST API server
- Handles file uploads with multer
- Manages encoding/decoding jobs with background threads
- Returns progress/status/download links
- Cleanup of old files

### 4. **TypeScript Wrapper (bindings/typescript/)**

**index.ts:**
- Uses ffi-napi to call C FFI functions
- Provides TypeScript types and classes
- Async/await support
- Error codes mapped to exceptions

**Example usage:**
```typescript
import { Encoder } from './index';

const encoder = new Encoder(1920, 1080, 30);
await encoder.encode("input.pdf", "output.mp4");
```

**app.ts:**
- Express REST API server
- Multer for file uploads
- UUID-based job tracking
- Streaming downloads
- Background job processing

### 5. **Data Flow: Encoding**

```
User UI (React/Vue)
    ↓
    │ POST /api/encode
    │ (with file)
    ↓
Flask/Express Backend
    ↓
    │ Save file
    │ Create job ID
    │ Return job ID
    ↓
Background Thread
    ↓
    │ Python/TypeScript wrapper
    │   ↓
    │ Call FFI function
    │   ↓
    │ Rust core
    │   - Read file chunks
    │   - Generate art
    │   - Compose video
    │   - Calculate checksum
    ↓
Write to disk
    ↓
User queries /api/status/{jobId}
    ↓
Get result download link
```

### 6. **Error Handling Strategy**

**Rust → C FFI:**
- Returns error codes (0 = success, others = error)
- No panic (all errors mapped to codes)

**C API → Python:**
```python
_check_error(error_code)  # Raises Python exception
```

**C API → TypeScript:**
```typescript
checkError(code);  // Throws F2V2FError
```

**In Web Backends:**
- Try/catch around FFI calls
- Return error JSON with status code
- Long operations wrapped in Promise/thread with timeout

## Using Each Binding

### Python Binding

**Installation:**
```bash
cd bindings/python
pip install -r requirements.txt
pip install -e .
```

**Basic usage:**
```python
from f2v2f import Encoder, Decoder

# Encode
encoder = Encoder(width=3840, height=2160, fps=60)
try:
    encoder.encode("huge-file.iso", "output.mp4")
except Exception as e:
    print(f"Encoding failed: {e}")

# Decode
decoder = Decoder()
try:
    decoder.decode("output.mp4", "recovered.iso")
except Exception as e:
    print(f"Decoding failed: {e}")
```

**Using Flask backend:**
```bash
cd bindings/python
source venv/bin/activate
flask run --host 0.0.0.0 --port 5000
```

Visit `http://localhost:5000/api/health` to verify.

### TypeScript/Node.js Binding

**Installation:**
```bash
cd bindings/typescript
npm install
npm run build
```

**Basic usage:**
```typescript
import { Encoder, Decoder } from './index';

// Encode
const encoder = new Encoder(1920, 1080, 30);
try {
    await encoder.encode("input.pdf", "output.mp4");
} catch (error) {
    console.error("Encoding failed:", error);
}

// Decode
const decoder = new Decoder();
try {
    await decoder.decode("output.mp4", "recovered.pdf");
} catch (error) {
    console.error("Decoding failed:", error);
}
```

**Using Express backend:**
```bash
cd bindings/typescript
npm run dev  # Development with ts-node
npm run build && npm start  # Production
```

Visit `http://localhost:3000/api/health` to verify.

## CLI Tool

The Rust CLI is built separately:

```bash
# Build
cargo build --release --bin f2v2f

# Use
./target/release/f2v2f encode input.pdf output.mp4
./target/release/f2v2f decode output.mp4 recovered.pdf
```

## Performance Considerations

### When to Use What

| Use Case | Recommendation |
|----------|---|
| Simple encoding | CLI tool (`f2v2f encode`) |
| Python integrations | Python bindings + Flask |  
| Node.js integrations | TypeScript bindings + Express |
| Maximum performance | Use Rust directly with --release |
| Web application | Choose Flask or Express |

### Memory Management

- **Rust:** Streaming (constant memory regardless of file size)
- **FFI calls:** Copy-on-call (small overhead for path strings)
- **Backend threads:** One per job, queued if needed

### Timeout Management

**For long operations:**
```python
# Python with timeout
import signal

def timeout_handler(signum, frame):
    raise TimeoutError("Operation exceeded timeout")

signal.signal(signal.SIGALRM, timeout_handler)
signal.alarm(3600)  # 1 hour timeout
encoder.encode("file", "output.mp4")
signal.alarm(0)  # Disable alarm
```

```typescript
// TypeScript with timeout
async function encodeWithTimeout(encoder: Encoder, input: string, output: string) {
    return Promise.race([
        encoder.encode(input, output),
        new Promise((_, reject) => 
            setTimeout(() => reject(new Error("Timeout")), 3600000)
        )
    ]);
}
```

## Deployment

### Docker Setup

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --lib

FROM python:3.11
COPY --from=builder /app/target/release/libf2v2f.so /usr/lib/
COPY bindings/python /app/python
COPY bindings/requirements.txt /app/
RUN pip install -r /app/requirements.txt
WORKDIR /app/python
CMD ["python3", "app.py"]
```

### Kubernetes Example

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: f2v2f-api
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: api
        image: f2v2f:latest
        ports:
        - containerPort: 5000
        resources:
          requests:
            memory: "2Gi"
            cpu: "1000m"
          limits:
            memory: "4Gi"
            cpu: "2000m"
        volumeMounts:
        - name: data
          mountPath: /data
      volumes:
      - name: data
        emptyDir: {}
```

## Testing

### Rust Tests
```bash
cargo test --release
```

### Python Tests
```bash
cd bindings/python
source venv/bin/activate
python -m pytest tests/
```

### TypeScript Tests
```bash
cd bindings/typescript
npm test
```

## Troubleshooting

### Library Not Found Error

**Problem:** "Could not find f2v2f library"

**Solution:**
```bash
# Rebuild library
cargo build --release --lib

# Verify location
ls -la bindings/lib/

# Check environment variable
export LD_LIBRARY_PATH=/path/to/f2v2f/bindings/lib:$LD_LIBRARY_PATH
```

### Python FFI Error

**Problem:** "undefined symbol" or "bad magic number"

**Solution:**
```bash
# Rebuild fresh
cargo clean
cargo build --release --lib --verbose

# Verify compilation
file target/release/libf2v2f.so
```

### TypeScript Build Error

**Problem:** "Cannot find module 'ffi-napi'"

**Solution:**
```bash
cd bindings/typescript
npm install --force
npm run build --verbose
```

## Next Steps

1. **Frontend Development:** Create React/Vue.js UI in `frontend/` directory
2. **Database:** Add PostgreSQL for job history (replace in-memory map)
3. **Message Queue:** Use Celery/Bull for distributed job processing
4. **Monitoring:** Add Prometheus metrics and health checks
5. **Authentication:** Implement JWT/OAuth2 for API security
6. **Rate Limiting:** Protect endpoints with rate limiting

## References

- [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [Python ctypes](https://docs.python.org/3/library/ctypes.html)
- [Node.js FFI](https://github.com/node-ffi/node-ffi)
- [Flask Documentation](https://flask.palletsprojects.com/)
- [Express Documentation](https://expressjs.com/)

---

**Happy cross-language development! 🚀**
