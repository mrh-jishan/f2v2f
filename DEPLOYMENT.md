# f2v2f - Deployment & Production Guide (Golang)

This guide covers deploying the f2v2f system with the high-performance Go backend.

## 🚀 Deployment Overview

The f2v2f system consists of three main components that need to be deployed together:

1. **Rust Core Library:** The heavy-lifting engine.
2. **Go Backend:** The high-concurrency web service (Primary).
3. **Next.js Frontend:** The user interface.

## 🐳 Docker Deployment (Recommended)

The easiest way to deploy f2v2f is using Docker Compose.

### Step 1: Build and Start
```bash
docker-compose -f docker-compose.yml up --build -d
```

### Step 2: Verify Services
- **Frontend:** http://localhost:3000
- **Backend:** http://localhost:5001

### Step 3: Check Logs
```bash
docker-compose -f docker-compose.yml logs -f
```

---

## 🏗️ Manual Production Build

### 1. Build Rust Core (Release)
```bash
cd lib
cargo build --release --lib
```

### 2. Build Go Backend
```bash
cd backend
go build -o server main.go
```

### 3. Build Next.js Frontend
```bash
cd frontend
npm install
npm run build
npm run start
```

---

## ⚙️ Configuration

### Environment Variables

**Backend (Go):**
- `PORT`: Server port (default: 5001)
- `GOLANG_ENV`: Set to `production`
- `ENCODING_WIDTH`: Default video width (1920)
- `ENCODING_HEIGHT`: Default video height (1080)

**Frontend:**
- `NEXT_PUBLIC_API_URL`: URL of the Go backend (e.g., `http://api.f2v2f.com`)

---

## 📈 Performance Tuning

### CPU & Memory
- Encoding is CPU-bound. Ensure your server has at least 2-4 cores for smooth operation.
- Memory usage is constant due to streaming (~200MB per concurrent job).

### FFmpeg Optimization
- The Rust core uses FFmpeg for video encoding. Ensure `ffmpeg` is in the system PATH.
- For production, use a modern version of FFmpeg (7.0+).

---

## 🛡️ Security Best Practices

1. **Reverse Proxy:** Use Nginx or Caddy as a reverse proxy for both frontend and backend.
2. **Rate Limiting:** Implement rate limiting on the `/api/encode` endpoint to prevent abuse.
3. **Storage:** Regularly clean up the `uploads/` and `outputs/` directories.
4. **SSL/TLS:** Always serve over HTTPS in production.

---

## 📊 Monitoring

- **Health Checks:** Monitor `http://localhost:5001/health`.
- **System Metrics:** Track CPU and Disk I/O during large encoding jobs.
- **Logs:** Go backend logs to stdout by default; redirect to a file or log aggregator as needed.

---

**Ready for production! 🚀**
