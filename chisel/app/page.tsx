import Link from "next/link";

export default function Home() {
  return (
    <section className="min-h-screen flex flex-col justify-center center px-4 sm:px-6 lg:px-8">
      <div className="max-w-4xl mx-auto text-center animate-fade-in">
        <h1 className="text-5xl md:text-7xl font-bold text-highlight mb-6">
          Cracked Pillars
        </h1>
        <p className="text-xl md:text-2xl text-text-muted mb-8 max-w-3xl mx-auto">
          Discover synergistic items and abilities
        </p>
        <Link
          href="/explore"
          className="bg-highlight hover:bg-highlight/90 text-white font-semibold py-4 px-8 rounded-lg text-lg transition-all duration-200 transform hover:scale-105 shadow-lg hover:shadow-xl"
        >
          Explore effects
        </Link>
      </div>
    </section>
  );
}
