export default function Home() {
  return (
    <>
      <div>Welcome to Cracked Pillars</div>
      <NavButton href="/explore">Explore</NavButton>
    </>
  );
}

function NavButton({
  href,
  children,
}: {
  href: string;
  children: React.ReactNode;
}) {
  return (
    <>
      <a href={href}>{children}</a>
    </>
  );
}
