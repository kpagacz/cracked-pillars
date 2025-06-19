import { Item } from "../server-functions/fetchItems";

export default function ({ item }: { item: Item }) {
  return (
    <div>
      <span>{item.name}</span> | <span>{item.tags}</span> |{" "}
      <a href={item.wiki_url}>Wiki</a>
    </div>
  );
}
