# F2V2F Next.js Frontend

Modern, responsive web interface for file encoding and decoding using the f2v2f system.

## 🚀 Quick Start

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Open browser
open http://localhost:3000
```

## 📦 What's Here

| Directory/File | Purpose |
|----------------|---------|
| `app/` | Next.js 14 App Router pages |
| `app/page.tsx` | Main UI with tabs (Encode, Decode, History) |
| `components/` | React components |
| `components/FileUploadForm.tsx` | Drag & drop file upload with settings |
| `components/JobStatus.tsx` | Real-time progress tracking |
| `components/FileHistory.tsx` | File browser with video player |
| `lib/api.ts` | Typed API client for backend |
| `styles/` | CSS and Tailwind configuration |
| `public/` | Static assets |

## 🛠️ Installation

### Prerequisites
- Node.js 16+
- npm or yarn
- Backend running on port 5000

### Setup

```bash
cd frontend

# Install dependencies
npm install

# Set environment variable (optional)
export NEXT_PUBLIC_API_URL="http://localhost:5000"

# Run development server
npm run dev

# Build for production
npm run build
npm start
```

## 🎨 Features

### 1. Encode Tab
- Drag & drop file upload
- Video settings:
  - Resolution (640x480 to 3840x2160)
  - FPS (1 to 60)
  - Chunk size
- Real-time progress bar
- Success notification with download link

### 2. Decode Tab
- Upload encoded MP4 file
- Decode back to original
- Progress tracking
- Download recovered file

### 3. History Tab
- List all processed files
- Video preview/playback
- File size and timestamps
- Download buttons
- Delete functionality

## 🌐 API Integration

The frontend communicates with the backend via `/api` endpoints:

```typescript
// lib/api.ts
const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:5000';

// Encode file
await api.encode(file, { width, height, fps, chunkSize });

// Check status
await api.getStatus(jobId);

// Get file history
await api.getFiles();

// Download file
window.open(api.getDownloadUrl(filename));
```

## 🎨 UI Components

### FileUploadForm

```tsx
<FileUploadForm
  onUpload={handleUpload}
  mode="encode" // or "decode"
/>
```

Features:
- Drag & drop area
- Click to browse
- File size validation
- Settings configuration
- Submit button

### JobStatus

```tsx
<JobStatus jobId={jobId} onComplete={handleComplete} />
```

Features:
- Polls backend for progress
- Shows progress bar (0-100%)
- Handles errors
- Shows completion message

### FileHistory

```tsx
<FileHistory />
```

Features:
- Lists all files from backend
- Video player for MP4 files
- Download buttons
- File metadata display

## 🔧 Configuration

### Environment Variables

```bash
# .env.local
NEXT_PUBLIC_API_URL=http://localhost:5000
```

### Next.js Config

```javascript
// next.config.js
module.exports = {
  reactStrictMode: true,
  // Add additional config
}
```

### Tailwind Config

```javascript
// tailwind.config.ts
export default {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      // Custom theme
    },
  },
}
```

## 🧪 Testing

```bash
# Run type checking
npm run type-check

# Build for production (includes type checking)
npm run build

# Test API connectivity
curl http://localhost:5000/health
```

## 🎨 Styling

Built with **Tailwind CSS** for:
- Responsive design (mobile, tablet, desktop)
- Dark theme
- Modern UI components
- Smooth animations

### Key Classes

```tsx
// Gradient background
className="bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900"

// Card styling
className="bg-white/5 backdrop-blur-sm rounded-xl shadow-xl border border-white/10"

// Button styling
className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg"
```

## 📱 Responsive Design

```tsx
// Mobile-first approach
<div className="
  w-full           // Mobile: full width
  md:w-1/2         // Tablet: half width
  lg:w-1/3         // Desktop: one-third width
">
```

## 🔍 Troubleshooting

### Backend Connection Failed

```bash
# Check backend is running
curl http://localhost:5000/health

# Verify environment variable
echo $NEXT_PUBLIC_API_URL

# Check CORS settings in backend
```

### Build Errors

```bash
# Clear Next.js cache
rm -rf .next

# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install

# Type errors
npm run type-check
```

### Port Already in Use

```bash
# Kill process on port 3000
lsof -i :3000 | grep -v COMMAND | awk '{print $2}' | xargs kill -9

# Or use different port
npm run dev -- -p 3001
```

## 📊 File Structure

```
frontend/
├── app/
│   ├── layout.tsx          # Root layout
│   ├── page.tsx            # Main page (tabs)
│   └── globals.css         # Global styles
├── components/
│   ├── FileUploadForm.tsx  # File upload component
│   ├── JobStatus.tsx       # Progress tracking
│   └── FileHistory.tsx     # File listing
├── lib/
│   └── api.ts              # API client
├── public/
│   └── favicon.ico         # Site icon
├── styles/
│   └── globals.css         # Tailwind imports
├── next.config.js          # Next.js config
├── tailwind.config.ts      # Tailwind config
├── tsconfig.json           # TypeScript config
└── package.json            # Dependencies
```

## 🔐 Security

- ✅ Environment variables for API URL
- ✅ CORS validation
- ✅ File size limits
- ✅ Input sanitization
- ✅ Error handling

## 📚 Dependencies

### Core
- `next` ^14.0.0 - React framework
- `react` ^18.0.0 - UI library
- `react-dom` ^18.0.0 - React renderer

### Styling
- `tailwindcss` ^3.0.0 - Utility CSS
- `postcss` ^8.0.0 - CSS processing
- `autoprefixer` ^10.0.0 - CSS vendor prefixes

### Development
- `typescript` ^5.0.0 - Type checking
- `@types/react` - React type definitions
- `@types/node` - Node type definitions

See [package.json](package.json) for full list.

## 🚀 Production Deployment

### Build

```bash
npm run build
```

### Serve

```bash
# Production server
npm start

# Or with PM2
pm2 start npm --name "f2v2f-frontend" -- start

# Or with Docker
docker build -t f2v2f-frontend .
docker run -p 3000:3000 f2v2f-frontend
```

See [../DEPLOYMENT.md](../DEPLOYMENT.md) for detailed deployment instructions.

## 🔗 Related Documentation

- **Backend API**: [../backend/README.md](../backend/README.md)
- **Rust Library**: [../lib/README.md](../lib/README.md)
- **Complete Guide**: [../COMPLETE_GUIDE.md](../COMPLETE_GUIDE.md)
- **Architecture**: [../ARCHITECTURE.md](../ARCHITECTURE.md)

## 🤝 Contributing

1. Make changes in `components/` or `app/`
2. Test with `npm run dev`
3. Type check with `npm run type-check` (if available, or `npx tsc --noEmit`)
4. Build with `npm run build`
5. Commit and push

---

**Built with Next.js 14**  
**Styled with Tailwind CSS**  
**Powered by React 18**  
**Ready for Production**
