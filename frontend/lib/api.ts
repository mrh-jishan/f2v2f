let API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:5000';
// Ensure /api is appended if not already present
if (!API_BASE.endsWith('/api')) {
  API_BASE = API_BASE + '/api';
}

export interface EncodeRequest {
  file: File;
  width?: number;
  height?: number;
  fps?: number;
  chunk_size?: number;
}

export interface DecodeRequest {
  file: File;
}

export interface JobStatus {
  job_id: string;
  operation: 'encode' | 'decode';
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  result_url?: string;
  error?: string;
}

export interface FileRecord {
  id: string;
  name: string;
  type: 'original' | 'encoded';
  size: number;
  created_at: string;
  video_url?: string;
  original_file?: string;
  checksum?: string;
}

// Start encoding job
export async function startEncode(request: EncodeRequest): Promise<{ job_id: string }> {
  const formData = new FormData();
  formData.append('file', request.file);
  if (request.width) formData.append('width', request.width.toString());
  if (request.height) formData.append('height', request.height.toString());
  if (request.fps) formData.append('fps', request.fps.toString());
  if (request.chunk_size) formData.append('chunk_size', request.chunk_size.toString());

  const response = await fetch(`${API_BASE}/encode`, {
    method: 'POST',
    body: formData,
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Encoding failed');
  }

  return response.json();
}

// Start decoding job
export async function startDecode(request: DecodeRequest): Promise<{ job_id: string }> {
  const formData = new FormData();
  formData.append('file', request.file);

  const response = await fetch(`${API_BASE}/decode`, {
    method: 'POST',
    body: formData,
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Decoding failed');
  }

  return response.json();
}

// Get job status
export async function getJobStatus(jobId: string): Promise<JobStatus> {
  const response = await fetch(`${API_BASE}/status/${jobId}`);

  if (!response.ok) {
    throw new Error('Failed to fetch job status');
  }

  return response.json();
}

// Get health check
export async function getHealth(): Promise<{ status: string }> {
  const response = await fetch(`${API_BASE}/health`);
  return response.json();
}

// Get all file records
export async function getFileRecords(): Promise<FileRecord[]> {
  try {
    const response = await fetch(`${API_BASE}/files`);
    if (response.ok) {
      return response.json();
    }
    return [];
  } catch {
    return [];
  }
}

// Delete a file record
export async function deleteFile(fileId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/files/${fileId}`, {
    method: 'DELETE',
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error || 'Failed to delete file');
  }
}

// Download result
export function getDownloadUrl(filename: string): string {
  return `${API_BASE}/download/${filename}`;
}
