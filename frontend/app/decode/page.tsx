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

export default function DecodePage() {
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
        title: 'Decoding Complete',
        message: `${activeJob.fileName || 'File'} has been decoded successfully!`,
      });
    }
    setActiveJob(null);
  };

  const handleError = (errorMessage: string) => {
    setError(errorMessage);
    setNotification({
      type: 'error',
      title: 'Decoding Failed',
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
            className="px-4 py-3 font-semibold transition border-b-2 border-transparent text-gray-400 hover:text-gray-300"
          >
            ✨ Encode
          </Link>
          <Link
            href="/decode"
            className="px-4 py-3 font-semibold transition border-b-2 border-primary text-primary"
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
                  <h2 className="text-2xl font-bold">Decoding in Progress</h2>
                  <JobStatus
                    jobId={activeJob.jobId}
                    onComplete={handleJobComplete}
                    onError={handleError}
                    onProgress={(progress, status) => handleJobProgress(activeJob.jobId, progress, status)}
                  />
                </>
              ) : (
                <>
                  <h2 className="text-2xl font-bold">Restore File from Video</h2>
                  <p className="text-gray-400 mb-6">
                    Upload an encoded video to restore the original file. The decoding process verifies
                    data integrity using SHA256 checksums.
                  </p>
                  <FileUploadForm
                    mode="decode"
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
                <p className="font-semibold text-white mb-1">📥 To Decode:</p>
                <p>Upload the encoded MP4 video to restore the original file</p>
              </div>
              <div>
                <p className="font-semibold text-white mb-1">🔒 File Integrity:</p>
                <p>SHA256 checksums ensure your decoded file is byte-perfect</p>
              </div>
              <div className="pt-4 border-t border-slate-700">
                <p className="text-xs font-semibold text-primary mb-2">SUPPORTED FORMATS</p>
                <p className="text-xs">MP4 videos created by f2v2f encoder</p>
              </div>
              <div className="pt-4 border-t border-slate-700">
                <p className="text-xs font-semibold text-primary mb-2">FILE SIZE LIMIT</p>
                <p className="text-xs">Up to 5GB per video</p>
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
