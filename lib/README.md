# F2V2F Rust Library

High-performance file encoding library with FFI (Foreign Function Interface) bindings for Python, TypeScript/Node.js, and other languages.

## 🚀 Quick Start

### Building the Library

```bash
cd lib

# Set environment variables for FFmpeg 7
export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"

# Build in release mode (optimized)
cargo build --release --lib

# Output: target/release/libf2v2f.dylib (macOS)
#         target/release/libf2v2f.so (Linux)
#         target/release/f2v2f.dll (Windows)
```

### Prerequisites

**Required:**
- Rust 1.70 or later (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- FFmpeg 7.1.3 (`brew install ffmpeg@7` on macOS)
- pkg-config (`brew install pkg-config` on macOS)

**Why FFmpeg 7?**
FFmpeg 8.0 has breaking changes. This library is tested with FFmpeg 7.1.3.

## 📦 What Gets Built

```
lib/target/release/
├── libf2v2f.dylib    # macOS dynamic library (761KB)
├── libf2v2f.so       # Linux shared object
├── f2v2f.dll         # Windows DLL
└── libf2v2f.a        # Static library (optional)
```

## 🔧 Using the Library

### From Golang

```go
import "github.com/mrh-jishan/f2v2f/backend/f2v2f"

// Encode
encoder, _ := f2v2f.NewEncoder(1920, 1080, 30, 4096, true, 3)
encodedSize, _ := encoder.Encode("input.pdf", "output.mp4")

// Decode
decoder, _ := f2v2f.NewDecoder(1920, 1080, 4096, true, encodedSize)
decoder.Decode("output.mp4", "restored.pdf")
```

**Go Setup:**
```bash
cd ../backend
go build -o server main.go
./server
```

### From TypeScript/Node.js

```typescript
import { Encoder, Decoder } from './f2v2f';

// Encode
const encoder = new Encoder({ width: 1920, height: 1080, fps: 30 });
await encoder.encode("input.pdf", "output.mp4");

// Decode
const decoder = new Decoder();
await decoder.decode("output.mp4", "recovered.pdf");
```

**Node.js Setup:**
```bash
cd ../frontend
npm install
node -e "const f2v2f = require('./f2v2f'); console.log('Library loaded');"
```

### From C/C++

```c
#include <stdio.h>

// Function signatures
extern "C" {
    int f2v2f_init();
    void* f2v2f_encode_create(uint32_t width, uint32_t height, uint32_t fps, size_t chunk_size);
    int f2v2f_encode_file(void* handle, const char* input, const char* output, void* callback);
    void f2v2f_encode_free(void* handle);
}

int main() {
    f2v2f_init();
    
    void* encoder = f2v2f_encode_create(1920, 1080, 30, 65536);
    int result = f2v2f_encode_file(encoder, "input.txt", "output.mp4", NULL);
    f2v2f_encode_free(encoder);
    
    printf("Encoding %s\n", result == 0 ? "succeeded" : "failed");
    return 0;
}
```

**Compile:**
```bash
gcc -o test test.c -L./target/release -lf2v2f
./test
```

## 🏗️ Architecture

### Core Modules

| Module | Purpose |
|--------|---------|
| `encoder.rs` | File encoding logic |
| `decoder.rs` | File decoding logic |
| `image_generator.rs` | Geometric art generation |
| `video_composer.rs` | FFmpeg video composition |
| `ffi.rs` | C FFI interface ⭐ |
| `config.rs` | Configuration structs |
| `error.rs` | Error handling |

### FFI Exports

All exported functions are in [src/ffi.rs](src/ffi.rs):

```rust
// Initialization
pub extern "C" fn f2v2f_init() -> i32;
pub extern "C" fn f2v2f_version() -> *const c_char;

// Encoding
pub extern "C" fn f2v2f_encode_create(width: u32, height: u32, fps: u32, chunk_size: usize) -> *mut EncodeHandle;
pub extern "C" fn f2v2f_encode_file(handle: *mut EncodeHandle, input: *const c_char, output: *const c_char, callback: Option<ProgressCallback>) -> i32;
pub extern "C" fn f2v2f_encode_free(handle: *mut EncodeHandle);

// Decoding
pub extern "C" fn f2v2f_decode_create() -> *mut DecodeHandle;
pub extern "C" fn f2v2f_decode_file(handle: *mut DecodeHandle, input: *const c_char, output: *const c_char, callback: Option<ProgressCallback>) -> i32;
pub extern "C" fn f2v2f_decode_free(handle: *mut DecodeHandle);
```

### Error Codes

```c
enum F2V2FErrorCode {
    Success = 0,
    InvalidInput = 1,
    IoError = 2,
    EncodingError = 3,
    DecodingError = 4,
    ConfigError = 5,
    OperationInProgress = 6,
    InvalidHandle = 7,
    Unknown = 255
};
```

## 🧪 Testing

```bash
# Run all tests
cargo test --release

# Run specific test
cargo test --release test_encode_small_file

# Run with output
cargo test --release -- --nocapture

# Check for errors
cargo check --release
```

## 🔍 Troubleshooting

### Build Errors

| Error | Solution |
|-------|----------|
| `cargo: command not found` | Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `FFmpeg headers not found` | Set `CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"` |
| `avfft.h not found` | Downgrade to FFmpeg 7: `brew install ffmpeg@7` |
| `ld: library not found -lavformat` | Set `LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"` |
| `pkg-config missing` | Install: `brew install pkg-config` |

### Runtime Errors

| Error | Solution |
|-------|----------|
| `Library not found` | Set `DYLD_LIBRARY_PATH` (macOS) or `LD_LIBRARY_PATH` (Linux) |
| `ffmpeg not found` | Ensure FFmpeg is in PATH |
| `Encoding failed` | Check FFmpeg installation: `ffmpeg -version` |
| `Invalid handle` | Call `f2v2f_encode_create()` before `f2v2f_encode_file()` |

### Environment Variables (macOS)

```bash
# For building
export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"
export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"

# For running
export DYLD_LIBRARY_PATH="/usr/local/opt/ffmpeg@7/lib:$DYLD_LIBRARY_PATH"
```

### Environment Variables (Linux)

```bash
# For building
export PKG_CONFIG_PATH="/usr/lib/pkgconfig:$PKG_CONFIG_PATH"

# For running
export LD_LIBRARY_PATH="/usr/lib:$LD_LIBRARY_PATH"
```

## 📊 Performance

| File Size | Encode Time | Decode Time | Memory Usage |
|-----------|-------------|-------------|--------------|
| 1 MB | ~100ms | ~80ms | ~50 MB |
| 10 MB | ~500ms | ~400ms | ~60 MB |
| 100 MB | ~5s | ~4s | ~80 MB |
| 1 GB | ~1-2min | ~1-2min | ~100 MB |

**Streaming Architecture:**
- Constant memory usage regardless of file size
- No temporary files created
- Progress callbacks every chunk

## 🔐 Security

- ✅ No unsafe operations exposed in public API
- ✅ Input validation on all FFI boundaries
- ✅ SHA256 checksums verify data integrity
- ✅ Memory-safe Rust prevents buffer overflows
- ✅ Error handling prevents crashes

## 📝 Development

### Adding New Features

1. **Add to Rust code** (e.g., `src/encoder.rs`)
2. **Export via FFI** in `src/ffi.rs`
3. **Update Go wrapper** in `../backend/f2v2f/bindings.go`
4. **Update TypeScript wrapper** in `../frontend/lib/api.ts`
5. **Rebuild library**: `cargo build --release --lib`
6. **Test changes**: `cargo test --release`

### Code Style

```bash
# Format code
cargo fmt

# Lint
cargo clippy --release

# Fix warnings
cargo fix --lib -p f2v2f
```

## 📚 Dependencies

See [Cargo.toml](Cargo.toml) for full list. Key dependencies:

- `tokio` - Async runtime
- `tracing` - Logging
- `sha2` - SHA256 checksums
- `image` - Image processing
- `indicatif` - Progress bars
- `lazy_static` - Static initialization

## 🔗 Related Documentation

- **Backend Integration**: [../backend/README.md](../backend/README.md)
- **Frontend Integration**: [../frontend/README.md](../frontend/README.md)
- **Complete Guide**: [../COMPLETE_GUIDE.md](../COMPLETE_GUIDE.md)
- **API Documentation**: [../ARCHITECTURE.md](../ARCHITECTURE.md)
- **Deployment**: [../DEPLOYMENT.md](../DEPLOYMENT.md)

## 📄 License

See [../README.md](../README.md) for license information.

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Make changes in `lib/src/`
4. Add tests
5. Run `cargo test --release`
6. Run `cargo fmt` and `cargo clippy`
7. Commit (`git commit -m 'Add amazing feature'`)
8. Push (`git push origin feature/amazing-feature`)
9. Open Pull Request

---

**Built with Rust 🦀**  
**Powered by FFmpeg 🎬**  
**Ready for Production ✅**
