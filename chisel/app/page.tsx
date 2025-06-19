import Link from "next/link";

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col">
      {/* Hero Section */}
      <section className="flex-1 items-center justify-center px-4 sm:px-6 lg:px-8">
        <div className="max-w-4xl mx-auto text-center animate-fade-in">
          <div className="mb-8">
            <h1 className="text-5xl md:text-7xl font-bold text-highlight mb-6">
              Cracked Pillars
            </h1>
            <p className="text-xl md:text-2xl text-text-muted mb-8 max-w-3xl mx-auto">
              Discover synergistic items and abilities
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
              <Link
                href="/explore"
                className="bg-highlight hover:bg-highlight/90 text-white font-semibold py-4 px-8 rounded-lg text-lg transition-all duration-200 transform hover:scale-105 shadow-lg hover:shadow-xl"
              >
                Explore effects
              </Link>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
