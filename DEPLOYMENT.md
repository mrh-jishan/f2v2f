# f2v2f - Complete Setup & Deployment Guide

## ✨ Project Summary

f2v2f is a **multi-language, multi-platform file encoding system** that converts any file into an artistic video and decodes it back. This comprehensive guide covers:

- ✅ Complete Rust core library with FFI exports
- ✅ Python bindings (ctypes) with Flask backend
- ✅ TypeScript/Node.js bindings with Express backend  
- ✅ Beautiful web UI (HTML/CSS/JS)
- ✅ Cross-platform build system
- ✅ Docker deployment ready
- ✅ Full error handling for long operations

## Quick Start (5 minutes)

### 1. Clone and Build

```bash
cd /Users/robin-hassan/Desktop/f2v2f

# macOS/Linux
chmod +x build.sh
./build.sh

# Windows
build.bat
```

This automatically:
- Builds Rust library in release mode
- Sets up Python environment
- Installs Node.js dependencies
- Compiles everything

### 2. Try the CLI

```bash
# Create test file
echo "Hello World" > test.txt

# Encode
./target/release/f2v2f encode test.txt output.mp4

# Decode
./target/release/f2v2f decode output.mp4 recovered.txt

# Verify
diff test.txt recovered.txt && echo "✅ Files match!"
```

### 3. Use Python Binding

```bash
cd bindings/python
source venv/bin/activate

python3 << 'EOF'
from f2v2f import Encoder, Decoder

encoder = Encoder()
encoder.encode("test.txt", "output.mp4")

decoder = Decoder()
decoder.decode("output.mp4", "recovered.txt")
EOF
```

### 4. Use TypeScript Binding

```bash
cd bindings/typescript
npm install && npm run build

node << 'EOF'
const { Encoder, Decoder } = require('./index');
const encoder = new Encoder();
encoder.encode("test.txt", "output.mp4")
  .then(() => console.log("Done!"))
  .catch(err => console.error(err));
EOF
```

### 5. Run Web Application

**Python (Flask):**
```bash
cd bindings/python
source venv/bin/activate
python app.py
# Visit http://localhost:5000
```

**Node.js (Express):**
```bash
cd bindings/typescript
npm install
npm run dev
# Visit http://localhost:3000
```

## Full Architecture

### Directory Structure

```
f2v2f/
├── src/                    # Rust library core
│   ├── main.rs            # CLI application
│   ├── lib.rs             # Library exports
│   ├── ffi.rs             # C FFI bindings ⭐ NEW
│   ├── encoder.rs         # File → data encoding
│   ├── decoder.rs         # Video → file decoding
│   ├── image_generator.rs # Geometric art
│   ├── video_composer.rs  # Video assembly
│   ├── config.rs          # Configuration
│   └── error.rs           # Error types
│
├── bindings/              # Multi-language binding ⭐ NEW
│   ├── lib/               # Compiled shared libraries
│   │   ├── libf2v2f.so    # Linux
│   │   ├── libf2v2f.dylib # macOS
│   │   └── f2v2f.dll      # Windows
│   │
│   ├── python/            # Python FFI + Flask ⭐ NEW
│   │   ├── f2v2f.py       # ctypes wrapper
│   │   ├── app.py         # Flask REST API
│   │   ├── setup.py       # Package installation
│   │   ├── requirements.txt
│   │   ├── venv/          # Python virtual environment
│   │   └── uploads/       # Temp file storage
│   │
│   ├── typescript/        # TypeScript FFI + Express ⭐ NEW
│   │   ├── index.ts       # Node FFI wrapper
│   │   ├── app.ts         # Express REST API
│   │   ├── package.json   # npm configuration
│   │   ├── dist/          # Compiled JS
│   │   ├── node_modules/  # npm dependencies
│   │   └── uploads/       # Temp file storage
│   │
│   └── frontend/          # Web UI ⭐ NEW
│       └── index.html     # Single-page app
│
├── examples/
│   ├── encode_file.rs     # Rust example
│   ├── decode_video.rs    # Rust example
│   ├── python_binding.py  # Python example ⭐ NEW
│   └── typescript_binding.ts # TypeScript example ⭐ NEW
│
├── build.sh               # Unix build script ⭐ NEW
├── build.bat              # Windows build script ⭐ NEW
├── Cargo.toml             # Rust manifest
├── ARCHITECTURE.md        # Design documentation ⭐ NEW
├── INTEGRATION_PHASE_2_3.md # Web integration guide
├── QUICKSTART.md          # User quick start
├── README.md              # User documentation
└── .gitignore
```

