import Link from "next/link";

export default function NotFound() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary via-secondary to-accent px-4">
      <div className="text-center max-w-md">
        <div className="text-8xl mb-6">🔍</div>
        <h1 className="text-4xl font-bold text-highlight mb-4">Page Not Found</h1>
        <p className="text-text-muted mb-8">
          The page you&apos;re looking for doesn&apos;t exist. It might have been moved or deleted.
        </p>
        <div className="space-y-4">
          <Link
            href="/"
            className="inline-block bg-highlight hover:bg-highlight/90 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-200 transform hover:scale-105"
          >
            Go Home
          </Link>
          <div className="text-text-muted text-sm">
            Or try exploring our <Link href="/explore" className="text-highlight hover:underline">item collection</Link>
          </div>
        </div>
      </div>
    </div>
  );
}
