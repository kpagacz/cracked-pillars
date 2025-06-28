'use client';

import { useEffect, useRef } from 'react';

interface ModalProps {
  children: React.ReactNode;
  isOpen?: boolean;
  onClose?: () => void;
  title?: string;
  className?: string;
}

export function Modal({
  children,
  isOpen = true,
  onClose,
  title,
  className = ''
}: ModalProps) {
  const modalRef = useRef<HTMLDivElement>(null);

  // Handle ESC key press
  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && onClose) {
        onClose();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      // Prevent body scroll when modal is open
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = 'unset';
    };
  }, [isOpen, onClose]);

  // Handle click outside to close
  const handleBackdropClick = (event: React.MouseEvent) => {
    if (event.target === event.currentTarget && onClose) {
      onClose();
    }
  };

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4 animate-fade-in"
      onClick={handleBackdropClick}
    >
      {/* Backdrop */}
      <div className="absolute inset-0 bg-text/40 backdrop-blur-sm transition-all duration-300" />

      {/* Modal Content */}
      <div
        ref={modalRef}
        className={`
          relative max-w-2xl w-full max-h-[90vh] overflow-hidden
          bg-primary/95 backdrop-blur-lg border border-border/50
          rounded-2xl shadow-2xl transform transition-all duration-300
          ${className}
        `}
        role="dialog"
        aria-modal="true"
        aria-labelledby={title ? "modal-title" : undefined}
      >
        {/* Header */}
        {(title || onClose) && (
          <div className="flex items-center justify-between p-6 border-b border-border/30">
            {title && (
              <h2
                id="modal-title"
                className="text-xl font-semibold text-text"
              >
                {title}
              </h2>
            )}
            {onClose && (
              <button
                onClick={onClose}
                className="
                  ml-auto p-2 text-text-muted hover:text-text
                  hover:bg-secondary/50 rounded-lg transition-all duration-200
                  focus:outline-none focus:ring-2 focus:ring-highlight focus:ring-offset-2 focus:ring-offset-primary
                "
                aria-label="Close modal"
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            )}
          </div>
        )}

        {/* Content */}
        <div className="overflow-y-auto max-h-[calc(90vh-8rem)]">
          <div className="p-6">
            {children}
          </div>
        </div>
      </div>
    </div>
  );
}
