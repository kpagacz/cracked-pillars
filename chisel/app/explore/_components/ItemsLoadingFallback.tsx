export default function ItemsLoadingFallback() {
  return (
    <div className="space-y-6">
      {/* Loading Header */}
      <div className="flex items-center justify-between">
        <div className="h-8 bg-secondary/50 rounded-lg w-48 animate-pulse-slow"></div>
        <div className="h-4 bg-secondary/50 rounded w-24 animate-pulse-slow"></div>
      </div>

      {/* Loading Items */}
      <div className="grid gap-4">
        {[...Array(5)].map((_, index) => (
          <div key={index} className="bg-secondary/30 border border-border/30 rounded-lg p-6">
            <div className="flex items-start justify-between">
              <div className="flex-1">
                <div className="h-6 bg-secondary/50 rounded w-3/4 mb-2 animate-pulse-slow"></div>
                <div className="flex gap-2 mb-4">
                  <div className="h-6 bg-secondary/50 rounded w-16 animate-pulse-slow"></div>
                  <div className="h-6 bg-secondary/50 rounded w-20 animate-pulse-slow"></div>
                  <div className="h-6 bg-secondary/50 rounded w-12 animate-pulse-slow"></div>
                </div>
              </div>
              <div className="ml-4 h-8 bg-secondary/50 rounded w-16 animate-pulse-slow"></div>
            </div>
          </div>
        ))}
      </div>

      {/* Loading Message */}
      <div className="text-center py-8">
        <div className="inline-flex items-center gap-2 text-text-muted">
          <div className="w-4 h-4 border-2 border-highlight border-t-transparent rounded-full animate-spin"></div>
          <span>Loading items...</span>
        </div>
      </div>
    </div>
  );
}
