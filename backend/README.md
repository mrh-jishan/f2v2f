# F2V2F Python Backend (Flask REST API)

Flask REST API server that provides file encoding/decoding services using the Rust f2v2f library via FFI.

## 🚀 Quick Start

```bash
# Install dependencies
pip install -r requirements.txt

# Run server
python3 app.py

# Server starts on http://localhost:5000
```

## 📦 What's Here

| File | Purpose |
|------|---------|
| `app.py` | Flask REST API server with background job processing |
| `f2v2f.py` | Python ctypes wrapper for Rust library |
| `requirements.txt` | Python dependencies |
| `setup.py` | Package installation config |
| `uploads/` | User uploaded files (auto-created) |
| `outputs/` | Encoded/decoded results (auto-created) |

## 🔧 Installation

### Prerequisites
- Python 3.8+
- Rust library built (`../lib/target/release/libf2v2f.dylib`)
- FFmpeg 7.1.3

### Setup

```bash
cd backend

# Create virtual environment (optional)
python3 -m venv venv
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Verify library is found
python3 -c "import f2v2f; print(f2v2f.version())"
```

## 🌐 API Endpoints

Base URL: `http://localhost:5000/api`

### Encode File

```bash
POST /api/encode
Content-Type: multipart/form-data

# Parameters:
- file: File to encode
- width: Video width (default: 1920)
- height: Video height (default: 1080)
- fps: Frames per second (default: 30)
- chunk_size: Chunk size in bytes (default: 65536)

# Response:
{
  "job_id": "uuid",
  "status": "pending"
}
```

### Decode File

```bash
POST /api/decode
Content-Type: multipart/form-data

# Parameters:
- file: MP4 file to decode

# Response:
{
  "job_id": "uuid",
  "status": "pending"
}
```

### Check Job Status

```bash
GET /api/status/<job_id>

# Response:
{
  "job_id": "uuid",
  "status": "completed|running|failed|pending",
  "progress": 0-100,
  "result_url": "/api/download/filename.mp4",
  "error_message": "..." (if failed)
}
```

### List Files

```bash
GET /api/files

# Response:
[
  {
    "filename": "file.mp4",
    "type": "encoded",
    "size": 12345,
    "timestamp": "2026-02-16T...",
    "url": "/api/download/file.mp4"
  },
  ...
]
```

### Download File

```bash
GET /api/download/<filename>

# Returns the file for download
```

### Health Check

```bash
GET /health

# Response:
{"status": "ok"}
```

## 🔌 How It Works

### 1. File Upload
```
User uploads file → Saved to uploads/ → Job created → Returns job_id
```

### 2. Background Processing
```
Flask thread → Calls f2v2f.py wrapper → Rust library (FFI) → FFmpeg → Output file
```

### 3. Progress Tracking
```
Rust calls Python callback → Update job progress → Frontend polls status
```

### 4. File Retrieval
```
Job completes → File saved to outputs/ → Frontend downloads via /api/download
```

## 🛠️ Configuration

### Environment Variables

```bash
# Optional - auto-detected if not set
export DYLD_LIBRARY_PATH="/usr/local/opt/ffmpeg@7/lib:../lib/target/release"
```

### Flask Settings

```python
# In app.py
UPLOAD_FOLDER = "uploads"
OUTPUT_FOLDER = "outputs"
MAX_CONTENT_LENGTH = 5 * 1024 * 1024 * 1024  # 5GB
```

## 🧪 Testing

```bash
# Test library loading
python3 -c "from f2v2f import Encoder; print('OK')"

# Test health endpoint
curl http://localhost:5000/health

# Test encode (with file)
curl -X POST -F "file=@test.txt" http://localhost:5000/api/encode
```

## 🔍 Troubleshooting

### Library Not Found

```bash
# Check if library exists
ls -lh ../lib/target/release/libf2v2f.dylib

# Set library path
export DYLD_LIBRARY_PATH="../lib/target/release:$DYLD_LIBRARY_PATH"

# Verify
python3 -c "import f2v2f; print(f2v2f.version())"
```

### FFmpeg Not Found

```bash
# Check FFmpeg
ffmpeg -version

# Install if missing
brew install ffmpeg@7

# Set environment
export DYLD_LIBRARY_PATH="/usr/local/opt/ffmpeg@7/lib:$DYLD_LIBRARY_PATH"
```

### Port Already in Use

```bash
# Kill process on port 5000
lsof -i :5000 | grep -v COMMAND | awk '{print $2}' | xargs kill -9

# Or use different port
flask run --port 5001
```

### Encoding Fails

```bash
# Check logs
tail -f app.log

# Verify library works
python3 -c "from f2v2f import Encoder; e = Encoder(); print('Library OK')"

# Check FFmpeg
which ffmpeg
ffmpeg -version
```

## 📊 File Structure

```
backend/
├── app.py              # Flask server
├── f2v2f.py           # Python FFI wrapper
├── requirements.txt    # Dependencies
├── setup.py           # Package config
├── uploads/           # Uploaded files (auto-created)
├── outputs/           # Encoded/decoded files (auto-created)
└── file_registry.json # File history (auto-created)
```

## 🔐 Security

- ✅ File size limits (5GB default)
- ✅ File type validation
- ✅ CORS enabled for localhost:3000
- ✅ Secure file naming (UUIDs)
- ✅ Error handling for all endpoints

## 📚 Dependencies

- `Flask` - Web framework
- `Flask-CORS` - Cross-origin requests
- `Werkzeug` - File handling utilities
- `python-dotenv` - Environment variables

See [requirements.txt](requirements.txt) for versions.

## 🚀 Production Deployment

See [../DEPLOYMENT.md](../DEPLOYMENT.md) for:
- Docker deployment
- Gunicorn configuration
- Nginx reverse proxy
- Background job queues (Celery)

## 🔗 Related Documentation

- **Rust Library**: [../lib/README.md](../lib/README.md)
- **Frontend**: [../frontend/README.md](../frontend/README.md)
- **Complete Guide**: [../COMPLETE_GUIDE.md](../COMPLETE_GUIDE.md)

---

**Built with Flask**  
**Powered by Rust FFI**  
**Ready for Production**
