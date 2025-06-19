import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Link from "next/link";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Cracked Pillars",
  description: "Discover the secrets of Pillars of Eternity II: Deadfire",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="bg-gradient-to-br from-primary via-secondary to-accent min-h-screen">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased text-text min-h-screen`}
      >
        <div className="min-h-screen flex flex-col">
          <header className="bg-secondary/80 backdrop-blur-sm border-b border-border/50 sticky top-0 z-50">
            <nav className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
              <div className="flex justify-between items-center h-16">
                <div className="flex flex-row items-center space-x-4">
                  <h1 className="text-2xl font-bold text-highlight">
                    Cracked Pillars
                  </h1>
                  <span className="text-text-muted text-sm hidden sm:block">
                    Pillars of Eternity II: Deadfire
                  </span>
                </div>
                <div className="flex items-center space-x-4">
                  <Link
                    href="/"
                    className="text-text hover:text-highlight transition-colors duration-200 px-3 py-2 rounded-md text-sm font-medium"
                  >
                    Home
                  </Link>
                  <Link
                    href="/explore"
                    className="bg-highlight text-white hover:bg-highlight/90 transition-colors duration-200 px-4 py-2 rounded-md text-sm font-medium"
                  >
                    Explore
                  </Link>
                </div>
              </div>
            </nav>
          </header>
          <main className="flex-1">{children}</main>
          <footer className="bg-secondary/60 border-t border-border/50 py-6">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
              <div className="text-center text-text-muted text-sm">
                <p>
                  © 2025 Cracked Pillars. Discover the secrets of Deadfire.
                </p>
              </div>
            </div>
          </footer>
        </div>
      </body>
    </html>
  );
}