## Language Bindings Overview

### Rust (Native)

**Use when:**
- Maximum performance needed
- Standalone CLI application
- No language interop required

**Building:**
```bash
cargo build --release
./target/release/f2v2f --help
```

### Python FFI (via ctypes)

**Use when:**
- Python application integration
- Data science workflows
- Flask/Django backends

**BuildandUse:**
```bash
cd bindings/python
pip install -r requirements.txt
pip install -e .

# In Python code
from f2v2f import Encoder
encoder = Encoder()
encoder.encode("file.pdf", "output.mp4")
```

**Key files:**
- `f2v2f.py` - ctypes wrapper (100 lines)
- `app.py` - Flask REST API example (300 lines)

### TypeScript/Node.js FFI

**Use when:**
- JavaScript/Node.js applications
- Express/Fastify backends
- Browser-based services

**Build and use:**
```bash
cd bindings/typescript
npm install
npm run build

// In TypeScript code
import { Encoder } from './index';
const encoder = new Encoder();
await encoder.encode("file.pdf", "output.mp4");
```

**Key files:**
- `index.ts` - FFI wrapper with types (200 lines)
- `app.ts` - Express REST API example (350 lines)

## API Endpoints

Both Flask and Express backends expose identical REST APIs:

### `/api/health` (GET)
Check if service is running
```bash
curl http://localhost:5000/api/health
# {"status": "healthy"}
```

### `/api/version` (GET)
Get library version
```bash
curl http://localhost:5000/api/version
# {"version": "f2v2f v0.1.0"}
```

### `/api/encode` (POST)
Encode file to video

**Parameters:**
- `file` (file) - Input file to encode
- `width` (int) - Video width (default: 1920)
- `height` (int) - Video height (default: 1080)
- `fps` (int) - Frames per second (default: 30)
- `chunk_size` (int) - File chunk size in bytes (default: 65536)

**Response:**
```json
{
  "job_id": "uuid",
  "status": "pending",
  "message": "Encoding started"
}
```

### `/api/decode` (POST)
Decode video back to file

**Parameters:**
- `file` (file) - Input video to decode

**Response:**
```json
{
  "job_id": "uuid",
  "status": "pending",
  "message": "Decoding started"
}
```

### `/api/status/{jobId}` (GET)
Get job status

**Response:**
```json
{
  "job_id": "uuid",
  "operation": "encode",
  "status": "running|pending|completed|failed",
  "progress": 45,
  "error": null,
  "result_url": "/api/download/filename.mp4"
}
```

### `/api/download/{filename}` (GET)
Download result file

## Deployment Options

### Option 1: Docker with Python/Flask

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY src src
COPY Cargo.* .
RUN cargo build --release --lib

FROM python:3.11-slim
RUN apt-get update && apt-get install -y ffmpeg
COPY --from=builder /app/target/release/libf2v2f.so /usr/lib/
COPY bindings/python /app/
WORKDIR /app
RUN pip install -r requirements.txt
CMD ["python", "app.py"]
```

**Build and run:**
```bash
docker build -t f2v2f:latest .
docker run -p 5000:5000 f2v2f:latest
```

### Option 2: Docker with Node.js/Express

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY src src
COPY Cargo.* .
RUN cargo build --release --lib

FROM node:18-alpine
COPY --from=builder /app/target/release/libf2v2f.so /usr/lib/
COPY bindings/typescript /app/
WORKDIR /app
RUN npm install && npm run build
CMD ["npm", "start"]
```

### Option 3: Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: f2v2f-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: f2v2f
  template:
    metadata:
      labels:
        app: f2v2f
    spec:
      containers:
      - name: api
        image: f2v2f:latest
        ports:
        - containerPort: 5000
        env:
        - name: FLASK_ENV
          value: "production"
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

Deploy:
```bash
kubectl apply -f deployment.yaml
kubectl port-forward svc/f2v2f-api 5000:5000
```

## Performance Tuning

### For Large Files (>1GB)

1. **Use Release build:**
   ```bash
   cargo build --release --lib
   # 50-100x faster than debug
   ```

