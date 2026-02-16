import { useState, useEffect, useCallback } from 'react';
import { JobStatus } from './api';

export function useWebSocket() {
    const [activeJobs, setActiveJobs] = useState<Record<string, JobStatus>>({});

    useEffect(() => {
        let socket: WebSocket | null = null;
        let reconnectTimeout: NodeJS.Timeout;

        const connect = () => {
            const apiBase = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:5000';
            const url = new URL(apiBase);
            const protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
            // Use the host from apiBase (e.g., localhost:5000)
            const wsUrl = `${protocol}//${url.host}/api/ws`;

            socket = new WebSocket(wsUrl);

            socket.onmessage = (event) => {
                try {
                    const job: JobStatus = JSON.parse(event.data);
                    setActiveJobs((prev) => {
                        const next = { ...prev };
                        if (job.status === 'completed' || job.status === 'failed') {
                            // Optionally keep completed jobs for a bit or remove them
                            // For now, let's keep them so the user sees the final state
                            next[job.job_id] = job;
                        } else {
                            next[job.job_id] = job;
                        }
                        return next;
                    });
                } catch (err) {
                    console.error('Failed to parse WS message:', err);
                }
            };

            socket.onclose = () => {
                reconnectTimeout = setTimeout(connect, 3000);
            };

            socket.onerror = (err) => {
                console.error('WS Error:', err);
                socket?.close();
            };
        };

        connect();

        return () => {
            socket?.close();
            clearTimeout(reconnectTimeout);
        };
    }, []);

    const removeJob = useCallback((jobId: string) => {
        setActiveJobs((prev) => {
            const next = { ...prev };
            delete next[jobId];
            return next;
        });
    }, []);

    return {
        activeJobs: Object.values(activeJobs),
        removeJob
    };
}
