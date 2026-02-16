'use client';

import { JobStatus, getDownloadUrl } from '@/lib/api';

interface JobQueueProps {
    jobs: JobStatus[];
    onRemove: (jobId: string) => void;
}

export default function JobQueue({ jobs, onRemove }: JobQueueProps) {
    const activeJobs = jobs.filter(j => j.status === 'pending' || j.status === 'running');
    const completedJobs = jobs.filter(j => j.status === 'completed' || j.status === 'failed');

    if (jobs.length === 0) {
        return (
            <div className="bg-slate-800/50 rounded-lg p-6 border border-slate-700/50 text-center h-fit">
                <p className="text-gray-500 text-sm italic">No active processes</p>
            </div>
        );
    }

    return (
        <div className="space-y-6">
            {activeJobs.length > 0 && (
                <div className="bg-slate-800/50 rounded-lg p-4 border border-blue-500/30 shadow-lg shadow-blue-500/10">
                    <h3 className="text-sm font-bold text-blue-400 mb-4 px-1 uppercase tracking-wider flex items-center gap-2">
                        <span className="animate-pulse w-2 h-2 bg-blue-500 rounded-full"></span>
                        Active Queue
                    </h3>
                    <div className="space-y-3">
                        {activeJobs.map((job) => (
                            <div key={job.job_id} className="bg-slate-900/60 rounded-md p-3 border border-slate-700/50 shadow-inner">
                                <div className="flex justify-between items-start mb-2">
                                    <div className="flex-1 min-w-0">
                                        <p className="text-sm font-semibold text-gray-200 truncate">{job.original_filename || 'Unknown file'}</p>
                                        <p className="text-[10px] text-gray-500 uppercase font-bold mt-0.5">
                                            {job.operation === 'encode' ? '🎬 Encoding' : '📄 Decoding'}
                                        </p>
                                    </div>
                                    <span className="text-[10px] font-mono text-blue-400 bg-blue-400/10 px-1.5 py-0.5 rounded leading-none border border-blue-400/20">
                                        {job.progress}%
                                    </span>
                                </div>
                                <div className="w-full bg-slate-800 rounded-full h-1.5 overflow-hidden">
                                    <div
                                        className="h-full bg-gradient-to-r from-blue-500 to-cyan-400 transition-all duration-300 ease-out"
                                        style={{ width: `${job.progress}%` }}
                                    />
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            )}

            {completedJobs.length > 0 && (
                <div className="bg-slate-800/50 rounded-lg p-4 border border-slate-700/50">
                    <h3 className="text-sm font-bold text-gray-400 mb-4 px-1 uppercase tracking-wider flex items-center justify-between">
                        <span>Recently Finished</span>
                        <button
                            onClick={() => completedJobs.forEach(j => onRemove(j.job_id))}
                            className="text-[10px] text-primary hover:text-secondary font-bold uppercase tracking-tight"
                        >
                            Clear All
                        </button>
                    </h3>
                    <div className="space-y-3">
                        {completedJobs.map((job) => (
                            <div key={job.job_id} className="bg-slate-900/40 rounded-md p-3 border border-slate-800 relative group animate-in fade-in slide-in-from-right-4 duration-300">
                                <button
                                    onClick={() => onRemove(job.job_id)}
                                    className="absolute top-2 right-2 text-gray-600 hover:text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity"
                                >
                                    ✕
                                </button>
                                <div className="pr-4">
                                    <p className="text-xs font-medium text-gray-300 truncate mb-1">{job.original_filename}</p>
                                    <div className="flex items-center gap-2">
                                        {job.status === 'completed' ? (
                                            <>
                                                <span className="w-1.5 h-1.5 bg-green-500 rounded-full"></span>
                                                <span className="text-[10px] text-green-500 font-bold uppercase tracking-wide">Success</span>
                                                {job.result_url && (
                                                    <a
                                                        href={getDownloadUrl(job.result_url.split('/').pop() || '')}
                                                        className="text-[10px] text-primary hover:underline ml-auto font-bold"
                                                    >
                                                        DOWNLOAD
                                                    </a>
                                                )}
                                            </>
                                        ) : (
                                            <>
                                                <span className="w-1.5 h-1.5 bg-red-500 rounded-full"></span>
                                                <span className="text-[10px] text-red-500 font-bold uppercase tracking-wide">Failed</span>
                                            </>
                                        )}
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
