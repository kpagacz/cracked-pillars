"use client";
import FilterTagsWidget from "./FilterTagsWidget";
import ItemsLoadingFallback from "../_components/ItemsLoadingFallback";
import ResultsOverview from "./ResultsOverview";
import { Item } from "../server-functions/fetchItems";
import { Suspense, useState, useTransition } from "react";
import { fetchItemsByTags } from "../server-functions/fetchItems";

export default function ExplorePage({
  initialItems,
  tags,
}: {
  initialItems: Item[];
  tags: Promise<string[]>;
}) {
  const [, startTransition] = useTransition();
  const [items, setItems] = useState(initialItems);
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
          <h1 className="text-4xl font-bold text-highlight mb-2">
            Explore Items
          </h1>
          <p className="text-text-muted">
            Discover and filter items and abilities from Pillars of Eternity II:
            Deadfire
          </p>
        </div>

        {/* Main Content */}
        <div className="grid lg:grid-cols-4 gap-8">
          {/* Filter Sidebar */}
          <div className="lg:col-span-1">
            <div className="bg-secondary/50 backdrop-blur-sm border border-border/30 rounded-lg p-6 sticky top-24">
              <h2 className="text-xl font-semibold text-text mb-4">Filters</h2>
              <FilterTagsWidget
                onFilterFormSubmitted={onFilterFormSubmitted}
                tagsPromise={tags}
              />
            </div>
          </div>

          {/* Results Section */}
          <div className="lg:col-span-3">
            <div className="bg-secondary/30 backdrop-blur-sm border border-border/30 rounded-lg p-6">
              <Suspense fallback={<ItemsLoadingFallback />}>
                <ResultsOverview items={items} />
              </Suspense>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
