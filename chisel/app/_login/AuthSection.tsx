"use client";

import React, { useEffect, useState } from "react";
import { AuthContext, InnerAuth } from "./AuthContext";
import config from "@/_config";
import { AuthSectionFallback } from "./AuthSectionFallback";
import { LoggedIn } from "./LoggedIn";
import GoogleLoginWrapper from "./GoogleLoginWrapper";

export function AuthSection() {
  const authContext = React.useContext(AuthContext);

  const [isLoading, setIsLoading] = useState(true);
  const [showLoggedIn, setShowLoggedIn] = useState(false);
  const [isFallbackExiting, setIsFallbackExiting] = useState(false);

  useEffect(() => {
    setIsLoading(true);

    fetch(`${config.NEXTJS_API_URL}/auth/verify`)
      .then((response) => {
        if (!response.ok) {
          throw "Response did not return ok";
        }
        return response.json();
      })
      .then((data) => {
        const authData = data.body as InnerAuth;
        authContext.setAuthContext(authData);
        setIsLoading(false);
      })
      .catch((error) => {
        console.error("Error while fetching auth verify ", error);
        setIsLoading(false);
      });
  }, []);

  useEffect(() => {
    if (authContext.authContext?.email && !isLoading) {
      setIsFallbackExiting(true);

      const timer = setTimeout(() => {
        setShowLoggedIn(true);
      }, 400);

      return () => clearTimeout(timer);
    } else {
      setShowLoggedIn(false);
      setIsFallbackExiting(false);
    }
  }, [authContext.authContext?.email, isLoading]);

  if (isLoading || (!showLoggedIn && authContext.authContext?.email)) {
    return (
      <div
        className={`${
          isFallbackExiting
            ? 'translate-x-[200%] opacity-0'
            : 'translate-x-0 opacity-100'
        }`}
        style={{
          transition: 'all 400ms cubic-bezier(0.4, 0.0, 0.2, 1)'
        }}
      >
        <AuthSectionFallback />
      </div>
    );
  } else if (authContext.authContext?.email) {
    return (
      <div
        className={`${
          showLoggedIn
            ? 'translate-x-0 opacity-100'
            : 'translate-x-[100%] opacity-0'
        }`}
        style={{
          transition: 'all 400ms cubic-bezier(0.4, 0.0, 0.2, 1)'
        }}
      >
        <LoggedIn />
      </div>
    );
  } else {
    return <GoogleLoginWrapper />;
  }
}
