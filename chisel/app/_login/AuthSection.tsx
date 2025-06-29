"use client";

import React from "react";
import { AuthContext } from "./AuthContext";
import { LoggedIn } from "./LoggedIn";
import GoogleLoginWrapper from "./GoogleLoginWrapper";

export function AuthSection() {
  const { authContext } = React.useContext(AuthContext);

  if (authContext?.jwt) {
    return <LoggedIn />;
  } else {
    return <GoogleLoginWrapper />;
  }
}
