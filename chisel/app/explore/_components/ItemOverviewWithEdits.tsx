"use client";

import { useState, useContext, useRef, useEffect } from "react";
import { Item } from "../server-functions/fetchItems";
import { updateTagsInItem } from "../server-functions/updateTagsInItem";
import { AuthContext } from "../../_login/AuthContext";
import { ErrorBanner } from "../../_login/ErrorBanner";

interface ItemOverviewWithEditsProps {
  item: Item;
  availableTags?: string[];
  onTagsUpdate?: (updatedItem: Item) => void;
}

export default function ItemOverviewWithEdits({
  item,
  availableTags = [],
  onTagsUpdate,
}: ItemOverviewWithEditsProps) {
  const { authContext } = useContext(AuthContext);
  const [isEditing, setIsEditing] = useState(false);
  const [newTag, setNewTag] = useState("");
  const [currentTags, setCurrentTags] = useState<string[]>(item.tags || []);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showError, setShowError] = useState(false);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [filteredSuggestions, setFilteredSuggestions] = useState<string[]>([]);
  const [selectedSuggestionIndex, setSelectedSuggestionIndex] = useState(-1);
  const inputRef = useRef<HTMLInputElement>(null);
  const suggestionsRef = useRef<HTMLDivElement>(null);

  // Filter suggestions based on input
  useEffect(() => {
    if (!newTag.trim()) {
      // Show all available tags when input is empty and focused
      const availableTagsNotUsed = availableTags.filter(
        (tag) => !currentTags.includes(tag),
      );
      setFilteredSuggestions(availableTagsNotUsed); // Show all available tags
    } else {
      // Filter tags that start with the input and aren't already used
      const filtered = availableTags.filter(
        (tag) =>
          tag.toLowerCase().startsWith(newTag.toLowerCase()) &&
          !currentTags.includes(tag),
      );
      setFilteredSuggestions(filtered); // Show all matching tags
    }
    setSelectedSuggestionIndex(-1);
  }, [newTag, availableTags, currentTags]);

  // Handle keyboard navigation
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      e.preventDefault();
      if (
        selectedSuggestionIndex >= 0 &&
        filteredSuggestions[selectedSuggestionIndex]
      ) {
        // Add the selected suggestion
        const selectedTag = filteredSuggestions[selectedSuggestionIndex];
        if (!currentTags.includes(selectedTag)) {
          handleAddTag(selectedTag);
        }
      } else {
        // Add the current input
        handleAddTag();
      }
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      setSelectedSuggestionIndex((prev) =>
        prev < filteredSuggestions.length - 1 ? prev + 1 : prev,
      );
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setSelectedSuggestionIndex((prev) => (prev > 0 ? prev - 1 : -1));
    } else if (e.key === "Escape") {
      setShowSuggestions(false);
      setSelectedSuggestionIndex(-1);
    }
  };

  const handleAddTag = async (tagToAdd?: string) => {
    const tag = tagToAdd || newTag.trim();
    if (!tag || currentTags.includes(tag)) {
      return;
    }

    const updatedTags = [...currentTags, tag];
    await updateTags(updatedTags);
    setNewTag("");
    setShowSuggestions(false);
    setSelectedSuggestionIndex(-1);
  };

  const handleDeleteTag = async (tagToDelete: string) => {
    const updatedTags = currentTags.filter((tag) => tag !== tagToDelete);
    await updateTags(updatedTags);
  };

  const handleInputFocus = () => {
    setShowSuggestions(true);
  };

  const handleInputBlur = () => {
    // Delay hiding suggestions to allow for clicks
    setTimeout(() => {
      setShowSuggestions(false);
      setSelectedSuggestionIndex(-1);
    }, 200);
  };

  const handleSuggestionClick = (suggestion: string) => {
    handleAddTag(suggestion);
  };

  const updateTags = async (updatedTags: string[]) => {
    if (!authContext?.jwt) {
      setError("Authentication required to update tags");
      setShowError(true);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const updatedItem = await updateTagsInItem(
        { ...item, tags: updatedTags },
        authContext.jwt,
      );

      setCurrentTags(updatedItem.tags || []);

      // Call the parent callback if provided
      if (onTagsUpdate) {
        onTagsUpdate(updatedItem);
      }
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to update tags";
      setError(errorMessage);
      setShowError(true);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <>
      <ErrorBanner
        message={error || ""}
        type="error"
        isVisible={showError}
        onClose={() => setShowError(false)}
        autoHide={true}
        autoHideDelay={5000}
      />

      <div className="bg-primary/30 hover:bg-primary/50 border border-border/50 hover:border-highlight/50 rounded-lg p-6 transition-all duration-200 group shadow-sm">
        <div className="flex items-start justify-between">
          <div className="flex-1">
            <h3 className="text-lg font-semibold text-text mb-2 group-hover:text-highlight transition-colors duration-200">
              {item.name}
            </h3>

            {/* Tags Display and Editing */}
            <div className="mb-4">
              <div className="flex items-center gap-2 mb-2">
                <span className="text-sm font-medium text-text-muted">
                  Tags:
                </span>
                {authContext && (
                  <button
                    onClick={() => setIsEditing(!isEditing)}
                    disabled={isLoading}
                    className="text-xs px-2 py-1 bg-highlight/20 hover:bg-highlight/30 text-highlight rounded transition-colors duration-200 disabled:opacity-50"
                  >
                    {isEditing ? "Cancel" : "Edit"}
                  </button>
                )}
              </div>

              {/* Tags List */}
              <div className="flex flex-wrap gap-2 mb-2">
                {currentTags.map((tag, index) => (
                  <span
                    key={index}
                    className="inline-flex items-center gap-1 px-2 py-1 rounded-md text-xs bg-accent/30 text-text-muted border border-border/30"
                  >
                    {tag}
                    {isEditing && (
                      <button
                        onClick={() => handleDeleteTag(tag)}
                        disabled={isLoading}
                        className="ml-1 text-red-400 hover:text-red-600 transition-colors duration-200 disabled:opacity-50"
                        title="Remove tag"
                      >
                        <svg
                          className="w-3 h-3"
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
                  </span>
                ))}
              </div>

              {/* Add New Tag Input with Autocomplete */}
              {isEditing && (
                <div className="relative">
                  <div className="flex gap-2 items-center">
                    <input
                      ref={inputRef}
                      type="text"
                      value={newTag}
                      onChange={(e) => setNewTag(e.target.value)}
                      onKeyDown={handleKeyDown}
                      onFocus={handleInputFocus}
                      onBlur={handleInputBlur}
                      placeholder="Add new tag..."
                      disabled={isLoading}
                      className="flex-1 px-3 py-1 text-sm bg-background border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-highlight/50 focus:border-highlight disabled:opacity-50"
                    />
                    <button
                      onClick={() => handleAddTag()}
                      disabled={
                        !newTag.trim() ||
                        isLoading ||
                        currentTags.includes(newTag.trim())
                      }
                      className="px-3 py-1 text-xs bg-highlight/20 hover:bg-highlight/30 text-highlight rounded transition-colors duration-200 disabled:opacity-50"
                    >
                      Add
                    </button>
                  </div>

                  {/* Suggestions Dropdown */}
                  {showSuggestions && filteredSuggestions.length > 0 && (
                    <div
                      ref={suggestionsRef}
                      className="absolute top-full left-0 right-0 mt-1 border border-border rounded-md shadow-xl z-10 max-h-48 overflow-y-auto"
                      style={{
                        backgroundColor: "#f8f6f1", // Using the primary color from theme
                        backdropFilter: "blur(8px)",
                        boxShadow:
                          "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",
                      }}
                    >
                      {filteredSuggestions.map((suggestion, index) => (
                        <button
                          key={suggestion}
                          onClick={() => handleSuggestionClick(suggestion)}
                          className={`w-full px-3 py-2 text-left text-sm hover:bg-highlight/20 transition-colors duration-200 ${
                            index === selectedSuggestionIndex
                              ? "bg-highlight/30 text-highlight"
                              : "text-text"
                          }`}
                          style={{
                            backgroundColor:
                              index === selectedSuggestionIndex
                                ? "rgba(139, 115, 85, 0.3)"
                                : "transparent",
                          }}
                        >
                          {suggestion}
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              )}

              {/* Loading Indicator */}
              {isLoading && (
                <div className="mt-2 text-xs text-text-muted flex items-center gap-2">
                  <svg
                    className="animate-spin h-3 w-3"
                    fill="none"
                    viewBox="0 0 24 24"
                  >
                    <circle
                      className="opacity-25"
                      cx="12"
                      cy="12"
                      r="10"
                      stroke="currentColor"
                      strokeWidth="4"
                    ></circle>
                    <path
                      className="opacity-75"
                      fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    ></path>
                  </svg>
                  Updating tags...
                </div>
              )}
            </div>
          </div>

          {item.wiki_url && (
            <a
              href={item.wiki_url}
              target="_blank"
              rel="noopener noreferrer"
              className="ml-4 px-4 py-2 bg-highlight/20 hover:bg-highlight/30 text-highlight border border-highlight/30 rounded-lg text-sm font-medium transition-all duration-200 hover:scale-105 flex items-center gap-2 shadow-sm"
            >
              <span>Wiki</span>
              <svg
                className="w-4 h-4"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                />
              </svg>
            </a>
          )}
        </div>
      </div>
    </>
  );
}
