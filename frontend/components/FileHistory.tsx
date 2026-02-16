'use client';

import { useState, useEffect } from 'react';
import { getFileRecords, deleteFile, FileRecord } from '@/lib/api';

export interface JobProgress {
  jobId: string;
  fileName: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  operation: 'encode' | 'decode';
}

interface FileHistoryProps {
  refreshTrigger?: number;
  selectedFile?: FileRecord | null;
  onFileSelect?: (file: FileRecord | null) => void;
  activeJobs?: JobProgress[];
}

export default function FileHistory({ refreshTrigger, selectedFile: propSelectedFile, onFileSelect, activeJobs = [] }: FileHistoryProps) {
  const [files, setFiles] = useState<FileRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [deleting, setDeleting] = useState<string | null>(null);
  const selectedFile = propSelectedFile !== undefined ? propSelectedFile : null;

  const handleFileSelect = (file: FileRecord | null) => {
    if (onFileSelect) {
      onFileSelect(file);
    }
  };

  const handleDelete = async (fileId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    if (!confirm('Are you sure you want to delete this file?')) return;
    
    setDeleting(fileId);
    try {
      await deleteFile(fileId);
      setFiles(files.filter(f => f.id !== fileId));
      if (selectedFile?.id === fileId) {
        handleFileSelect(null);
      }
    } catch (error) {
      console.error('Failed to delete file:', error);
      alert('Failed to delete file');
    } finally {
      setDeleting(null);
    }
  };

  useEffect(() => {
    const fetchFiles = async () => {
      setLoading(true);
      try {
        const records = await getFileRecords();
        setFiles(records);
      } catch (error) {
        console.error('Failed to fetch files:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchFiles();
  }, [refreshTrigger]);

  if (loading) {
    return <div className="text-center py-8">Loading file history...</div>;
  }

  if (files.length === 0) {
    return (
      <div className="text-center py-12 text-gray-400">
        <p className="text-2xl mb-2">📁</p>
        <p>No files yet. Start by encoding or decoding a file!</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {selectedFile ? (
        // File Detail View
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <button
              onClick={() => handleFileSelect(null)}
              className="flex items-center gap-2 text-primary hover:text-secondary transition"
            >
              ← Back to Files
            </button>
            <button
              onClick={(e) => handleDelete(selectedFile.id, e)}
              disabled={deleting === selectedFile.id}
              className="px-4 py-2 bg-red-600/20 hover:bg-red-600/30 text-red-400 rounded-lg border border-red-600/30 transition disabled:opacity-50"
            >
              {deleting === selectedFile.id ? 'Deleting...' : '🗑️ Delete'}
            </button>
          </div>

          <div className="bg-slate-800/50 rounded-lg p-6 border border-primary/20">
            <h3 className="text-xl font-semibold mb-4">{selectedFile.name}</h3>

            {/* Video Player */}
            {selectedFile.video_url && (
              <div className="mb-6">
                <video
                  controls
                  playsInline
                  preload="metadata"
                  className="w-full rounded-lg bg-black"
                  src={selectedFile.video_url}
                  controlsList="nodownload"
                >
                  <p className="text-red-400 p-4">
                    Your browser doesn't support HTML5 video. 
                    <a href={selectedFile.video_url} className="underline ml-2">Download the video</a> instead.
                  </p>
                </video>
                <p className="text-xs text-gray-500 mt-2">
                  💡 Tip: If video doesn't play, try downloading and playing in VLC media player
                </p>
              </div>
            )}

            {/* File Details */}
            <div className="space-y-2 text-sm text-gray-400">
              <p>
                <strong>Type:</strong> {selectedFile.type === 'encoded' ? 'Encoded Video' : 'Original File'}
              </p>
              <p>
                <strong>Size:</strong> {(selectedFile.size / 1024 / 1024).toFixed(2)} MB
              </p>
              <p>
                <strong>Created:</strong> {new Date(selectedFile.created_at).toLocaleDateString()}
              </p>
              {selectedFile.checksum && (
                <p>
                  <strong>SHA256:</strong>
                  <code className="block text-xs break-all text-gray-500 mt-1">
                    {selectedFile.checksum}
                  </code>
                </p>
              )}
            </div>
          </div>
        </div>
      ) : (
        // File List View
        <div className="grid gap-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold">File History</h3>
            {activeJobs.length > 0 && (
              <span className="text-sm text-gray-400">
                {activeJobs.length} job{activeJobs.length > 1 ? 's' : ''} in progress
              </span>
            )}
          </div>
          
          {/* Active Jobs */}
          {activeJobs.map((job) => (
            <div
              key={job.jobId}
              className="bg-slate-800/50 rounded-lg p-4 border border-primary/30 animate-pulse"
            >
              <div className="flex items-start justify-between gap-4">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <p className="font-semibold truncate">{job.fileName}</p>
                    <span className="px-2 py-0.5 bg-blue-600/20 text-blue-400 border border-blue-600/30 rounded text-xs font-semibold">
                      {job.status === 'pending' ? '⏳ Pending' : '⚙️ Processing'}
                    </span>
                  </div>
                  <p className="text-sm text-gray-400">
                    {job.operation === 'encode' ? '🎬 Encoding' : '📄 Decoding'} • {job.progress}%
                  </p>
                  <div className="mt-2 w-full bg-slate-700 rounded-full h-1.5">
                    <div
                      className="bg-primary h-1.5 rounded-full transition-all duration-300"
                      style={{ width: `${job.progress}%` }}
                    />
                  </div>
                </div>
              </div>
            </div>
          ))}
          
          {/* Completed Files */}
          {files.map((file) => (
            <div
              key={file.id}
              className="bg-slate-800/50 hover:bg-slate-700/50 rounded-lg p-4 border border-primary/20 hover:border-primary/40 transition group"
            >
              <div className="flex items-start justify-between gap-4">
                <button
                  onClick={() => handleFileSelect(file)}
                  className="flex-1 text-left"
                >
                  <p className="font-semibold truncate">{file.name}</p>
                  <p className="text-sm text-gray-400">
                    {file.type === 'encoded' ? '🎬 Video' : '📄 Original'} •{' '}
                    {(file.size / 1024 / 1024).toFixed(2)} MB
                  </p>
                  <p className="text-xs text-gray-500 mt-1">
                    {new Date(file.created_at).toLocaleDateString()} at{' '}
                    {new Date(file.created_at).toLocaleTimeString()}
                  </p>
                </button>
                <div className="flex items-center gap-2">
                  <button
                    onClick={(e) => handleDelete(file.id, e)}
                    disabled={deleting === file.id}
                    className="px-3 py-1 text-sm bg-red-600/20 hover:bg-red-600/30 text-red-400 rounded border border-red-600/30 transition opacity-0 group-hover:opacity-100 disabled:opacity-50"
                  >
                    {deleting === file.id ? '...' : '🗑️'}
                  </button>
                  <button
                    onClick={() => handleFileSelect(file)}
                    className="text-2xl"
                  >
                    →
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
