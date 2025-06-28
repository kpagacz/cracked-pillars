"use server";

export interface Tag {
  name: string;
  description: string;
}

export default async function fetchTags(): Promise<Tag[]> {
  try {
    const endpoint = `${process.env.SERVER_API_ENDPOINT}/tags`;
    console.log(`[DEBUG] fetchTags called with endpoint: ${endpoint}`);
    console.log(
      `[DEBUG] SERVER_API_ENDPOINT env var: ${process.env.SERVER_API_ENDPOINT}`,
    );

    const tags = await fetch(endpoint, { cache: "no-store" });
    console.log(`[DEBUG] fetchTags response status: ${tags.status}`);

    if (!tags.ok) {
      console.error(`[DEBUG] fetchTags failed with status: ${tags.status}`);
      return [];
    }

    const data = (await tags.json()) as Tag[];
    console.log(`[DEBUG] fetchTags successful, got ${data.length} tags`);
    return data;
  } catch (error) {
    console.error("[DEBUG] Error fetching tags:", error);
    console.error("[DEBUG] Error details:", JSON.stringify(error, null, 2));
    return [];
  }
}
