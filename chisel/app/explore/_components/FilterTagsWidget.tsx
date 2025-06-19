import {
  ChangeEvent,
  EventHandler,
  MouseEvent,
  Suspense,
  use,
  useState,
} from "react";

export default function ({
  onFilterFormSubmitted,
  tagsPromise,
}: {
  onFilterFormSubmitted: (payload: string[]) => void;
  tagsPromise: Promise<string[]>;
}) {
  const tags = use(tagsPromise);
  const [searchedTags, setSearchedTags] = useState<string[]>([""]);
  const [searchText, setSearchText] = useState<string>("");
  const [recommendedTags, setRecommendedTags] = useState<string[]>(tags);
  const onSearchedTagChanged = (event: ChangeEvent<HTMLInputElement>) => {
    const tagPrefix = event.target.value;
    const filtered = tags.filter(
      (tag) => tag.startsWith(tagPrefix) && !searchedTags.includes(tag),
    );
    setRecommendedTags(filtered);
    setSearchText(tagPrefix);
  };
  const onTagAdded = (tag: string) => {
    setSearchedTags([...searchedTags, tag]);
    setRecommendedTags(recommendedTags.filter((t) => t !== tag));
  };
  const clearTags = (_: MouseEvent<HTMLButtonElement>) => {
    setSearchedTags([]);
  };
  const onFormSubmitted: EventHandler<MouseEvent<HTMLFormElement>> = (e) => {
    e.preventDefault();
    onFilterFormSubmitted(searchedTags);
  };
  return (
    <>
      <form onSubmit={onFormSubmitted}>
        <input
          name="tagSearch"
          type="text"
          id="tagSearch"
          placeholder="Start typing a tag..."
          autoFocus
          onChange={onSearchedTagChanged}
          value={searchText}
          onBlur={(e) => {
            e.target.focus();
          }}
        ></input>
        <Suspense fallback={<span>Loading available tags...</span>}>
          <TagList tags={recommendedTags} onTagAdded={onTagAdded} />
        </Suspense>
        <div>
          {searchedTags.map((tag) => {
            return <span key={tag}>{tag}</span>;
          })}
        </div>
        <button onClick={clearTags}>Clear tags</button>
        <button type="submit">Filter</button>
      </form>
    </>
  );
}

function Tag({
  tag,
  onTagAdded,
}: {
  tag: string;
  onTagAdded: (_: string) => void;
}) {
  return (
    <>
      {tag}
      <button
        onClick={(e) => {
          e.preventDefault();
          onTagAdded(tag);
        }}
      >
        +
      </button>
    </>
  );
}

function TagList({
  tags,
  onTagAdded,
}: {
  tags: string[];
  onTagAdded: (_: string) => void;
}) {
  return (
    <>
      {tags.map((tag) => {
        return <Tag key={tag} tag={tag} onTagAdded={onTagAdded} />;
      })}
    </>
  );
}
