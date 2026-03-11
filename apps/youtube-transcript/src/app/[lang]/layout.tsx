import {Link} from '@/lib/navigation';

export default function LocaleLayout({
  children,
  params: {locale}
}: {
  children: React.ReactNode;
  params: {locale: string};
}) {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-4 py-4 flex items-center justify-between">
          <h1 className="text-xl font-semibold">
            📺 <span className="hidden sm:inline">YouTube Transcript Tool</span>
          </h1>
          <div className="flex gap-2">
            <Link
              href="/"
              locale="en"
              className={`px-3 py-1 rounded text-sm ${locale === 'en' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}`}
            >
              EN
            </Link>
            <Link
              href="/"
              locale="zh"
              className={`px-3 py-1 rounded text-sm ${locale === 'zh' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}`}
            >
              中
            </Link>
          </div>
        </div>
      </header>
      <main className="container mx-auto px-4 py-8">
        {children}
      </main>
      <footer className="border-t mt-12">
        <div className="container mx-auto px-4 py-6 text-center text-sm text-muted-foreground">
          <p>100% Free • No Registration Required</p>
        </div>
      </footer>
    </div>
  );
}
