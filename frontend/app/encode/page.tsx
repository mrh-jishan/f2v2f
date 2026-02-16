'use client';

import { useState } from 'react';
import Link from 'next/link';
import FileUploadForm from '@/components/FileUploadForm';
import JobStatus from '@/components/JobStatus';
import StatusModal from '@/components/StatusModal';
import JobQueue from '@/components/JobQueue';
import { useWebSocket } from '@/lib/useWebSocket';

interface ActiveJob {
  jobId: string;
  fileName?: string;
}

interface Notification {
  type: 'success' | 'error' | 'info' | 'warning';
  title: string;
  message: string;
}

export default function EncodePage() {
  const [currentJob, setCurrentJob] = useState<ActiveJob | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [notification, setNotification] = useState<Notification | null>(null);
  const { activeJobs, removeJob } = useWebSocket();

  const handleJobStart = (jobId: string, fileName?: string) => {
    setCurrentJob({ jobId, fileName });
    setError(null);
  };

  const handleJobComplete = () => {
    if (currentJob) {
      setNotification({
        type: 'success',
        title: 'Encoding Complete',
        message: `${currentJob.fileName || 'File'} has been encoded successfully!`,
      });
    }
    setCurrentJob(null);
  };

  const handleError = (errorMessage: string) => {
    setError(errorMessage);
    setNotification({
      type: 'error',
      title: 'Encoding Failed',
      message: errorMessage,
    });
    setCurrentJob(null);
  };

  return (
    <div className="min-h-screen bg-slate-950 text-slate-200 selection:bg-primary/30">
      {/* Dynamic Background Effects */}
      <div className="fixed inset-0 pointer-events-none overflow-hidden z-0">
        <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] bg-primary/5 blur-[120px] rounded-full animate-pulse-slow"></div>
        <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-secondary/5 blur-[120px] rounded-full animate-pulse-slow animation-delay-2000"></div>
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
            <Link href="/encode" className="px-5 py-1.5 rounded-full text-sm font-semibold transition bg-white/10 text-white shadow-sm ring-1 ring-white/10">✨ Encode</Link>
            <Link href="/decode" className="px-5 py-1.5 rounded-full text-sm font-medium transition text-white/50 hover:text-white">🎥 Decode</Link>
            <Link href="/history" className="px-5 py-1.5 rounded-full text-sm font-medium transition text-white/50 hover:text-white">📁 History</Link>
          </nav>
          <div className="flex items-center gap-3">
            {activeJobs.filter(j => j.status === 'running').length > 0 && (
              <div className="hidden lg:flex items-center gap-2 px-3 py-1 bg-primary/10 border border-primary/20 rounded-full text-xs font-bold text-primary animate-pulse">
                <span className="w-1.5 h-1.5 bg-primary rounded-full"></span>
                LIVE TRACKING
              </div>
            )}
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-6 py-12 relative z-10">
        {/* Error Message */}
        {error && (
          <div className="mb-8 bg-red-500/10 border border-red-500/20 rounded-2xl p-4 flex justify-between items-center animate-in fade-in slide-in-from-top-4">
            <div className="flex items-center gap-3">
              <div className="w-8 h-8 rounded-full bg-red-500/20 flex items-center justify-center text-red-500 font-bold">!</div>
              <p className="text-red-400 text-sm font-medium">
                <strong>Error:</strong> {error}
              </p>
            </div>
            <button onClick={() => setError(null)} className="text-red-500/50 hover:text-red-500 transition px-2">✕</button>
          </div>
        )}

        {/* Content Area */}
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-12">
          <div className="lg:col-span-3">
            <div className="space-y-12">
              <section>
                <h1 className="text-4xl lg:text-5xl font-extrabold tracking-tight text-white mb-4">
                  Transform Data into <span className="text-primary">Art</span>
                </h1>
                <p className="text-slate-400 max-w-2xl text-lg leading-relaxed">
                  Our advanced engine converts any file into an artistic geometric video.
                  High redundancy, perfect recovery, and stunning visuals.
                </p>
              </section>

              <div className="bg-white/[0.02] border border-white/5 rounded-3xl p-8 lg:p-10 shadow-2xl relative overflow-hidden group hover:border-white/10 transition-colors">
                <div className="absolute inset-0 bg-gradient-to-br from-primary/5 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"></div>

                {currentJob ? (
                  <div className="relative z-10 animate-in fade-in zoom-in-95 duration-500">
                    <div className="flex items-center justify-between mb-8">
                      <h2 className="text-2xl font-bold text-white">Encoding in Progress</h2>
                      <button
                        onClick={() => setCurrentJob(null)}
                        className="text-xs font-bold text-slate-500 hover:text-white uppercase tracking-widest transition"
                      >
                        Cancel View
                      </button>
                    </div>
                    <JobStatus
                      jobId={currentJob.jobId}
                      wsData={activeJobs.find(j => j.job_id === currentJob.jobId)}
                      onComplete={handleJobComplete}
                      onError={handleError}
                    />
                  </div>
                ) : (
                  <div className="relative z-10">
                    <FileUploadForm
                      mode="encode"
                      onJobStart={(jobId, fileName) => handleJobStart(jobId, fileName)}
                      onError={handleError}
                    />
                  </div>
                )}
              </div>
            </div>
          </div>

          {/* Sidebar - Process Queue */}
          <div className="space-y-8">
            <h2 className="text-xs uppercase font-black tracking-[0.2em] text-slate-500 flex items-center justify-between">
              Process Queue
              <span className="bg-slate-800 text-slate-400 px-2 py-0.5 rounded text-[10px]">{activeJobs.length}</span>
            </h2>
            <JobQueue
              jobs={activeJobs}
              onRemove={removeJob}
            />

            {/* Quick Info */}
            <div className="bg-gradient-to-br from-primary/10 to-transparent p-6 rounded-2xl border border-primary/10">
              <h3 className="text-sm font-bold text-white mb-3 flex items-center gap-2">
                <span className="w-1.5 h-1.5 bg-primary rounded-full"></span>
                System Intel
              </h3>
              <ul className="space-y-3 text-xs text-slate-400">
                <li className="flex justify-between">
                  <span>Engine:</span>
                  <span className="text-slate-300 font-mono">f2v2f-go/v2.1</span>
                </li>
                <li className="flex justify-between">
                  <span>Redundancy:</span>
                  <span className="text-slate-300 font-mono">Dynamic</span>
                </li>
                <li className="flex justify-between">
                  <span>Compression:</span>
                  <span className="text-slate-300 font-mono">Zstd Multithreaded</span>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </main>

      {/* Status Modal */}
      {notification && (
        <StatusModal
          show={true}
          type={notification.type}
          title={notification.title}
          message={notification.message}
          onClose={() => setNotification(null)}
        />
      )}

      {/* Footer */}
      <footer className="max-w-7xl mx-auto px-6 py-12 border-t border-white/5 flex flex-col md:flex-row items-center justify-between gap-6 opacity-50">
        <div className="flex items-center gap-4 text-xs font-medium uppercase tracking-widest text-slate-500">
          <span>Privacy</span>
          <span>Security</span>
          <span>Protocols</span>
        </div>
        <p className="text-sm text-slate-500 font-medium">f2v2f © 2026 • Optimized for High Concurrency</p>
      </footer>
    </div>
  );
}
