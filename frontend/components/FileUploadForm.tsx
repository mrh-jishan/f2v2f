'use client';

import { useState, useCallback } from 'react';
import { startEncode, startDecode, EncodeRequest, DecodeRequest } from '@/lib/api';

interface FileUploadFormProps {
  mode: 'encode' | 'decode';
  onJobStart: (jobId: string, fileName?: string) => void;
  onError: (error: string) => void;
}

const DEFAULT_ENCODE_CONFIG = {
  width: 1920,
  height: 1080,
  fps: 30,
  chunk_size: 4096,
  use_compression: true,
  compression_level: 3,
};

export default function FileUploadForm({ mode, onJobStart, onError }: FileUploadFormProps) {
  const [file, setFile] = useState<File | null>(null);
  const [loading, setLoading] = useState(false);
  const [config, setConfig] = useState(DEFAULT_ENCODE_CONFIG);

  const handleFileChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0];
    if (selectedFile) {
      setFile(selectedFile);
    }
  }, []);

  const handleConfigChange = (key: string, value: any) => {
    setConfig((prev) => ({
      ...prev,
      [key]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!file) {
      onError('Please select a file');
      return;
    }

    setLoading(true);

    try {
      if (mode === 'encode') {
        const request: EncodeRequest = {
          file,
          width: config.width,
          height: config.height,
          fps: config.fps,
          chunk_size: config.chunk_size,
          use_compression: config.use_compression,
          compression_level: config.compression_level,
        };
        const { job_id } = await startEncode(request);
        onJobStart(job_id, file.name);
      } else {
        const request: DecodeRequest = { file };
        const { job_id } = await startDecode(request);
        onJobStart(job_id, file.name);
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Upload failed';
      onError(message);
    } finally {
      setLoading(false);
      setFile(null);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* File Input */}
      <div className="border-2 border-dashed border-primary/30 rounded-lg p-8 hover:border-primary/60 transition">
        <label className="block">
          <input
            type="file"
            onChange={handleFileChange}
            disabled={loading}
            className="w-full"
            accept={mode === 'encode' ? '*' : 'video/mp4'}
          />
          <div className="mt-3 text-sm text-gray-400">
            {file ? (
              <div>
                <p className="font-semibold text-primary">{file.name}</p>
                <p>{(file.size / 1024 / 1024).toFixed(2)} MB</p>
              </div>
            ) : (
              <p>
                {mode === 'encode'
                  ? 'Click to select file to encode'
                  : 'Click to select video to decode'}
              </p>
            )}
          </div>
        </label>
      </div>

      {/* Configuration (only for encode) */}
      {mode === 'encode' && (
        <div className="space-y-4 bg-slate-800/50 p-4 rounded-lg">
          <h3 className="text-lg font-semibold text-primary">Encoding Settings</h3>

          <div className="grid grid-cols-2 gap-4">
            {/* Resolution */}
            <div>
              <label className="block text-sm mb-2">Resolution</label>
              <select
                className="w-full bg-slate-700 rounded px-3 py-2"
                onChange={(e) => {
                  const [w, h] = e.target.value.split('x').map(Number);
                  setConfig((prev) => ({ ...prev, width: w, height: h }));
                }}
                value={`${config.width}x${config.height}`}
              >
                <option value="1280x720">1280x720 (HD)</option>
                <option value="1920x1080">1920x1080 (Full HD)</option>
                <option value="2560x1440">2560x1440 (2K)</option>
                <option value="3840x2160">3840x2160 (4K)</option>
              </select>
            </div>

            {/* FPS */}
            <div>
              <label className="block text-sm mb-2">FPS</label>
              <select
                className="w-full bg-slate-700 rounded px-3 py-2"
                value={config.fps}
                onChange={(e) => handleConfigChange('fps', parseInt(e.target.value))}
              >
                <option value={24}>24 FPS</option>
                <option value={30}>30 FPS</option>
                <option value={60}>60 FPS</option>
              </select>
            </div>
          </div>

          {/* Chunk Size */}
          <div>
            <label className="block text-sm mb-2">
              Chunk Size: {(config.chunk_size / 1024).toFixed(0)} KB
            </label>
            <input
              type="range"
              min="1024"
              max="10485760"
              value={config.chunk_size}
              onChange={(e) => handleConfigChange('chunk_size', parseInt(e.target.value))}
              className="w-full"
            />
            <div className="text-xs text-gray-400 mt-1">
              Smaller = more frames = slower but clearer (1KB-10MB)
            </div>
          </div>

          {/* Compression Support */}
          <div className="pt-4 border-t border-slate-700 space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <label className="block font-semibold text-primary">High-Speed Compression</label>
                <p className="text-xs text-gray-400">Reduce output size significantly before encoding</p>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  className="sr-only peer"
                  checked={config.use_compression}
                  onChange={(e) => handleConfigChange('use_compression', e.target.checked)}
                />
                <div className="w-11 h-6 bg-slate-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-primary"></div>
              </label>
            </div>

            {config.use_compression && (
              <div className="animate-in fade-in slide-in-from-top-1">
                <label className="block text-sm mb-2">
                  Compression Level: {config.compression_level}
                  <span className="ml-2 text-xs text-gray-500">(1-22, higher is slower but smaller)</span>
                </label>
                <input
                  type="range"
                  min="1"
                  max="22"
                  value={config.compression_level}
                  onChange={(e) => handleConfigChange('compression_level', parseInt(e.target.value))}
                  className="w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-primary"
                />
              </div>
            )}
          </div>
        </div>
      )}

      {/* Submit Button */}
      <button
        type="submit"
        disabled={!file || loading}
        className="w-full bg-gradient-to-r from-primary to-secondary text-white font-semibold py-3 rounded-lg hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed transition"
      >
        {loading ? (
          <span className="flex items-center justify-center gap-2">
            <span className="animate-spin">⚙️</span>
            {mode === 'encode' ? 'Encoding...' : 'Decoding...'}
          </span>
        ) : (
          <span>{mode === 'encode' ? 'Encode to Video' : 'Decode from Video'}</span>
        )}
      </button>
    </form>
  );
}
