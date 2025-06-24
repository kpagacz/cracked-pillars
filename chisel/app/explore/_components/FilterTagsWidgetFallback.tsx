export default function FilterTagsWidgetFallback() {
  return (
    <div className="space-y-6">
      {/* Search Input Skeleton */}
      <div>
        <div className="h-4 bg-accent/30 rounded w-20 mb-2 animate-pulse-slow"></div>
        <div className="h-12 bg-accent/30 rounded-lg w-full animate-pulse-slow"></div>
      </div>

      {/* Available Tags Section */}
      <div>
        <div className="h-4 bg-accent/30 rounded w-24 mb-3 animate-pulse-slow"></div>
        <div className="flex flex-wrap gap-2">
          {[...Array(8)].map((_, index) => (
            <div
              key={index}
              className="h-10 bg-accent/30 rounded-lg w-20 animate-pulse-slow"
              style={{
                animationDelay: `${index * 0.1}s`,
              }}
            ></div>
          ))}
        </div>
      </div>

      {/* Action Buttons Skeleton */}
      <div className="flex gap-3 pt-4">
        <div className="flex-1 h-10 bg-accent/30 rounded-lg animate-pulse-slow"></div>
        <div className="flex-1 h-10 bg-accent/30 rounded-lg animate-pulse-slow"></div>
      </div>

      {/* Loading Message */}
      <div className="text-center py-4">
        <div className="inline-flex items-center gap-2 text-text-muted">
          <div className="w-4 h-4 border-2 border-highlight border-t-transparent rounded-full animate-spin"></div>
          <span className="text-sm">Loading filters...</span>
        </div>
      </div>
    </div>
  );
}
