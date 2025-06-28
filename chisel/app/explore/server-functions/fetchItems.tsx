"use server";

export interface Item {
  name: string;
  slug: string;
  tags: string[];
  wiki_url: string;
  type: "ability" | "item";
}

export interface RawItem {
  name: string;
  slug: string;
  tags: string[];
  wiki_url: string;
}

export async function fetchItemsByTags(tags: string[]): Promise<Item[]> {
  const searchParams = new URLSearchParams();
  tags.forEach((tag) => searchParams.append("tags", tag));
  searchParams.append("filter_logic", "and");
  console.log("Fetching items with tags: ", tags);
  const abilitiesApi = `${process.env.SERVER_API_ENDPOINT}/indexed?${searchParams}`;
  try {
    const response = await fetch(abilitiesApi);
    const data = await response.json();
    return data as Item[];
  } catch (error) {
    console.error("Error fetching items by tags: ", error);
    return [];
  }
}

export async function fetchAllItems(): Promise<Item[]> {
  try {
    console.log(
      `[DEBUG] fetchAllItems called with SERVER_API_ENDPOINT: ${process.env.SERVER_API_ENDPOINT}`,
    );

    const itemsResponse = await fetch(
      `${process.env.SERVER_API_ENDPOINT}/items`,
      { cache: "no-store" },
    );
    console.log(
      `[DEBUG] fetchAllItems items response status: ${itemsResponse.status}`,
    );

    if (!itemsResponse.ok) {
      console.error(
        `[DEBUG] fetchAllItems items failed with status: ${itemsResponse.status}`,
      );
      return [];
    }

    const items = await itemsResponse.json();
    console.log(`[DEBUG] fetchAllItems got ${items.length} items`);

    const properItems: Item[] = items.map((item: RawItem) => {
      return {
        name: item.name,
        slug: item.slug,
        tags: item.tags,
        wiki_url: item.wiki_url,
        type: "item",
      };
    });

    const abilitiesResponse = await fetch(
      `${process.env.SERVER_API_ENDPOINT}/abilities`,
    );
    console.log(
      `[DEBUG] fetchAllItems abilities response status: ${abilitiesResponse.status}`,
    );

    if (!abilitiesResponse.ok) {
      console.error(
        `[DEBUG] fetchAllItems abilities failed with status: ${abilitiesResponse.status}`,
      );
      return properItems; // Return items even if abilities fail
    }

    const abilities = await abilitiesResponse.json();
    console.log(`[DEBUG] fetchAllItems got ${abilities.length} abilities`);

    const properAbilities: Item[] = abilities.map((ability: RawItem) => {
      return {
        name: ability.name,
        slug: ability.slug,
        tags: ability.tags,
        wiki_url: ability.wiki_url,
        type: "ability",
      };
    });

    const result = [...properItems, ...properAbilities];
    console.log(`[DEBUG] fetchAllItems returning ${result.length} total items`);
    return result;
  } catch (error) {
    console.error("[DEBUG] Error fetching all items: ", error);
    console.error("[DEBUG] Error details:", JSON.stringify(error, null, 2));
    return [];
  }
}
