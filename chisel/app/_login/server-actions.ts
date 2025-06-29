"use server";

import { cookies } from "next/headers";
import config from "@/_config";
import { InnerAuth } from "./AuthContext";

export async function verifyAuthServer(): Promise<InnerAuth | null> {
  try {
    const cookieStore = await cookies();
    const authToken = cookieStore.get("auth_token");

    if (!authToken) {
      return null;
    }

    const backendResponse = await fetch(
      `${config.SERVER_API_ENDPOINT}/auth/verify?` +
        new URLSearchParams({
          auth_token: authToken.value,
        }),
      {
        // Add timeout and better error handling
        signal: AbortSignal.timeout(5000), // 5 second timeout
      }
    );

    if (!backendResponse.ok) {
      return null;
    }

    const body = await backendResponse.json();
    return { ...body, jwt: authToken.value };
  } catch (error) {
    console.error("Error verifying auth on server:", error);
    return null;
  }
}
