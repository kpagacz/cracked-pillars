"use client";

import { FilterTagsWidget } from "./FilterTagsWidget";
import ItemsLoadingFallback from "../_components/ItemsLoadingFallback";
import ResultsOverview from "./ResultsOverview";
import { Item } from "../server-functions/fetchItems";
import { Suspense, use, useState, useTransition } from "react";
import { fetchItemsByTags } from "../server-functions/fetchItems";
import FilterTagsWidgetFallback from "./FilterTagsWidgetFallback";
import { Tag } from "../server-functions/fetchTags";
import { Modal } from "../../_components/Modal";
import { TagsExplanation } from "./TagsExplanation";

export default function ExplorePage({
  initialItemsPromise,
  tags,
}: {
  initialItemsPromise: Promise<Item[]>;
  tags: Promise<Tag[]>;
}) {
  const initialItems = use(initialItemsPromise);
  const resolvedTags = use(tags);
  const [, startTransition] = useTransition();
  const [items, setItems] = useState(initialItems);
  const [isTagsModalOpen, setIsTagsModalOpen] = useState(false);
  const onFilterFormSubmitted = (tags: string[]) =>
    startTransition(async () => {
      const newItems = await fetchItemsByTags(tags);
      startTransition(() => {
        setItems(newItems);
      });
    });

  return (
    <div className="min-h-screen bg-gradient-to-br from-primary via-secondary to-accent">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Page Header */}
        <div className="mb-8">
          <div className="flex items-center justify-between mb-2">
            <h1 className="text-4xl font-bold text-highlight">
              Explore Items
            </h1>
            <button
              onClick={() => setIsTagsModalOpen(true)}
              className="
                px-4 py-2 bg-highlight/20 hover:bg-highlight/30
                text-highlight border border-highlight/30 rounded-lg
                transition-all duration-200 font-medium
                focus:outline-none focus:ring-2 focus:ring-highlight focus:ring-offset-2
              "
            >
              <span className="flex items-center gap-2">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                Tags Help
              </span>
            </button>
          </div>
          <p className="text-text-muted">
            Discover and filter items and abilities from Pillars of Eternity II:
            Deadfire
          </p>
        </div>

        {/* Main Content */}
        <div className="grid lg:grid-cols-4 gap-8">
          {/* Filter Sidebar */}
          <div className="lg:col-span-1">
            <div className="bg-secondary/70 backdrop-blur-sm border border-border/50 rounded-lg p-6 sticky top-24 shadow-sm">
              <h2 className="text-xl font-semibold text-text mb-4">Filters</h2>
              <Suspense fallback={<FilterTagsWidgetFallback />}>
                <FilterTagsWidget
                  onFilterFormSubmitted={onFilterFormSubmitted}
                  tagsPromise={tags}
                />
              </Suspense>
            </div>
          </div>

          {/* Results Section */}
          <div className="lg:col-span-3">
            <div className="bg-secondary/50 backdrop-blur-sm border border-border/50 rounded-lg p-6 shadow-sm">
              <Suspense fallback={<ItemsLoadingFallback />}>
                <ResultsOverview items={items} availableTags={tags} />
              </Suspense>
            </div>
          </div>
        </div>

        {/* Tags Explanation Modal */}
        <Modal
          isOpen={isTagsModalOpen}
          onClose={() => setIsTagsModalOpen(false)}
          title="Tags Explanation"
          className="max-w-4xl"
        >
          <TagsExplanation tags={resolvedTags} />
        </Modal>
      </div>
    </div>
  );
}
