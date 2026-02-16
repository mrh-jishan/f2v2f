'use client';

import { useState, useEffect } from 'react';
import { getJobStatus, getDownloadUrl, JobStatus } from '@/lib/api';
import { useWebSocket } from '@/lib/useWebSocket';

interface JobStatusComponentProps {
  jobId: string;
  onComplete: (result: JobStatus) => void;
  onError: (error: string) => void;
  onProgress?: (progress: number, status: string) => void;
}

export default function JobStatusComponent({
  jobId,
  onComplete,
  onError,
  onProgress,
}: JobStatusComponentProps) {
  const [status, setStatus] = useState<JobStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const { activeJobs } = useWebSocket();

  // Get initial status via API
  useEffect(() => {
    let mounted = true;
    
    const fetchInitial = async () => {
      try {
        const jobStatus = await getJobStatus(jobId);
        if (mounted) {
          setStatus(jobStatus);
          setLoading(false);
        }
      } catch (error) {
        if (mounted) {
          const message = error instanceof Error ? error.message : 'Failed to fetch status';
          onError(message);
          setLoading(false);
        }
      }
    };

    fetchInitial();

    return () => {
      mounted = false;
    };
  }, [jobId, onError]);

  // Listen to WebSocket updates for this job
  useEffect(() => {
    const job = activeJobs.find(j => j.job_id === jobId);
    if (job) {
      setStatus(job);
      setLoading(false);
      
      // Call onProgress callback if provided
      if (onProgress && (job.status === 'pending' || job.status === 'running')) {
        onProgress(job.progress, job.status);
      }

      if (job.status === 'completed') {
        onComplete(job);
      } else if (job.status === 'failed') {
        onError(job.error || 'Job failed');
      }
    }
  }, [activeJobs, jobId, onComplete, onError, onProgress]);

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
        <a
          href={getDownloadUrl(status.result_url.split('/').pop() || '')}
          className="block w-full bg-gradient-to-r from-green-600 to-green-700 text-white font-semibold py-3 rounded-lg hover:opacity-90 transition text-center"
        >
          📥 Download Result
        </a>
      )}
    </div>
  );
}
