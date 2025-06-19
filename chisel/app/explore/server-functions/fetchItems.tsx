"use server";

export interface Item {
  name: string;
  tags: string[];
  wiki_url: string;
}

export async function fetchItemsByTags(tags: string[]): Promise<Item[]> {
  const searchParams = new URLSearchParams();
  tags.forEach((tag) => searchParams.append("tags", tag));
  searchParams.append("per_page", "1000");
  let abilitiesApi = `http://${process.env.NEXT_PUBLIC_API_URL}/abilities?${searchParams}`;
  try {
    const response = await fetch(abilitiesApi);
    const data = await response.json();
    return data.data as Item[];
  } catch (error) {
    console.error("Error fetching items by tags: ", error);
    return [];
  }
}
