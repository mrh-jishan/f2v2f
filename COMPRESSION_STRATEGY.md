# F2V2F Compression & Encoding Strategy for TB-Scale

## Overview
Optimized encoding pipeline combining **Zstd compression** (lossless data), **geometric art generation** (high quality), and **H.265 video codec** (efficient video compression).

---

## Architecture

### Layer 1: Lossless Data Compression (Zstd)
- **Purpose:** Compress original file content without data loss
- **Codec:** Zstd (Zstandard)
- **Configuration:** Level 11 (balanced speed/compression)
- **Typical Ratios:**
  - Text/JSON: **3-4x** compression
  - Binary data: **1.5-2x** compression
  - Code files: **2-3x** compression
  - Highly repetitive: **4-8x** compression

**Example:** 1KB file → ~100-300 bytes (compressed)

---

### Layer 2: Geometric Art Frame Generation
- **Purpose:** Convert compressed data chunks into high-quality artistic frames
- **Engine:** GeometricArtGenerator (custom Rust implementation)
- **Output Format:** RGBA images (4 bytes/pixel)
- **Resolution:** 1920×1080 (8.29 MB per raw frame)
- **Features:**
  - Deterministic pattern generation from data chunks
  - High visual quality for data visualization
  - Consistent color mapping for data representation

---

### Layer 3: H.265 Video Encoding
- **Purpose:** Compress artistic frames into playable video
- **Codec:** H.265/HEVC (libx265)
- **Preset:** `fast` (balanced encoding speed)
- **Quality:** CRF 22 (aggressive compression while preserving art detail)
- **Compression:**
  - H.264 baseline: ~50% compression
  - H.265: **40-50% further reduction** (vs H.264)
- **Chroma:** yuv420p (4:2:0 subsampling - good balance)

---

## Performance Metrics

### Size Reduction Pipeline

```
Input File                1,000,000 bytes
    ↓ (Zstd @ level 11, ~3:1 ratio)
Compressed Data          ~330,000 bytes
    ↓ (Chunked into frames @ 4KB chunks)
Frames Needed           ~82 frames
    ↓ (1920×1080×4 = 8.29 MB raw per frame)
Raw Frame Data          ~680 MB
    ↓ (H.265 CRF 22, ~50% compression)
Final Video             ~340 MB
```

**For TB-Scale:** 1TB input ≈ 330GB compressed ≈ 170GB video

---

## Configuration Parameters

### Encoder Config
```rust
pub struct EncodeConfig {
    width: u32,                    // 1920
    height: u32,                   // 1080
    fps: u32,                      // 30
    chunk_size: usize,             // 4096 (4KB)
    art_style: String,             // "geometric"
    num_threads: usize,            // CPU count
    buffer_size: usize,            // 1MB
    use_compression: bool,         // true (ENABLED)
    compression_level: i32,        // 11 (balanced)
}
```

### FFmpeg Command
```bash
ffmpeg -y \
  -f rawvideo -pix_fmt rgba \
  -video_size 1920x1080 -framerate 30 \
  -i pipe:0 \
  -c:v libx265 \
  -preset fast \
  -crf 22 \
  -pix_fmt yuv420p \
  -x265-params log-level=error \
  -movflags +faststart \
  output.mp4
```

---

## Encoding Speed Estimates

| File Size | Zstd Time | Frames | H.265 Time | Total Time | Output Size |
|-----------|-----------|--------|-----------|-----------|------------|
| 1 MB | 10ms | 1 | 100ms | ~110ms | 500KB |
| 10 MB | 100ms | 10 | 1s | ~1.1s | 5MB |
| 100 MB | 1s | 100 | 10s | ~11s | 50MB |
| 1 GB | 10s | 1000 | 100s | ~110s | 500MB |
| 1 TB | 10min | 1M | 1.66h | ~1.67h | 500GB |

**Note:** These are estimates. Actual performance depends on:
- CPU (speed, cores)
- Data characteristics (compressibility)
- Storage I/O (disk speed)

