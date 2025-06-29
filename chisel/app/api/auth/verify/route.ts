import { NextRequest, NextResponse } from "next/server";
import config from "@/_config";

export async function GET(request: NextRequest) {
  try {
    const cookie = request.cookies.get("auth_token");
    if (cookie === undefined) {
      console.debug("No cookie found");
      return NextResponse.json({
        status: 401,
        message: "No authorization cookie found",
      });
    }
    const backendResponse = await fetch(
      `${config.SERVER_API_ENDPOINT}/auth/verify?` +
        new URLSearchParams({
          auth_token: cookie.value,
        }),
    );
    if (!backendResponse.ok) {
      console.error("Backend return 401");
      return NextResponse.json({
        message: "Unauthorized",
        status: 401,
      });
    }
    const body = await backendResponse.json();
    const responseBody = { ...body, jwt: cookie.value };

    return NextResponse.json({
      success: true,
      body: responseBody,
    });
  } catch (error) {
    console.error("Error verifying user:", error);
    return NextResponse.json({ error: "Bad request" }, { status: 400 });
  }
}
