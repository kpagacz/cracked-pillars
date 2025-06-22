import ExplorePage from "./_components/ExplorePage";
import { fetchAllItems, Item } from "./server-functions/fetchItems";
import fetchTags from "./server-functions/fetchTags";

export default function ExplorePageWrapper() {
  const allItems: Promise<Item[]> = fetchAllItems();

  const tags = fetchTags();
  return <ExplorePage initialItemsPromise={allItems} tags={tags} />;
}
