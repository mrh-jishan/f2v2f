'use client';

import { useState } from 'react';
import Link from 'next/link';
import FileUploadForm from '@/components/FileUploadForm';
import JobStatus from '@/components/JobStatus';
import StatusModal from '@/components/StatusModal';

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
  const [activeJob, setActiveJob] = useState<ActiveJob | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [notification, setNotification] = useState<Notification | null>(null);

  const handleJobStart = (jobId: string, fileName?: string) => {
    setActiveJob({ jobId, fileName });
    setError(null);
  };

  const handleJobProgress = (_jobId: string, _progress: number, _status: string) => {
    // Progress tracking handled by JobStatus component
  };

  const handleJobComplete = () => {
    if (activeJob) {
      setNotification({
        type: 'success',
        title: 'Encoding Complete',
        message: `${activeJob.fileName || 'File'} has been encoded successfully!`,
      });
    }
    setActiveJob(null);
  };

  const handleError = (errorMessage: string) => {
    setError(errorMessage);
    setNotification({
      type: 'error',
      title: 'Encoding Failed',
      message: errorMessage,
    });
    setActiveJob(null);
  };

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
            className="px-4 py-3 font-semibold transition border-b-2 border-primary text-primary"
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
            className="px-4 py-3 font-semibold transition border-b-2 border-transparent text-gray-400 hover:text-gray-300"
          >
            📁 History
          </Link>
        </div>

        {/* Error Message */}
        {error && (
          <div className="mb-6 bg-red-900/20 border border-red-500 rounded-lg p-4 flex justify-between items-center">
            <p className="text-red-400">
              <strong>Error:</strong> {error}
            </p>
            <button
              onClick={() => setError(null)}
              className="text-red-400 hover:text-red-300 transition"
            >
              ✕
            </button>
          </div>
        )}

        {/* Content Area */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="lg:col-span-2">
            <div className="space-y-6">
              {activeJob ? (
                <>
                  <h2 className="text-2xl font-bold">Encoding in Progress</h2>
                  <JobStatus
                    jobId={activeJob.jobId}
                    onComplete={handleJobComplete}
                    onError={handleError}
                    onProgress={(progress, status) => handleJobProgress(activeJob.jobId, progress, status)}
                  />
                </>
              ) : (
                <>
                  <h2 className="text-2xl font-bold">Convert File to Video</h2>
                  <p className="text-gray-400 mb-6">
                    Transform any file into a beautiful artistic video. Each chunk of data is
                    converted into a unique geometric pattern with vibrant colors.
                  </p>
                  <FileUploadForm
                    mode="encode"
                    onJobStart={(jobId, fileName) => handleJobStart(jobId, fileName)}
                    onError={handleError}
                  />
                </>
              )}
            </div>
          </div>

          {/* Sidebar */}
          <div className="bg-slate-800/50 rounded-lg p-6 border border-primary/20 h-fit">
            <h3 className="text-lg font-semibold mb-4">ℹ️ Quick Start</h3>
            <div className="space-y-4 text-sm text-gray-400">
              <div>
                <p className="font-semibold text-white mb-1">📤 To Encode:</p>
                <p>Select any file, configure resolution/FPS, and encode to MP4</p>
              </div>
              <div>
                <p className="font-semibold text-white mb-1">🎨 How It Works:</p>
                <p>Files are split into chunks and converted to unique geometric art patterns</p>
              </div>
              <div className="pt-4 border-t border-slate-700">
                <p className="text-xs font-semibold text-primary mb-2">SUPPORTED FORMATS</p>
                <p className="text-xs">Any file type</p>
              </div>
              <div className="pt-4 border-t border-slate-700">
                <p className="text-xs font-semibold text-primary mb-2">FILE SIZE LIMIT</p>
                <p className="text-xs">Up to 5GB per file</p>
              </div>
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
      <footer className="bg-slate-900/50 border-t border-primary/10 mt-12">
        <div className="max-w-6xl mx-auto px-4 py-6 text-center text-gray-500 text-sm">
          <p>f2v2f © 2026 • Novel File Encoding System</p>
        </div>
      </footer>
    </div>
  );
}
