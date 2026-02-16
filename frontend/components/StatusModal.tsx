'use client';

import { useEffect } from 'react';

interface StatusModalProps {
  show: boolean;
  type: 'success' | 'error' | 'info' | 'warning';
  title: string;
  message: string;
  onClose: () => void;
  autoClose?: boolean;
  autoCloseDelay?: number;
}

export default function StatusModal({
  show,
  type,
  title,
  message,
  onClose,
  autoClose = true,
  autoCloseDelay = 3000,
}: StatusModalProps) {
  useEffect(() => {
    if (show && autoClose) {
      const timer = setTimeout(onClose, autoCloseDelay);
      return () => clearTimeout(timer);
    }
  }, [show, autoClose, autoCloseDelay, onClose]);

  if (!show) return null;

  const styles = {
    success: {
      bg: 'bg-green-900/90',
      border: 'border-green-500',
      text: 'text-green-400',
      icon: '✅',
    },
    error: {
      bg: 'bg-red-900/90',
      border: 'border-red-500',
      text: 'text-red-400',
      icon: '❌',
    },
    info: {
      bg: 'bg-blue-900/90',
      border: 'border-blue-500',
      text: 'text-blue-400',
      icon: 'ℹ️',
    },
    warning: {
      bg: 'bg-yellow-900/90',
      border: 'border-yellow-500',
      text: 'text-yellow-400',
      icon: '⚠️',
    },
  };

  const style = styles[type];

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
      <div
        className={`${style.bg} ${style.border} border-2 rounded-lg p-6 max-w-md w-full shadow-2xl animate-in fade-in zoom-in duration-200`}
      >
        <div className="flex items-start gap-4">
          <div className="text-3xl">{style.icon}</div>
          <div className="flex-1">
            <h3 className={`text-lg font-semibold ${style.text} mb-2`}>{title}</h3>
            <p className="text-gray-300 text-sm">{message}</p>
          </div>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition text-xl leading-none"
          >
            ✕
          </button>
        </div>
      </div>
    </div>
  );
}
