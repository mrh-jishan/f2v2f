'use client';

import { useState } from 'react';
import Link from 'next/link';
import FileHistory from '@/components/FileHistory';
import { FileRecord } from '@/lib/api';
import JobQueue from '@/components/JobQueue';
import { useWebSocket } from '@/lib/useWebSocket';

export default function HistoryPage() {
  const [selectedFile, setSelectedFile] = useState<FileRecord | null>(null);
  const { activeJobs, removeJob } = useWebSocket();

  return (
    <div className="min-h-screen bg-slate-950 text-slate-200">
      {/* Background Effects */}
      <div className="fixed inset-0 pointer-events-none overflow-hidden z-0">
        <div className="absolute top-[20%] right-[-5%] w-[30%] h-[30%] bg-primary/5 blur-[100px] rounded-full"></div>
        <div className="absolute bottom-[20%] left-[-5%] w-[30%] h-[30%] bg-secondary/5 blur-[100px] rounded-full"></div>
      </div>

      {/* Header */}
      <header className="border-b border-white/5 bg-slate-950/50 backdrop-blur-md sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
          <Link href="/" className="flex items-center gap-2 group">
            <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-secondary flex items-center justify-center text-white font-bold shadow-lg shadow-primary/20 group-hover:scale-110 transition-transform">
              f
            </div>
            <span className="text-xl font-bold tracking-tight bg-gradient-to-r from-white to-white/60 bg-clip-text text-transparent">
              f2v2f
            </span>
          </Link>
          <nav className="hidden md:flex items-center gap-1 bg-white/5 p-1 rounded-full border border-white/5">
            <Link href="/encode" className="px-5 py-1.5 rounded-full text-sm font-medium transition text-white/50 hover:text-white">✨ Encode</Link>
            <Link href="/decode" className="px-5 py-1.5 rounded-full text-sm font-medium transition text-white/50 hover:text-white">🎥 Decode</Link>
            <Link href="/history" className="px-5 py-1.5 rounded-full text-sm font-semibold transition bg-white/10 text-white shadow-sm ring-1 ring-white/10">📁 History</Link>
          </nav>
          <div className="w-8 h-8"></div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-6 py-12 relative z-10">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-12">
          <div className="lg:col-span-3">
            <div className="space-y-12">
              <section>
                <h1 className="text-4xl font-extrabold tracking-tight text-white mb-4">
                  Registry <span className="text-slate-500">Archive</span>
                </h1>
                <p className="text-slate-400 max-w-2xl text-lg leading-relaxed">
                  Browse the complete history of your file conversions. View metadata,
                  watch encoded videos, and download restored payloads.
                </p>
              </section>

              <FileHistory
                selectedFile={selectedFile}
                onFileSelect={setSelectedFile}
              />
            </div>
          </div>

          {/* Sidebar - Process Queue */}
          <div className="space-y-8">
            <h2 className="text-xs uppercase font-black tracking-[0.2em] text-slate-500 flex items-center justify-between">
              Live Pipeline
              <span className="bg-slate-800 text-slate-400 px-2 py-0.5 rounded text-[10px]">{activeJobs.length}</span>
            </h2>
            <JobQueue
              jobs={activeJobs}
              onRemove={removeJob}
            />

            {/* Legend */}
            <div className="space-y-4">
              <h3 className="text-[10px] font-black uppercase tracking-widest text-slate-500 px-1">Registry Legend</h3>
              <div className="space-y-2">
                <div className="flex items-center gap-3 p-3 rounded-xl bg-white/[0.02] border border-white/5">
                  <span className="w-2 h-2 rounded-full bg-primary"></span>
                  <span className="text-xs font-bold text-slate-400">Encoded MP4 Asset</span>
                </div>
                <div className="flex items-center gap-3 p-3 rounded-xl bg-white/[0.02] border border-white/5">
                  <span className="w-2 h-2 rounded-full bg-secondary"></span>
                  <span className="text-xs font-bold text-slate-400">Decoded Original Payload</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="max-w-7xl mx-auto px-6 py-12 border-t border-white/5 flex flex-col md:flex-row items-center justify-between gap-6 opacity-30">
        <p className="text-sm font-medium">f2v2f System Registry v2.1</p>
        <p className="text-xs text-slate-500">Optimizing Data Persistence</p>
      </footer>
    </div>
  );
}
