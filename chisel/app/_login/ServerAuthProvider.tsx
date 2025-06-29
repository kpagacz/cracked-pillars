import { verifyAuthServer } from "./server-actions";
import { AuthContextProvider } from "./AuthContext";

export async function ServerAuthProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const initialAuth = await verifyAuthServer();

  return (
    <AuthContextProvider initialAuth={initialAuth}>
      {children}
    </AuthContextProvider>
  );
}
