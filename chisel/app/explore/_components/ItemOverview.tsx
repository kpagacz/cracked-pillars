import { Item } from "../server-functions/fetchItems";

export default function ItemOverview({ item }: { item: Item }) {
  return (
    <div className="bg-secondary/30 hover:bg-secondary/50 border border-border/30 hover:border-highlight/30 rounded-lg p-6 transition-all duration-200 group">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <h3 className="text-lg font-semibold text-text mb-2 group-hover:text-highlight transition-colors duration-200">
            {item.name}
          </h3>

          {item.tags && item.tags.length > 0 && (
            <div className="flex flex-wrap gap-2 mb-4">
              {item.tags.map((tag, index) => (
                <span
                  key={index}
                  className="inline-flex items-center px-2 py-1 rounded-md text-xs bg-accent/30 text-text-muted border border-border/30"
                >
                  {tag}
                </span>
              ))}
            </div>
          )}
        </div>

        {item.wiki_url && (
          <a
            href={item.wiki_url}
            target="_blank"
            rel="noopener noreferrer"
            className="ml-4 px-4 py-2 bg-highlight/20 hover:bg-highlight/30 text-highlight border border-highlight/30 rounded-lg text-sm font-medium transition-all duration-200 hover:scale-105 flex items-center gap-2"
          >
            <span>Wiki</span>
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
            </svg>
          </a>
        )}
      </div>
    </div>
  );
}
