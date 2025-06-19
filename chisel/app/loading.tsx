export default function Loading() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary via-secondary to-accent">
      <div className="text-center">
        <div className="w-16 h-16 border-4 border-highlight border-t-transparent rounded-full animate-spin mx-auto mb-6"></div>
        <h2 className="text-2xl font-semibold text-text mb-2">Loading</h2>
        <p className="text-text-muted">Please wait while we prepare your experience...</p>
      </div>
    </div>
  );
}
