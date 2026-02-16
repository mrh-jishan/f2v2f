#!/bin/bash

# F2V2F Quick Start - Launches both backend and frontend

PROJECT_DIR="/Users/robin-hassan/Desktop/f2v2f"

# Set environment variables
export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"
export DYLD_LIBRARY_PATH="/usr/local/opt/ffmpeg@7/lib:$PROJECT_DIR/lib/target/release:$DYLD_LIBRARY_PATH"
export NEXT_PUBLIC_API_URL="http://localhost:5000"

echo "🚀 F2V2F System Starting..."
echo ""

# Start backend
echo "📦 Starting Flask backend on port 5000..."
cd "$PROJECT_DIR/backend"
python3 app.py > backend.log 2>&1 &
BACKEND_PID=$!
echo "   Backend PID: $BACKEND_PID"

# Wait for backend to start
sleep 2

# Start frontend
echo "⚛️  Starting Next.js frontend on port 3000..."
cd "$PROJECT_DIR/frontend"
NEXT_PUBLIC_API_URL="http://localhost:5000" npm run dev > frontend.log 2>&1 &
FRONTEND_PID=$!
echo "   Frontend PID: $FRONTEND_PID"

# Wait and check if started
sleep 3

if curl -s http://localhost:3000 > /dev/null 2>&1; then
    echo ""
    echo "✓ Frontend is ready at http://localhost:3000"
fi

if curl -s http://localhost:5000/health > /dev/null 2>&1; then
    echo "✓ Backend is ready at http://localhost:5000"
fi

echo ""
echo "================================================"
echo "  System Running!"
echo "================================================"
echo "Frontend: http://localhost:3000"
echo "Backend:  http://localhost:5000"
echo ""
echo "Logs:"
echo "  Backend:  $PROJECT_DIR/backend/backend.log"
echo "  Frontend: $PROJECT_DIR/frontend/frontend.log"
echo ""
echo "To stop, run:"
echo "  kill $BACKEND_PID $FRONTEND_PID"
echo ""

# Keep script running
wait
