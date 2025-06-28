import {
  ChangeEvent,
  EventHandler,
  MouseEvent,
  Suspense,
  use,
  useState,
} from "react";

import { Tag } from "../server-functions/fetchTags";

export function FilterTagsWidget({
  onFilterFormSubmitted,
  tagsPromise,
}: {
  onFilterFormSubmitted: (payload: string[]) => void;
  tagsPromise: Promise<Tag[]>;
}) {
  const tags = use(tagsPromise).map((tag) => tag.name);
  const [searchedTags, setSearchedTags] = useState<string[]>([]);
  const [searchText, setSearchText] = useState<string>("");
  const [recommendedTags, setRecommendedTags] = useState<string[]>(tags);
  const onSearchedTagChanged = (event: ChangeEvent<HTMLInputElement>) => {
    const tagPrefix = event.target.value;
    const filtered = tags.filter(
      (tag) => tag.startsWith(tagPrefix) && !searchedTags.includes(tag),
    );
    setRecommendedTags(filtered);
    setSearchText(tagPrefix);
  };
  const onTagAdded = (tag: string) => {
    setSearchedTags([...searchedTags, tag]);
    setRecommendedTags(recommendedTags.filter((t) => t !== tag));
  };
  const onTagRemoved = (tag: string) => {
    const newSearchedTags = searchedTags.filter((t) => t !== tag);
    setSearchedTags(newSearchedTags);
    // Recalculate recommendedTags based on current search and remaining selected tags
    const filtered = tags.filter(
      (tag) => tag.startsWith(searchText) && !newSearchedTags.includes(tag),
    );
    setRecommendedTags(filtered);
  };
  const clearTags = () => {
    setSearchedTags([]);
    setRecommendedTags(tags);
    setSearchText("");
  };
  const onFormSubmitted: EventHandler<MouseEvent<HTMLFormElement>> = (e) => {
    e.preventDefault();
    onFilterFormSubmitted(searchedTags);
  };

  return (
    <div className="space-y-6">
      <form onSubmit={onFormSubmitted} className="space-y-4">
        {/* Search Input */}
        <div>
          <label
            htmlFor="tagSearch"
            className="block text-sm font-medium text-text mb-2"
          >
            Search Tags
          </label>
          <input
            name="tagSearch"
            type="text"
            id="tagSearch"
            placeholder="Start typing a tag..."
            autoFocus
            onChange={onSearchedTagChanged}
            value={searchText}
            className="w-full px-4 py-3 bg-primary/50 border border-border/50 rounded-lg text-text placeholder-text-muted focus:outline-none focus:ring-2 focus:ring-highlight focus:border-transparent transition-all duration-200"
          />
        </div>

        {/* Recommended Tags */}
        <div>
          <h3 className="text-sm font-medium text-text mb-3">Available Tags</h3>
          <Suspense
            fallback={
              <div className="text-text-muted text-sm animate-pulse-slow">
                Loading available tags...
              </div>
            }
          >
            <TagList tags={recommendedTags} onTagAdded={onTagAdded} />
          </Suspense>
        </div>

        {/* Selected Tags */}
        {searchedTags.length > 0 && (
          <div>
            <h3 className="text-sm font-medium text-text mb-3">
              Selected Tags
            </h3>
            <div className="flex flex-wrap gap-2 mb-4">
              {searchedTags.map((tag) => (
                <button
                  key={tag}
                  onClick={() => onTagRemoved(tag)}
                  className="inline-flex items-center gap-2 px-3 py-1 rounded-full text-sm bg-highlight/20 text-highlight border border-highlight/30 hover:bg-highlight/30 transition-all duration-200 group"
                >
                  <span>{tag}</span>
                  <span className="text-highlight/70 group-hover:text-highlight transition-colors duration-200">
                    ×
                  </span>
                </button>
              ))}
            </div>
          </div>
        )}

        {/* Action Buttons */}
        <div className="flex gap-3 pt-4">
          <button
            type="button"
            onClick={clearTags}
            className="flex-1 px-4 py-2 bg-primary/50 hover:bg-primary/70 text-text border border-border/50 rounded-lg transition-all duration-200 hover:border-highlight/50 shadow-sm"
          >
            Clear All
          </button>
          <button
            type="submit"
            className="flex-1 px-4 py-2 bg-highlight hover:bg-highlight/90 text-white font-medium rounded-lg transition-all duration-200 transform hover:scale-105 shadow-sm"
          >
            Apply Filters
          </button>
        </div>
      </form>
    </div>
  );
}

function TagPill({
  tag,
  onTagAdded,
}: {
  tag: string;
  onTagAdded: (_: string) => void;
}) {
  return (
    <button
      onClick={(e) => {
        e.preventDefault();
        onTagAdded(tag);
      }}
      className="inline-flex items-center gap-2 px-3 py-2 bg-primary/30 hover:bg-primary/50 text-text border border-border/30 rounded-lg transition-all duration-200 hover:border-highlight/50 group shadow-sm"
    >
      <span className="text-sm">{tag}</span>
      <span className="text-highlight group-hover:scale-110 transition-transform duration-200">
        +
      </span>
    </button>
  );
}

function TagList({
  tags,
  onTagAdded,
}: {
  tags: string[];
  onTagAdded: (_: string) => void;
}) {
  if (tags.length === 0) {
    return (
      <div className="text-text-muted text-sm italic">No tags available</div>
    );
  }

  return (
    <div className="flex flex-wrap gap-2 max-h-96 overflow-y-auto">
      {tags.map((tag) => {
        return <TagPill key={tag} tag={tag} onTagAdded={onTagAdded} />;
      })}
    </div>
  );
}
