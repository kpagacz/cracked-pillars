"use server";

export default async function fetchTags(): Promise<string[]> {
  try {
    const tags = await fetch(`http://${process.env.NEXT_PUBLIC_API_URL}/tags`);
    const data = (await tags.json()) as string[];
    return data;
  } catch (error) {
    console.error("Error fetching tags:", error);
    return [];
  }
}
