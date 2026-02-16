# f2v2f Quick Start Guide ⚡

Get f2v2f running in 3 minutes with the new Go backend.

## Prerequisites Check

```bash
rustc --version      # Should be 1.70+
go version           # Should be 1.20+
node --version       # Should be 16+
npm --version        # Should be 8+
ffmpeg -version      # Should be 7.0+
```

## Start Development (The Fast Way)

### One-Time Setup
```bash
cd f2v2f
make all           # Build everything (core, backend, frontend)
```

### Run It
```bash
# Terminal 1: Start Backend
make backend-run   # runs on http://localhost:5001

# Terminal 2: Start Frontend
make frontend-dev  # runs on http://localhost:3000

# Open http://localhost:3000 in your browser
```

## Common Commands

| What | Command |
|------|---------|
| Build everything | `make all` |
| Build just core | `make core` |
| Build Go backend | `make backend` |
| Run Go backend | `make backend-run` |
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
# Backend: http://localhost:5001
```

## File Locations

```
Rust Core:           lib/src/*.rs
Golang Backend:      backend/main.go
Next.js Frontend:    frontend/app/page.tsx
React Components:    frontend/components/
Tailwind Styles:     frontend/styles/globals.css
API Client:          frontend/lib/api.ts
```

## First Steps

1. **Upload a File**
   - Click "Encode" tab
   - Drag & drop a file
   - Watch progress (now faster with Go!)
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
   - See all your past encodes/decodes (stored in SQLite)
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
# Change port (Go backend)
PORT=5002 make backend-run
PORT=3001 make frontend-dev
```

### Still stuck?
See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture.

## Project Structure

```
f2v2f/
├── lib/                      # Rust Core (Performance engine)
├── backend/                  # Golang Fiber REST API
│   ├── main.go               # Main server
│   ├── f2v2f/                # Go bindings for Rust
│   └── f2v2f.db              # SQLite Database
├── frontend/                 # Next.js web app
│   ├── app/page.tsx         # Main interface
│   ├── components/          # React components
│   └── lib/api.ts           # API calls
└── Makefile.new             # Build commands
```

## API Quick Reference

All endpoints: `http://localhost:5001/api/`

```bash
# Start encoding
curl -X POST -F "file=@myfile.pdf" \
  http://localhost:5001/api/encode
# Returns: {"job_id": "xxx", "status": "pending"}

# Check status
curl http://localhost:5001/api/status/xxx
# Returns: status, progress, etc.

# Get file history
curl http://localhost:5001/api/files
# Returns: array of files

# Download result
curl -O http://localhost:5001/api/download/output.mp4
```

## Configuration

**Backend** (.env):
```bash
PORT=5001
ENCODING_WIDTH=1920
ENCODING_HEIGHT=1080
ENCODING_FPS=30
```

**Frontend** (.env.local):
```bash
NEXT_PUBLIC_API_URL=http://localhost:5001/api
```

## Development Tips

### Hot Reload
- Backend: Rebuild and restart (edit `main.go`)
- Frontend: Automatic (edit `app/page.tsx`)

### Debug Mode
- Go backend provides detailed logs in the terminal.
- Frontend uses standard Next.js dev server.

## Next Steps

1. ✅ Get it running (follow above)
2. ✅ Test with small file (< 10MB)
3. ✅ Explore the UI
4. ✅ Read [README.md](README.md) for full docs
5. ✅ Check [ARCHITECTURE.md](ARCHITECTURE.md) for details
6. 🚀 Deploy with Docker!

---

**Ready?**
```bash
cd f2v2f && make all && make backend-run &
(cd frontend && npm run dev)
```

Open http://localhost:3000 and start encoding! 🎉
