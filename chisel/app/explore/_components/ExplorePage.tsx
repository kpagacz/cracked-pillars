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
  const [_, startTransition] = useTransition();
  const [items, setItems] = useState(initialItems);
  const onFilterFormSubmitted = (tags: string[]) =>
    startTransition(async () => {
      const newItems = await fetchItemsByTags(tags);
      startTransition(() => {
        setItems(newItems);
      });
    });

  return (
    <>
      <FilterTagsWidget
        onFilterFormSubmitted={onFilterFormSubmitted}
        tagsPromise={tags}
      />
      <Suspense fallback={<ItemsLoadingFallback />}>
        <ResultsOverview items={items} />
      </Suspense>
    </>
  );
}
