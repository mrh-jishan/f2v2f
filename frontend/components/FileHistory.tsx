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
}

export default function FileHistory({ refreshTrigger, selectedFile: propSelectedFile, onFileSelect }: FileHistoryProps) {
  const [files, setFiles] = useState<FileRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [deleting, setDeleting] = useState<string | null>(null);
  const [filter, setFilter] = useState<'all' | 'encoded' | 'original'>('all');
  const selectedFile = propSelectedFile !== undefined ? propSelectedFile : null;

  const handleFileSelect = (file: FileRecord | null) => {
    if (onFileSelect) {
      onFileSelect(file);
    }
  };

  const handleDelete = async (fileId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    if (!confirm('Are you sure you want to delete this file? This cannot be undone.')) return;

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

  const filteredFiles = filter === 'all'
    ? files
    : files.filter(f => f.type === filter);

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center py-20 gap-4">
        <div className="w-10 h-10 border-4 border-primary/20 border-t-primary rounded-full animate-spin"></div>
        <p className="text-slate-500 font-medium">Synchronizing with registry...</p>
      </div>
    );
  }

  if (files.length === 0) {
    return (
      <div className="text-center py-20 bg-white/[0.02] border border-dashed border-white/10 rounded-3xl">
        <p className="text-4xl mb-4">📁</p>
        <p className="text-slate-400 font-medium">Registry is empty</p>
        <p className="text-xs text-slate-600 mt-2">Start by encoding or decoding a file!</p>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      {selectedFile ? (
        // File Detail View
        <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4">
          <div className="flex items-center justify-between">
            <button
              onClick={() => handleFileSelect(null)}
              className="flex items-center gap-2 text-sm font-bold text-slate-400 hover:text-white transition uppercase tracking-widest"
            >
              ← System Overview
            </button>
            <button
              onClick={(e) => handleDelete(selectedFile.id, e)}
              disabled={deleting === selectedFile.id}
              className="px-6 py-2 bg-red-500 hover:bg-red-600 text-white text-xs font-black uppercase tracking-tighter rounded-full shadow-lg shadow-red-500/20 transition-all active:scale-95 disabled:opacity-50"
            >
              {deleting === selectedFile.id ? 'Purging...' : 'Confirm Destruction'}
            </button>
          </div>

          <div className="bg-white/[0.03] rounded-3xl p-8 border border-white/5 shadow-2xl">
            <div className="flex flex-wrap items-center gap-3 mb-6">
              <span className={`px-3 py-1 rounded-full text-[10px] font-black uppercase tracking-widest ${selectedFile.type === 'encoded' ? 'bg-primary/20 text-primary border border-primary/30' : 'bg-secondary/20 text-secondary border border-secondary/30'
                }`}>
                {selectedFile.type === 'encoded' ? 'Encoded Asset' : 'Decoded Payload'}
              </span>
              <h3 className="text-2xl font-bold text-white flex-1">{selectedFile.name}</h3>
            </div>

            {/* Video Player */}
            {selectedFile.video_url && (
              <div className="mb-8 overflow-hidden rounded-2xl border border-white/10 bg-black shadow-inner relative group">
                <video
                  controls
                  playsInline
                  preload="metadata"
                  className="w-full aspect-video"
                  src={selectedFile.video_url}
                >
                  <p className="text-red-400 p-4">
                    Your browser doesn't support HTML5 video.
                  </p>
                </video>
              </div>
            )}

            {/* File Details Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              <div className="p-4 rounded-2xl bg-white/[0.02] border border-white/5">
                <p className="text-[10px] uppercase font-black text-slate-500 mb-1">Metrics</p>
                <p className="text-slate-300 font-mono text-sm leading-relaxed">
                  Size: {(selectedFile.size / 1024 / 1024).toFixed(2)} MB<br />
                  Index: {selectedFile.id.substring(0, 8)}
                </p>
              </div>
              <div className="p-4 rounded-2xl bg-white/[0.02] border border-white/5">
                <p className="text-[10px] uppercase font-black text-slate-500 mb-1">Timestamp</p>
                <p className="text-slate-300 font-mono text-sm leading-relaxed">
                  {new Date(selectedFile.created_at).toLocaleDateString()}<br />
                  {new Date(selectedFile.created_at).toLocaleTimeString()}
                </p>
              </div>
              {selectedFile.checksum && (
                <div className="p-4 rounded-2xl bg-white/[0.02] border border-white/5 col-span-full">
                  <p className="text-[10px] uppercase font-black text-slate-500 mb-1">Integrity Signature (SHA256)</p>
                  <code className="text-[10px] break-all text-slate-400 leading-tight">
                    {selectedFile.checksum}
                  </code>
                </div>
              )}
            </div>
          </div>
        </div>
      ) : (
        // File List View
        <div className="space-y-6">
          <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
            <div className="flex items-center gap-1 bg-white/5 p-1 rounded-xl border border-white/10 shrink-0">
              {(['all', 'encoded', 'original'] as const).map((t) => (
                <button
                  key={t}
                  onClick={() => setFilter(t)}
                  className={`px-4 py-1.5 rounded-lg text-xs font-bold uppercase tracking-tight transition-all ${filter === t ? 'bg-white/10 text-white shadow-lg' : 'text-slate-500 hover:text-slate-300'
                    }`}
                >
                  {t}
                </button>
              ))}
            </div>
            <p className="text-[10px] font-black uppercase tracking-widest text-slate-600">
              Registry Database • {filteredFiles.length} Records
            </p>
          </div>

          <div className="grid gap-3">
            {filteredFiles.map((file) => (
              <div
                key={file.id}
                onClick={() => handleFileSelect(file)}
                className="bg-white/[0.02] hover:bg-white/[0.05] rounded-2xl p-5 border border-white/5 hover:border-white/10 transition-all group cursor-pointer active:scale-[0.99]"
              >
                <div className="flex items-center justify-between gap-4">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <span className={`w-1.5 h-1.5 rounded-full ${file.type === 'encoded' ? 'bg-primary shadow-[0_0_8px_rgba(244,63,94,0.5)]' : 'bg-secondary shadow-[0_0_8px_rgba(168,85,247,0.5)]'}`}></span>
                      <p className="font-bold text-slate-200 truncate pr-2 tracking-tight">{file.name}</p>
                    </div>
                    <div className="flex items-center gap-3 text-[10px] font-black uppercase tracking-widest text-slate-500">
                      <span>{file.type}</span>
                      <span className="w-1 h-1 bg-slate-800 rounded-full"></span>
                      <span>{(file.size / 1024 / 1024).toFixed(2)} MB</span>
                      <span className="hidden sm:inline w-1 h-1 bg-slate-800 rounded-full"></span>
                      <span className="hidden sm:inline">{new Date(file.created_at).toLocaleDateString()}</span>
                    </div>
                  </div>
                  <div className="flex items-center gap-4">
                    <button
                      onClick={(e) => handleDelete(file.id, e)}
                      disabled={deleting === file.id}
                      className="w-10 h-10 flex items-center justify-center bg-red-500/10 hover:bg-red-500 text-red-500 hover:text-white rounded-xl border border-red-500/20 transition-all opacity-0 group-hover:opacity-100 disabled:opacity-50"
                      title="Destroy Record"
                    >
                      {deleting === file.id ? '...' : '🗑️'}
                    </button>
                    <div className="text-slate-600 group-hover:text-primary transition-colors pr-2">
                      →
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
