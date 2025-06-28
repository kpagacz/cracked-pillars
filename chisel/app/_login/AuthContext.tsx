"use client";

import { createContext, useState } from "react";

export interface InnerAuth {
  email: string;
  role: "Admin" | "Editor" | "Viewer";
  jwt: string;
}

interface AuthContextType {
  authContext: null | InnerAuth;
  setAuthContext: React.Dispatch<React.SetStateAction<null | InnerAuth>>;
}

export const AuthContext = createContext<AuthContextType>({
  authContext: null,
  setAuthContext: () => {},
});

export function AuthContextProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [authContext, setAuthContext] = useState<null | InnerAuth>(null);

  return (
    <AuthContext value={{ authContext, setAuthContext }}>
      {children}
    </AuthContext>
  );
}
