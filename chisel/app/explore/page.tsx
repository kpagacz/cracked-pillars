import ExplorePage from "./_components/ExplorePage";
import { Item } from "./server-functions/fetchItems";
import fetchTags from "./server-functions/fetchTags";

export default function () {
  const allItems: Item[] =
    // TODO actually fetch items
    [];
  const tags = fetchTags();
  return <ExplorePage initialItems={allItems} tags={tags} />;
}
