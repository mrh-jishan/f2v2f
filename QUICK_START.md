# f2v2f Quick Start Guide ⚡

Get f2v2f running in 3 minutes.

## Prerequisites Check

```bash
rustc --version      # Should be 1.70+
python3 --version    # Should be 3.8+
node --version       # Should be 16+
npm --version        # Should be 8+
```

## Start Development (The Fast Way)

### One-Time Setup
```bash
cd f2v2f
make all           # Build everything (5-10 minutes)
```

### Run It
```bash
# Terminal 1: Start Backend
make backend-run   # runs on http://localhost:5000

# Terminal 2: Start Frontend
make frontend-dev  # runs on http://localhost:3000

# Open http://localhost:3000 in your browser
```

## Common Commands

| What | Command |
|------|---------|
| Build everything | `make all` |
| Build just core | `make core` |
| Run Flask | `make backend-run` |
| Run Next.js | `make frontend-dev` |
| Run both (Docker) | `docker-compose -f docker-compose.new.yml up` |
| Test everything | `make test` |
| Clean all | `make clean` |
| View help | `make help` |

## Using Docker (Even Faster)

```bash
cd f2v2f
docker-compose -f docker-compose.new.yml up
# Frontend: http://localhost:3000
# Backend: http://localhost:5000
```

## File Locations

```
Rust Core:           lib/src/*.rs
Flask Backend:       backend/app.py
Next.js Frontend:    frontend/app/page.tsx
React Components:    frontend/components/
Tailwind Styles:     frontend/styles/globals.css
API Client:          frontend/lib/api.ts
```

## First Steps

1. **Upload a File**
   - Click "Encode" tab
   - Drag & drop a file
   - Watch progress
   - Download the video

2. **Watch the Video**
   - Click video player
   - Play and watch your file as art!

3. **Decode It Back**
   - Click "Decode" tab
   - Upload the video
   - Download original file
   - Verify it matches (should be identical!)

4. **Browse History**
   - Click "History" tab
   - See all your past encodes/decodes
   - Watch videos directly

## Troubleshooting

### Won't start?
```bash
make clean
make all
# Then try again
```

### "Library not found"?
```bash
ls lib/target/release/libf2v2f.*
# Should exist. If not: make core
```

### Port in use?
```bash
# Change port
FLASK_PORT=5001 make backend-run
PORT=3001 make frontend-dev
```

### Still stuck?
See [SETUP_GUIDE.md](SETUP_GUIDE.md) for detailed troubleshooting.

## Project Structure

```
f2v2f/
├── lib/                      # Rust (build once)
├── backend/                  # Flask REST API
│   ├── app.py               # Main server
│   ├── f2v2f.py             # Library wrapper
│   └── venv/                # Python environment
├── frontend/                 # Next.js web app
│   ├── app/page.tsx         # Main interface
│   ├── components/          # React components
│   └── lib/api.ts           # API calls
└── Makefile.new             # Build commands
```

## API Quick Reference

All endpoints: `http://localhost:5000/api/`

```bash
# Start encoding
curl -X POST -F "file=@myfile.pdf" \
  http://localhost:5000/api/encode
# Returns: {"job_id": "xxx"}

# Check status
curl http://localhost:5000/api/status/xxx
# Returns: status, progress, etc.

# Get file history
curl http://localhost:5000/api/files
# Returns: array of files

# Download result
curl -O http://localhost:5000/api/download/output.mp4
```

## Configuration

**Backend** (.env):
```bash
FLASK_PORT=5000
ENCODING_WIDTH=1920
ENCODING_HEIGHT=1080
ENCODING_FPS=30
```

**Frontend** (.env.local):
```bash
NEXT_PUBLIC_API_URL=http://localhost:5000/api
```

## Development Tips

### Hot Reload
- Backend: Restart Flask (edit `app.py`)
- Frontend: Automatic (edit `app/page.tsx`)

### Debug Mode
```bash
# Backend
FLASK_DEBUG=1 make backend-run

# Frontend (already in dev mode)
make frontend-dev
```

### Build for Production
```bash
cd frontend
npm run build
npm run start
```

## What to Explore

### Encoding Settings
- Resolution: 1280x720 to 3840x2160
- FPS: 24 to 60
- Chunk Size: 1KB to 10MB (smaller = more frames)

### File Types
- Encoding: Any file type (PDF, images, documents, etc.)
- Decoding: MP4 videos only

### File Limits
- Maximum size: 5GB per file
- Larger files = longer processing time

## Performance

| File | Encode | Decode |
|------|--------|--------|
| 1MB | ~100ms | ~80ms |
| 10MB | ~600ms | ~500ms |
| 100MB | ~6s | ~5s |
| 1GB | ~1-2m | ~1-2m |

## Next Steps

1. ✅ Get it running (follow above)
2. ✅ Test with small file (< 10MB)
3. ✅ Explore the UI
4. ✅ Read [README_NEW.md](README_NEW.md) for full docs
5. ✅ Check [SETUP_GUIDE.md](SETUP_GUIDE.md) for details
6. 🚀 Deploy with Docker or to cloud!

## Help & Support

- **Setup Issues**: [SETUP_GUIDE.md](SETUP_GUIDE.md)
- **Full Docs**: [README_NEW.md](README_NEW.md)
- **Architecture**: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- **Deployment**: [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)
- **Issues**: GitHub Issues page

---

**Ready?**
```bash
cd f2v2f && make all && make backend-run &
(cd frontend && npm run dev)
```

Open http://localhost:3000 and start encoding! 🎉
