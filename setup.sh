#!/bin/bash

# F2V2F - Complete System Startup Script
# This script sets up all environment variables and starts both services

set -e

PROJECT_DIR="/Users/robin-hassan/Desktop/f2v2f"
cd "$PROJECT_DIR"

echo "================================================"
echo "  F2V2F - File to Video Converter"
echo "  System Startup"
echo "================================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# ============================================
# 1. Verify Rust library exists
# ============================================
echo -e "${BLUE}[1/5]${NC} Checking Rust library..."
if [ -f "lib/target/release/libf2v2f.dylib" ]; then
    LIB_SIZE=$(du -h lib/target/release/libf2v2f.dylib | cut -f1)
    echo -e "${GREEN}✓${NC} Rust library found ($LIB_SIZE)"
else
    echo -e "${YELLOW}⚠${NC} Rust library not found. Building..."
    cd lib
    export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
    export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
    export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"
    export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"
    
    cargo build --release --lib
    cd ..
    echo -e "${GREEN}✓${NC} Rust library built"
fi

# ============================================
# 2. Check FFmpeg
# ============================================
echo ""
echo -e "${BLUE}[2/5]${NC} Checking FFmpeg..."
if [ -d "/usr/local/opt/ffmpeg@7" ]; then
    FFMPEG_VERSION=$(/usr/local/opt/ffmpeg@7/bin/ffmpeg -version 2>&1 | head -1)
    echo -e "${GREEN}✓${NC} FFmpeg 7 found: $FFMPEG_VERSION"
else
    echo -e "${YELLOW}⚠${NC} FFmpeg 7 not found. Installing..."
    brew install ffmpeg@7
    echo -e "${GREEN}✓${NC} FFmpeg 7 installed"
fi

# ============================================
# 3. Set up environment variables
# ============================================
echo ""
echo -e "${BLUE}[3/5]${NC} Setting environment variables..."

export FFMPEG_DIR="/usr/local/opt/ffmpeg@7"
export LDFLAGS="-L/usr/local/opt/ffmpeg@7/lib"
export CPPFLAGS="-I/usr/local/opt/ffmpeg@7/include"
export PKG_CONFIG_PATH="/usr/local/opt/ffmpeg@7/lib/pkgconfig:$PKG_CONFIG_PATH"
export DYLD_LIBRARY_PATH="/usr/local/opt/ffmpeg@7/lib:$PROJECT_DIR/lib/target/release:$DYLD_LIBRARY_PATH"
export NEXT_PUBLIC_API_URL="http://localhost:5000"

echo -e "${GREEN}✓${NC} Environment configured"
echo "  FFMPEG_DIR: $FFMPEG_DIR"
echo "  DYLD_LIBRARY_PATH includes FFmpeg and Rust library"

# ============================================
# 4. Check Node.js dependencies
# ============================================
echo ""
echo -e "${BLUE}[4/5]${NC} Checking Node.js dependencies..."
if [ -d "frontend/node_modules" ]; then
    echo -e "${GREEN}✓${NC} Node modules found"
else
    echo -e "${YELLOW}⚠${NC} Installing Node.js dependencies..."
    cd frontend
    npm install --silent
    cd "$PROJECT_DIR"
    echo -e "${GREEN}✓${NC} Dependencies installed"
fi

# ============================================
# 5. Ready to start
# ============================================
echo ""
echo -e "${BLUE}[5/5]${NC} Starting services..."
echo ""
echo -e "${GREEN}================================================${NC}"
echo -e "${GREEN}✓ System Ready!${NC}"
echo -e "${GREEN}================================================${NC}"
echo ""
echo "To start the services, run in separate terminals:"
echo ""
echo -e "${YELLOW}Terminal 1 - Backend:${NC}"
echo "  cd $PROJECT_DIR/backend"
echo "  python3 app.py"
echo ""
echo -e "${YELLOW}Terminal 2 - Frontend:${NC}"
echo "  cd $PROJECT_DIR/frontend"
echo "  npm run dev"
echo ""
echo -e "${YELLOW}Then open in browser:${NC}"
echo "  http://localhost:3000"
echo ""
echo "Full documentation: $PROJECT_DIR/BUILD_AND_RUN.md"
echo ""
