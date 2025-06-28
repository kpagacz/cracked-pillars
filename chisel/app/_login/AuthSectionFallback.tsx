import React from "react";

export function AuthSectionFallback() {
  return (
    <div className="flex items-center gap-3">
      {/* Loading Skeleton for User Info */}
      <div className="flex items-center gap-2">
        {/* Loading Avatar */}
        <div className="w-8 h-8 bg-gray-400 rounded-full animate-pulse"></div>

        {/* Loading User Details */}
        <div className="hidden sm:flex flex-col items-start gap-1">
          <div className="w-32 h-3 bg-gray-400 rounded animate-pulse"></div>
          <div className="w-16 h-2 bg-gray-400 rounded-full animate-pulse"></div>
        </div>
      </div>

      {/* Loading Button - match the actual logout button size */}
      <div className="w-20 h-8 bg-gray-400 rounded-md animate-pulse"></div>
    </div>
  );
}
