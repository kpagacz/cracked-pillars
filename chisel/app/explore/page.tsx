import ExplorePage from "./_components/ExplorePage";
import { fetchAllItems, Item } from "./server-functions/fetchItems";
import fetchTags, { Tag } from "./server-functions/fetchTags";

export default function ExplorePageWrapper() {
  const allItems: Promise<Item[]> = fetchAllItems();
  const tags: Promise<Tag[]> = fetchTags();

  return <ExplorePage initialItemsPromise={allItems} tags={tags} />;
}
