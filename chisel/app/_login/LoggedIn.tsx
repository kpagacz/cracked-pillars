import React from "react";
import { AuthContext } from "./AuthContext";
import config from "../_config";

export function LoggedIn() {
  const authContext = React.useContext(AuthContext);

  const handleLogout = async () => {
    try {
      await fetch(config.NEXTJS_API_URL + "/auth/logout", {
        method: "POST",
      });
      authContext.setAuthContext(null);
    } catch (error) {
      console.error("Logout failed:", error);
    }
  };

  const getRoleBadgeColor = (role: string) => {
    switch (role) {
      case "admin":
        return "bg-red-100 text-red-800 border-red-200";
      case "editor":
        return "bg-blue-100 text-blue-800 border-blue-200";
      case "viewer":
        return "bg-green-100 text-green-800 border-green-200";
      default:
        return "bg-gray-100 text-gray-800 border-gray-200";
    }
  };

  const getRoleDisplayName = (role: string) => {
    switch (role) {
      case "admin":
        return "Admin";
      case "editor":
        return "Editor";
      case "viewer":
        return "Viewer";
      default:
        return role;
    }
  };

  return (
    <div className="flex items-center gap-3">
      {/* User Info */}
      <div className="flex items-center gap-2">
        {/* User Avatar */}
        <div className="w-8 h-8 bg-gradient-to-br from-highlight to-accent rounded-full flex items-center justify-center shadow-sm">
          <span className="text-white text-sm font-semibold">
            {authContext.authContext?.email?.charAt(0).toUpperCase() || "U"}
          </span>
        </div>

        {/* User Details */}
        <div className="hidden sm:flex flex-col items-start">
          <span className="text-sm font-medium text-text truncate max-w-32">
            {authContext.authContext?.email}
          </span>
          <span
            className={`text-xs px-2 py-0.5 rounded-full border ${getRoleBadgeColor(authContext.authContext?.role || "")}`}
          >
            {getRoleDisplayName(authContext.authContext?.role || "")}
          </span>
        </div>
      </div>

      {/* Logout Button */}
      <button
        onClick={handleLogout}
        className="bg-highlight text-white hover:bg-highlight/90 transition-colors duration-200 px-3 py-1.5 rounded-md text-sm font-medium shadow-sm hover:shadow-md hover:-translate-y-0.5 active:translate-y-0 active:shadow-sm focus:outline-none focus:ring-2 focus:ring-highlight/20 focus:border-highlight"
      >
        Sign out
      </button>
    </div>
  );
}
