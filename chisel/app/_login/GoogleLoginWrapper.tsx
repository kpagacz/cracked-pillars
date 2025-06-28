import {
  GoogleOAuthProvider,
  TokenResponse,
  useGoogleLogin,
} from "@react-oauth/google";
import config from "../_config";
import React, { useState } from "react";
import { AuthContext, InnerAuth } from "./AuthContext";
import { ErrorBanner } from "./ErrorBanner";

function CustomGoogleButton() {
  const authContext = React.useContext(AuthContext);
  const [showError, setShowError] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");

  const login = useGoogleLogin({
    onSuccess: (tokenResponse: TokenResponse) => {
      fetch(config.NEXTJS_API_URL + "/auth/login", {
        method: "POST",
        body: JSON.stringify({
          googleAccessToken: tokenResponse,
        }),
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
          }
          return response.json();
        })
        .then((data: InnerAuth) => {
          authContext.setAuthContext(data);
        })
        .catch((error) => {
          console.error("Login error:", error);
          setErrorMessage("Failed to sign in. Please try again.");
          setShowError(true);
        });
    },
    onError: (error) => {
      console.error("Google OAuth error:", error);
      setErrorMessage("Sign-in was cancelled or failed. Please try again.");
      setShowError(true);
    },
    flow: "implicit",
    scope:
      "https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email",
  });

  return (
    <>
      <button
        onClick={() => login()}
        className="w-25 h-10 border border-border rounded-lg bg-primary hover:bg-secondary hover:border-highlight text-text font-medium text-sm shadow-sm hover:shadow-md hover:-translate-y-0.5 active:translate-y-0 active:shadow-sm focus:outline-none focus:ring-2 focus:ring-highlight/20 focus:border-highlight transition-all duration-200 ease-in-out flex items-center justify-center"
      >
        <div className="flex items-center gap-2">
          <div className="w-[18px] h-[18px] bg-gradient-to-br from-blue-500 via-yellow-500 to-red-500 rounded-sm flex items-center justify-center shadow-sm">
            <span className="text-white text-xs font-bold drop-shadow-sm">
              G
            </span>
          </div>
          <span className="text-text font-medium">Sign in</span>
        </div>
      </button>

      <ErrorBanner
        message={errorMessage}
        type="error"
        isVisible={showError}
        onClose={() => setShowError(false)}
        autoHide={true}
        autoHideDelay={5000}
      />
    </>
  );
}

export default function GoogleLoginWrapper() {
  return (
    <GoogleOAuthProvider clientId={config.GOOGLE_CLIENT_ID}>
      <CustomGoogleButton />
    </GoogleOAuthProvider>
  );
}
