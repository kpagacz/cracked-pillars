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
  initialAuth = null,
}: {
  children: React.ReactNode;
  initialAuth?: InnerAuth | null;
}) {
  const [authContext, setAuthContext] = useState<null | InnerAuth>(initialAuth);

  return (
    <AuthContext.Provider value={{ authContext, setAuthContext }}>
      {children}
    </AuthContext.Provider>
  );
}
