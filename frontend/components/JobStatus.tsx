'use client';

import { useState, useEffect } from 'react';
import { getJobStatus, getDownloadUrl, JobStatus } from '@/lib/api';
import FilePreview from './FilePreview';

interface JobStatusComponentProps {
  jobId: string;
  onComplete: (result: JobStatus) => void;
  onError: (error: string) => void;
  onProgress?: (progress: number, status: string) => void;
  wsData?: JobStatus;
}

export default function JobStatusComponent({
  jobId,
  onComplete,
  onError,
  onProgress,
  wsData,
}: JobStatusComponentProps) {
  const [status, setStatus] = useState<JobStatus | null>(null);

  // Sync with wsData if provided
  useEffect(() => {
    if (wsData) {
      setStatus(wsData);

      if (onProgress && (wsData.status === 'pending' || wsData.status === 'running')) {
        onProgress(wsData.progress, wsData.status);
      }

      if (wsData.status === 'completed') {
        onComplete(wsData);
      } else if (wsData.status === 'failed') {
        onError(wsData.error || 'Job failed');
      }
    }
  }, [wsData, onComplete, onError, onProgress]);

  useEffect(() => {
    // Only poll if wsData is NOT available
    if (wsData) return;

    let mounted = true;
    const interval = setInterval(async () => {
      try {
        const jobStatus = await getJobStatus(jobId);
        if (mounted) {
          setStatus(jobStatus);

          // Call onProgress callback if provided
          if (onProgress && (jobStatus.status === 'pending' || jobStatus.status === 'running')) {
            onProgress(jobStatus.progress, jobStatus.status);
          }

          if (jobStatus.status === 'completed') {
            onComplete(jobStatus);
          } else if (jobStatus.status === 'failed') {
            onError(jobStatus.error || 'Job failed');
          }
        }
      } catch (error) {
        if (mounted) {
          const message = error instanceof Error ? error.message : 'Failed to fetch status';
          onError(message);
        }
      }
    }, 500);

    return () => {
      mounted = false;
      clearInterval(interval);
    };
  }, [jobId, onComplete, onError, onProgress]);

  if (!status) {
    return (
      <div className="text-center py-8">
        <div className="animate-spin text-4xl mb-4">⏳</div>
        <p>Loading job status...</p>
      </div>
    );
  }

  const statusColor = {
    pending: 'text-yellow-400',
    running: 'text-blue-400',
    completed: 'text-green-400',
    failed: 'text-red-400',
  }[status.status];

  const statusEmoji = {
    pending: '⏳',
    running: '🔄',
    completed: '✅',
    failed: '❌',
  }[status.status];

  return (
    <div className="space-y-6">
      {/* Status Card */}
      <div className="bg-slate-800/50 rounded-lg p-6 border border-primary/20">
        <div className="flex items-center gap-3 mb-4">
          <span className="text-3xl">{statusEmoji}</span>
          <div>
            <h3 className="text-lg font-semibold">
              {status.operation === 'encode' ? 'Encoding' : 'Decoding'}
            </h3>
            <p className={`font-medium ${statusColor}`}>
              {status.status.charAt(0).toUpperCase() + status.status.slice(1)}
            </p>
          </div>
        </div>

        {/* Progress Bar */}
        {status.status !== 'completed' && status.status !== 'failed' && (
          <div className="space-y-2">
            <div className="w-full bg-slate-700 rounded-full h-3 overflow-hidden">
              <div
                className="h-full bg-gradient-to-r from-primary to-secondary transition-all duration-300"
                style={{ width: `${status.progress}%` }}
              />
            </div>
            <p className="text-right text-sm text-gray-400">{status.progress}%</p>
          </div>
        )}

        {/* Job ID */}
        <div className="mt-4 pt-4 border-t border-slate-700">
          <p className="text-xs text-gray-500 break-all">Job ID: {jobId}</p>
        </div>
      </div>

      {/* Error Message */}
      {status.status === 'failed' && status.error && (
        <div className="bg-red-900/20 border border-red-500 rounded-lg p-4">
          <p className="text-red-400">
            <strong>Error:</strong> {status.error}
          </p>
        </div>
      )}

      {/* Download Button */}
      {status.status === 'completed' && status.result_url && (
        <div className="space-y-4">
          <FilePreview
            url={getDownloadUrl(status.result_url.split('/').pop() || '')}
            filename={status.status === 'completed' && status.operation === 'encode' ? 'Encoded Video.mp4' : (status.original_filename || 'restored_file')}
          />

          <a
            href={getDownloadUrl(status.result_url.split('/').pop() || '')}
            className="block w-full bg-gradient-to-r from-green-600 to-green-700 text-white font-semibold py-3 rounded-lg hover:opacity-90 transition text-center shadow-lg shadow-green-900/20"
          >
            📥 Download {status.operation === 'encode' ? 'Video' : 'Restored File'}
          </a>
        </div>
      )}
    </div>
  );
}
