import { NextRequest, NextResponse } from "next/server";
import config from "@/_config";

interface GoogleUserInfo {
  sub: string;
  name: string;
  given_name: string;
  family_name: string;
  picture: string;
  email: string;
  email_verified: boolean;
}

interface GoogleAccessToken {
  access_token: string;
  authuser: string;
  expires_in: number;
  prompt: string;
  scope: string;
  token_type: string;
}

interface LoginRequestBody {
  googleAccessToken: GoogleAccessToken;
}

interface BackendLoginResponse {
  jwt: string;
  email: string;
  role: string;
}

export async function POST(request: NextRequest) {
  try {
    const body: LoginRequestBody = await request.json();
    console.log("Received body:", body);
    const { googleAccessToken } = body;

    console.log("Google Access Token:", googleAccessToken);
    const userInfoResponse = await fetch(
      "https://www.googleapis.com/oauth2/v3/userinfo",
      {
        method: "GET",
        headers: {
          Accept: "application/json",
          Authorization: `Bearer ${googleAccessToken.access_token}`,
        },
      },
    );
    if (!userInfoResponse.ok) {
      console.error(
        "Failed to fetch user info from Google:",
        userInfoResponse.statusText,
      );
      return NextResponse.json(
        { error: "Failed to fetch user info" },
        { status: 400 },
      );
    }
    const userInfo: GoogleUserInfo = await userInfoResponse.json();
    console.log("User info from Google:", userInfo);

    const backendResponse: Response = await fetch(
      config.SERVER_API_ENDPOINT + "/auth/login",
      {
        method: "POST",
        body: JSON.stringify(userInfo),
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
      },
    );

    // Query backend for login token
    if (!backendResponse.ok) {
      console.error("Failed to sign in user:", backendResponse.statusText);
      // Read body as UTF-8 string
      const errorBody = await backendResponse.text();
      console.error("Backend error response:", errorBody);
      return NextResponse.json(
        { error: "Failed to sign in user" },
        { status: 400 },
      );
    }
    const backendResponseBody: BackendLoginResponse =
      await backendResponse.json();
    console.log("Backend response:", backendResponseBody);

    return NextResponse.json(
      {
        email: backendResponseBody.email,
        role: backendResponseBody.role,
        jwt: backendResponseBody.jwt,
      },
      {
        headers: {
          "Set-Cookie": `auth_token=${backendResponseBody.jwt}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=${60 * 60 * 24 * 30}`,
        },
      },
    );
  } catch (error) {
    console.error("Error signing in a user:", error);
    return NextResponse.json({ error: "Bad request" }, { status: 400 });
  }
}
