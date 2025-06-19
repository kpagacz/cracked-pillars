"use client";

import { useEffect } from "react";

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    console.error(error);
  }, [error]);

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary via-secondary to-accent px-4">
      <div className="text-center max-w-md">
        <div className="text-8xl mb-6">⚠️</div>
        <h1 className="text-4xl font-bold text-highlight mb-4">Something went wrong!</h1>
        <p className="text-text-muted mb-8">
          We encountered an unexpected error. Please try again or contact support if the problem persists.
        </p>
        <div className="space-y-4">
          <button
            onClick={reset}
            className="inline-block bg-highlight hover:bg-highlight/90 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-200 transform hover:scale-105"
          >
            Try Again
          </button>
          <div className="text-text-muted text-sm">
            Error: {error.message || "Unknown error"}
          </div>
        </div>
      </div>
    </div>
  );
}