2. **Increase chunk size:**
   ```bash
   ./target/release/f2v2f encode huge.iso output.mp4 --chunk-size 262144
   # Larger chunks = faster encoding
   ```

3. **Lower resolution:**
   ```bash
   ./target/release/f2v2f encode large.bin output.mp4 --resolution 1280x720
   # Reduces video file size
   ```

4. **Adjust worker threads:**
   - Rust automatically uses all CPU cores
   - For Docker/cloud, consider CPU limits

### Memory Optimization

The system uses **streaming** - constant memory regardless of file size:

- Encodes process 64KB chunks
- Each chunk becomes one frame
- Frame written immediately
- No buffering entire file

Maximum memory usage: ~100MB for any file size

## Error Handling & Exceptions

### Rust Level
- All errors return error codes (0 = success)
- No panics in FFI layer
- Stack traces preserved for debugging

### Python Level

```python
try:
    encoder.encode("file.pdf", "output.mp4")
except EncodingError as e:
    print(f"Encoding failed: {e}")
except InvalidInputError as e:
    print(f"Invalid input: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

### TypeScript Level

```typescript
try {
    await encoder.encode("file.pdf", "output.mp4");
} catch (error) {
    if (error instanceof F2V2FError) {
        console.error(`Code ${error.code}: ${error.message}`);
    } else {
        throw error;
    }
}
```

### Web API Level

All endpoints return appropriate HTTP status codes:
- **202**: Job accepted (async operation)
- **400**: Invalid input
- **404**: Job/file not found
- **413**: File too large
- **500**: Server error

## Monitoring & Logging

### Python/Flask

```bash
# Enable debug logging
FLASK_LOG_LEVEL=DEBUG flask run
```

### TypeScript/Express

```bash
# Enable debug output
DEBUG=* npm run dev
```

### Docker Logs

```bash
docker logs -f container_id
docker logs --timestamps container_id
```

## Testing

### Unit Tests
```bash
cargo test --lib --release
```

### Integration Tests
```bash
cargo test --release
```

### Python Tests
```bash
cd bindings/python
pytest tests/
```

### TypeScript Tests
```bash
cd bindings/typescript
npm test
```

## Troubleshooting

### "Library not found" Error

**Problem:** FFI can't find shared library

**Solution:**
```bash
# Rebuild fresh
cargo clean
cargo build --release --lib

# Verify library exists
file target/release/libf2v2f.so

# Set library path (macOS/Linux)
export LD_LIBRARY_PATH=$(pwd)/bindings/lib:$LD_LIBRARY_PATH
export DYLD_LIBRARY_PATH=$(pwd)/bindings/lib:$DYLD_LIBRARY_PATH
```

### Out of Memory During Encoding

**Problem:** Process uses too much memory

**Solution:**
1. Use smaller chunk size: `--chunk-size 16384`
2. Use lower resolution: `--resolution 1280x720`
3. Restart after each file

### Slow Encoding

**Problem:** Encoding takes very long

**Solution:**
```bash
# Verify using Release build (CRITICAL)
cargo build --release --lib

# Check system resources
top  # Or Task Manager on Windows

# Use lower resolution  
--resolution 1280x720

# Use larger chunks
--chunk-size 262144
```

## Next Steps

1. **Advanced Features:**
   - Add authentication (JWT/OAuth2)
   - Implement database for job history
   - Add queue system (Celery/Bull)
   - Implement caching layer

2. **Optimization:**
   - GPU acceleration for art generation
   - SIMD optimization for pattern generation
   - Parallel frame composition

3. **Scaling:**
   - Horizontal scaling with load balancing
   - Distributed job queue
   - Microservices architecture

## References

- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [ctypes Documentation](https://docs.python.org/3/library/ctypes.html)
- [Node FFI Package](https://github.com/node-ffi/node-ffi)
- [Flask Documentation](https://flask.palletsprojects.com/)
- [Express Documentation](https://expressjs.com/)
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)

## Support

For issues or questions:
1. Check [ARCHITECTURE.md](ARCHITECTURE.md) for design details
2. Review [QUICKSTART.md](QUICKSTART.md) for usage examples
3. Check error messages - they're descriptive
4. Ensure you're using Release builds for benchmarking

---

**Happy encoding! 🎉**
