"use server";

export interface Item {
  name: string;
  slug: string;
  tags: string[];
  wiki_url: string;
}

interface RawItem {
  name: string;
  slug: string;
  tags: string[];
  wiki_url: string;
}

export async function fetchItemsByTags(tags: string[]): Promise<Item[]> {
  const searchParams = new URLSearchParams();
  tags.forEach((tag) => searchParams.append("tags", tag));
  searchParams.append("filter_logic", "and");
  const abilitiesApi = `http://${process.env.NEXT_PUBLIC_API_URL}/indexed?${searchParams}`;
  try {
    const response = await fetch(abilitiesApi);
    const data = await response.json();
    console.log("Fetched items by tags: ", data);
    return data as Item[];
  } catch (error) {
    console.error("Error fetching items by tags: ", error);
    return [];
  }
}

export async function fetchAllItems(): Promise<Item[]> {
  try {
    const itemsResponse = await fetch(
      `http://${process.env.NEXT_PUBLIC_API_URL}/items`,
    );
    const items = await itemsResponse.json();
    console.log("Fetched all items: ", items);
    const properItems: Item[] = items.map((item: RawItem) => {
      return {
        name: item.name,
        slug: item.slug,
        tags: item.tags,
        wiki_url: item.wiki_url,
      };
    });

    const abilitiesResponse = await fetch(
      `http://${process.env.NEXT_PUBLIC_API_URL}/abilities`,
    );
    const abilities = await abilitiesResponse.json();
    console.log("Fetched all abilities: ", abilities);
    const properAbilities: Item[] = abilities.map((ability: RawItem) => {
      return {
        name: ability.name,
        slug: ability.slug,
        tags: ability.tags,
        wiki_url: ability.wiki_url,
      };
    });
    return [...properItems, ...properAbilities];
  } catch (error) {
    console.error("Error fetching all items: ", error);
    return [];
  }
}
