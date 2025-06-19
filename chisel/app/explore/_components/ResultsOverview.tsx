import { Item } from "../server-functions/fetchItems";
import ItemOverview from "./ItemOverview";

export default function ({ items }: { items: Item[] }) {
  return (
    <>
      {items.map((item) => {
        return <ItemOverview key={item.name} item={item} />;
      })}
    </>
  );
}
