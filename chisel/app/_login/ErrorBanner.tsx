import React, { useState, useEffect } from "react";

export function ErrorBanner({
  message,
  type = "error",
  isVisible,
  onClose,
  autoHide = false,
  autoHideDelay = 5000,
}: {
  message: string;
  type?: "error" | "success" | "warning" | "info";
  isVisible: boolean;
  onClose: () => void;
  autoHide?: boolean;
  autoHideDelay?: number;
}) {
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    if (isVisible && autoHide) {
      const timer = setTimeout(() => {
        handleClose();
      }, autoHideDelay);
      return () => clearTimeout(timer);
    }
  }, [isVisible, autoHide, autoHideDelay]);

  const handleClose = () => {
    setIsAnimating(true);
    setTimeout(() => {
      onClose();
      setIsAnimating(false);
    }, 300);
  };

  const getTypeStyles = () => {
    switch (type) {
      case "success":
        return "bg-green-100 text-green-800 border-green-200 shadow-green-200/50";
      case "warning":
        return "bg-yellow-100 text-yellow-800 border-yellow-200 shadow-yellow-200/50";
      case "info":
        return "bg-blue-100 text-blue-800 border-blue-200 shadow-blue-200/50";
      case "error":
      default:
        return "bg-red-100 text-red-800 border-red-200 shadow-red-200/50";
    }
  };

  const getIcon = () => {
    switch (type) {
      case "success":
        return (
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M5 13l4 4L19 7"
            ></path>
          </svg>
        );
      case "warning":
        return (
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
            ></path>
          </svg>
        );
      case "info":
        return (
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            ></path>
          </svg>
        );
      case "error":
      default:
        return (
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            ></path>
          </svg>
        );
    }
  };

  if (!isVisible) return null;

  return (
    <div
      className={`fixed top-4 right-4 z-50 max-w-sm transform transition-all duration-300 ease-in-out ${
        isAnimating
          ? "opacity-0 scale-95 translate-x-full"
          : "opacity-100 scale-100 translate-x-0"
      }`}
      role="alert"
      aria-live="assertive"
    >
      <div
        className={`rounded-lg border shadow-lg p-4 ${getTypeStyles()} backdrop-blur-sm bg-opacity-95`}
      >
        <div className="flex items-start gap-3">
          <div className="flex-shrink-0 mt-0.5">{getIcon()}</div>
          <div className="flex-1 min-w-0">
            <p className="text-sm font-medium leading-5">{message}</p>
          </div>
          <button
            onClick={handleClose}
            className="flex-shrink-0 inline-flex items-center justify-center w-6 h-6 rounded-full text-current opacity-70 hover:opacity-100 hover:bg-current/10 focus:outline-none focus:ring-2 focus:ring-current/20 focus:opacity-100 transition-all duration-200"
            aria-label="Close banner"
          >
            <svg
              className="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M6 18L18 6M6 6l12 12"
              ></path>
            </svg>
          </button>
        </div>
      </div>
    </div>
  );
}
