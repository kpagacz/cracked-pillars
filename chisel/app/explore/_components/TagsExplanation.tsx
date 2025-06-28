import { Tag } from "../server-functions/fetchTags";

export function TagsExplanation({ tags }: { tags: Tag[] }) {
  return (
    <div className="space-y-6">
      <p className="text-text-muted text-sm leading-relaxed">
        Tags help you filter and find specific types of items and abilities. Each tag represents a category or property that items can have.
      </p>

      <div className="grid gap-4">
        {tags.map((tag) => (
          <div
            key={tag.name}
            className="bg-secondary/30 border border-border/30 rounded-lg p-4 hover:bg-secondary/50 transition-colors duration-200"
          >
            <h3 className="text-lg font-semibold text-highlight mb-2">
              {tag.name}
            </h3>
            <p className="text-text leading-relaxed">
              {tag.description}
            </p>
          </div>
        ))}
      </div>

      {tags.length === 0 && (
        <div className="text-center py-8">
          <p className="text-text-muted">No tags available at the moment.</p>
        </div>
      )}
    </div>
  );
}
