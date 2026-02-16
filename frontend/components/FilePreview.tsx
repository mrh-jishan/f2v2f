'use client';

import { useState, useEffect } from 'react';

interface FilePreviewProps {
    url: string;
    filename: string;
}

export default function FilePreview({ url, filename }: FilePreviewProps) {
    const [fileType, setFileType] = useState<'image' | 'video' | 'text' | 'pdf' | 'other'>('other');
    const [content, setContent] = useState<string | null>(null);

    useEffect(() => {
        const ext = filename.split('.').pop()?.toLowerCase();
        if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg'].includes(ext || '')) {
            setFileType('image');
        } else if (['mp4', 'webm', 'ogg', 'mov'].includes(ext || '')) {
            setFileType('video');
        } else if (['txt', 'js', 'py', 'json', 'md', 'rs', 'go'].includes(ext || '')) {
            setFileType('text');
            fetch(url)
                .then(res => res.text())
                .then(text => setContent(text.slice(0, 5000))) // Limit preview
                .catch(() => setContent("Could not load preview"));
        } else if (ext === 'pdf') {
            setFileType('pdf');
        } else {
            setFileType('other');
        }
    }, [url, filename]);

    return (
        <div className="mt-6 bg-slate-900 rounded-lg border border-primary/30 overflow-hidden">
            <div className="bg-slate-800 px-4 py-2 flex justify-between items-center border-b border-primary/10">
                <span className="text-sm font-medium text-gray-300">🔍 Preview: {filename}</span>
                <span className="text-xs text-gray-500 uppercase">{fileType}</span>
            </div>

            <div className="p-4 flex justify-center bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-slate-800 to-slate-900 min-h-[200px] items-center">
                {fileType === 'image' && (
                    <img src={url} alt={filename} className="max-h-[400px] rounded shadow-lg transition-transform hover:scale-105" />
                )}

                {fileType === 'video' && (
                    <video controls className="max-w-full max-h-[400px] w-full shadow-2xl rounded-lg border border-white/10" autoPlay muted loop>
                        <source src={url} />
                        Your browser does not support the video tag.
                    </video>
                )}

                {fileType === 'text' && (
                    <pre className="w-full text-xs text-green-400 font-mono bg-slate-950 p-4 rounded overflow-auto max-h-[400px]">
                        {content || "Loading..."}
                    </pre>
                )}

                {fileType === 'pdf' && (
                    <iframe src={url} className="w-full h-[500px] rounded" />
                )}

                {fileType === 'other' && (
                    <div className="text-center space-y-3">
                        <div className="text-4xl">📄</div>
                        <p className="text-gray-400">Preview not available for this file type</p>
                        <a href={url} download className="inline-block text-xs text-primary hover:underline">
                            Download to view full file
                        </a>
                    </div>
                )}
            </div>
        </div>
    );
}
