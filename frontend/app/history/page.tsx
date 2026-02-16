'use client';

import { useState } from 'react';
import Link from 'next/link';
import FileHistory, { JobProgress } from '@/components/FileHistory';
import { FileRecord } from '@/lib/api';

export default function HistoryPage() {
  const [selectedFile, setSelectedFile] = useState<FileRecord | null>(null);
  const [historyRefresh] = useState(0);
  const [activeJobs] = useState<JobProgress[]>([]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
      {/* Header */}
      <header className="bg-slate-900/50 backdrop-blur border-b border-primary/10 sticky top-0 z-50">
        <div className="max-w-6xl mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <Link href="/" className="text-3xl font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
              🎬 f2v2f
            </Link>
            <p className="text-gray-400">File to Video to File Converter</p>
          </div>
        </div>
      </header>

      <main className="max-w-6xl mx-auto px-4 py-8">
        {/* Tab Navigation */}
        <div className="flex gap-4 mb-8 border-b border-primary/20">
          <Link
            href="/encode"
            className="px-4 py-3 font-semibold transition border-b-2 border-transparent text-gray-400 hover:text-gray-300"
          >
            ✨ Encode
          </Link>
          <Link
            href="/decode"
            className="px-4 py-3 font-semibold transition border-b-2 border-transparent text-gray-400 hover:text-gray-300"
          >
            🎥 Decode
          </Link>
          <Link
            href="/history"
            className="px-4 py-3 font-semibold transition border-b-2 border-primary text-primary"
          >
            📁 History
          </Link>
        </div>

        {/* Content Area */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="lg:col-span-2">
            <div className="space-y-6">
              <h2 className="text-2xl font-bold">Previous Files & Videos</h2>
              <p className="text-gray-400 mb-6">
                Browse and watch your previously encoded or decoded files. Click on any file to view
                details and download.
              </p>
              <FileHistory
                refreshTrigger={historyRefresh}
                selectedFile={selectedFile}
                onFileSelect={setSelectedFile}
                activeJobs={activeJobs}
              />
            </div>
          </div>

          {/* Sidebar */}
          <div className="bg-slate-800/50 rounded-lg p-6 border border-primary/20 h-fit">
            <h3 className="text-lg font-semibold mb-4">ℹ️ File Management</h3>
            <div className="space-y-4 text-sm text-gray-400">
              <div>
                <p className="font-semibold text-white mb-1">📂 View Files:</p>
                <p>Click any file to view details, play videos, or download</p>
              </div>
              <div>
                <p className="font-semibold text-white mb-1">🗑️ Delete Files:</p>
                <p>Hover over files to see delete button, or use delete in detail view</p>
              </div>
              <div>
                <p className="font-semibold text-white mb-1">🎬 Video Playback:</p>
                <p>Videos play directly in browser. If issues, download for VLC</p>
              </div>
              <div className="pt-4 border-t border-slate-700">
                <p className="text-xs font-semibold text-primary mb-2">STORAGE</p>
                <p className="text-xs">Files stored locally on server</p>
              </div>
              <div className="pt-4 border-t border-slate-700">
                <p className="text-xs font-semibold text-primary mb-2">FILE TYPES</p>
                <p className="text-xs">🎬 Encoded videos • 📄 Decoded originals</p>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-slate-900/50 border-t border-primary/10 mt-12">
        <div className="max-w-6xl mx-auto px-4 py-6 text-center text-gray-500 text-sm">
          <p>f2v2f © 2026 • Novel File Encoding System</p>
        </div>
      </footer>
    </div>
  );
}
