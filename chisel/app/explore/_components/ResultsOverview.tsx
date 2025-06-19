import { Item } from "../server-functions/fetchItems";
import ItemOverview from "./ItemOverview";

export default function ResultsOverview({ items }: { items: Item[] }) {
  if (items.length === 0) {
    return (
      <div className="text-center py-12">
        <div className="text-6xl mb-4">🔍</div>
        <h3 className="text-xl font-semibold text-text mb-2">No items found</h3>
        <p className="text-text-muted">
          Try adjusting your filters to find what you&apos;re looking for
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Results Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-semibold text-text">
          Results ({items.length})
        </h2>
        <div className="text-text-muted text-sm">
          Showing {items.length} item{items.length !== 1 ? 's' : ''}
        </div>
      </div>

      {/* Items Grid */}
      <div className="grid gap-4">
        {items.map((item) => (
          <ItemOverview key={item.name} item={item} />
        ))}
      </div>
    </div>
  );
}
