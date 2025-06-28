"use server";
import config from "@/_config";

import { Item } from "./fetchItems";

export async function updateTagsInItem(item: Item, jwt: string): Promise<Item> {
  const itemOrAbility = { item: "items", ability: "abilities" }[item.type];
  try {
    const response = await fetch(
      `${config.SERVER_API_ENDPOINT}/${itemOrAbility}/${item.slug}/tags`,
      {
        method: "PATCH",
        headers: {
          Authorization: `Bearer ${jwt}`,
          "Content-Type": "application/json",
          Accept: "application/json",
        },
        body: JSON.stringify(item.tags),
      },
    );
    if (!response.ok) {
      console.error("Error when updating tags in an item");
      return item;
    }

    let body = await fetch(
      `${config.SERVER_API_ENDPOINT}/${itemOrAbility}/${item.slug}`,
      {
        method: "GET",
        headers: {
          Accept: "application/json",
        },
      },
    );
    if (!body.ok) {
      console.error(
        "Error when fetching updated item after updating tags in an item",
      );
      return item;
    }
    let rawItem = await body.json();
    return {
      name: rawItem.name,
      slug: rawItem.slug,
      tags: rawItem.tags,
      wiki_url: rawItem.wiki_url,
      type: "item",
    };
  } catch (err) {
    console.error("Error when updating tags in an item: ", err);
    return item;
  }
}
