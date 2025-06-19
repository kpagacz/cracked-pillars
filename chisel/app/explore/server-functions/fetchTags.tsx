"use server";

export default async function (): Promise<string[]> {
  try {
    let tags = await fetch(`http://${process.env.NEXT_PUBLIC_API_URL}/tags`);
    let data = (await tags.json()) as string[];
    return data;
  } catch (error) {
    console.error("Error fetching tags:", error);
    return [];
  }
}