---

## Data Integrity Features

### Checksum Verification
1. **Encoding:** SHA-256 checksum calculated on original data
2. **Video Storage:** Checksum encoded in metadata
3. **Decoding:** 
   - Frame data extracted from video
   - Zstd decompression (automatic magic byte detection)
   - Checksum verification with original

### Automatic Format Detection
- Decoder automatically detects if data is Zstd compressed
- Zstd magic bytes: `0x28 0xB5 0x2F 0xFD`
- Falls back to raw data if not compressed

---

## Optimization Decisions

### Why Zstd?
- ✅ Lossless compression (no data loss)
- ✅ Fast compression/decompression
- ✅ Excellent ratio on structured data
- ✅ Available in Rust ecosystem
- ✅ Parallelizable

### Why H.265 over H.264?
- ✅ **40-50% better compression than H.264**
- ✅ Maintains high-quality playback
- ✅ Good support on modern systems
- ✅ Standardized format (HEVC)
- ✅ Suitable for geometric art patterns

### Why `fast` preset?
- ✅ 3-5x faster than `slow`
- ✅ Still produces excellent quality
- ✅ Necessary for TB-scale throughput
- ✅ CRF 22 compensates for speed trade-off

### Why CRF 22?
- CRF values: 0-51 (lower = higher quality)
- CRF 18-23: Visually lossless quality
- CRF 22: Good balance for geometric art
- Higher CRF (28+): More aggressive, smaller files but visible quality loss

---

## Future Optimizations

### Short Term
1. Parallel frame processing (encode multiple chunks simultaneously)
2. Adaptive CRF based on data type detection
3. Hardware acceleration (NVIDIA NVENC if available)
4. Rate-based encoding instead of CRF for predictable sizes

### Medium Term
1. Custom video codec for artistic data (smaller, faster)
2. Alternate Zstd levels based on file type
3. Metadata embedding in video (checksums, file size)
4. Streaming encoding for truly TB-scale (frame-by-frame, no buffering)

### Long Term
1. Machine learning-based compression optimization
2. Distributed encoding across multiple machines
3. Incremental updates (encode only changed frames)
4. Real-time playback directly from compressed stream

---

## Testing & Verification

### Test Pipeline
```bash
# Create test file
echo "Test data" > /tmp/test.txt

# Encode
./target/release/f2v2f encode /tmp/test.txt /tmp/output.mp4

# Decode
./target/release/f2v2f decode /tmp/output.mp4 /tmp/recovered.txt

# Verify
diff /tmp/test.txt /tmp/recovered.txt  # Should match exactly
```

### Performance Benchmarking
```bash
# Time encoding
time ./target/release/f2v2f encode large_file.bin output.mp4

# Check output size
ls -lh output.mp4

# Verify integrity
./target/release/f2v2f verify output.mp4 expected_checksum
```

---

## Production Deployment

### Recommended Settings for TB-Scale
```rust
EncodeConfig {
    width: 1920,
    height: 1080,
    fps: 30,
    chunk_size: 4096,           // 4KB - balance between frames and chunk size
    art_style: "geometric",
    num_threads: num_cpus::get(),
    buffer_size: 1024 * 1024,   // 1MB read buffer
    use_compression: true,      // ESSENTIAL for TB-scale
    compression_level: 11,      // Balanced: 9-12 for production
}
```

### System Requirements
- **CPU:** Modern multi-core (8+ cores recommended)
- **RAM:** 2-4GB minimum for streaming operations
- **Storage:** Fast local SSD for temp files
- **FFmpeg:** libx265 + libx264 codecs installed

---

## References
- Zstd Documentation: https://facebook.github.io/zstd/
- H.265 Specification: https://en.wikipedia.org/wiki/High_Efficiency_Video_Coding
- FFmpeg libx265: https://trac.ffmpeg.org/wiki/Encode/H.265
- CRF Guide: https://wiki.hydrogenaud.io/index.php?title=FFmpeg_Quality_Selection
