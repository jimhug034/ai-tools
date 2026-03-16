import type { Metadata } from "next";
import { NextIntlClientProvider } from "next-intl";
import { getMessages } from "next-intl/server";
import { Link } from "@/lib/navigation";
import { Inter, Inter_Tight, JetBrains_Mono } from "next/font/google";
import "../globals.css";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
  display: "swap",
});

const interTight = Inter_Tight({
  subsets: ["latin"],
  variable: "--font-inter-tight",
  display: "swap",
});

const jetbrainsMono = JetBrains_Mono({
  subsets: ["latin"],
  variable: "--font-jetbrains-mono",
  display: "swap",
});

export const metadata: Metadata = {
  title: "YouTube Transcript Tool",
  description: "Get instant transcripts from YouTube videos",
};

export default async function LocaleLayout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ lang: string }>;
}) {
  const { lang: locale } = await params;
  const messages = await getMessages();

  return (
    <html
      lang={locale}
      className={`${inter.variable} ${interTight.variable} ${jetbrainsMono.variable}`}
    >
      <body>
        <NextIntlClientProvider messages={messages}>
          <div className="min-h-screen bg-background">
            <header className="border-b border-neutral-200 dark:border-neutral-800">
              <div className="container mx-auto px-4 py-4 flex items-center justify-between">
                <Link href="/" className="flex items-center gap-2 hover:opacity-80 transition-opacity">
                  <div className="w-8 h-8 bg-primary rounded-md flex items-center justify-center">
                    <span className="text-primary-foreground font-semibold text-sm">YT</span>
                  </div>
                  <span className="font-semibold">Transcript</span>
                </Link>
                <nav className="flex gap-1" aria-label="Language selector">
                  <Link
                    href="/"
                    locale="en"
                    className={`px-3 py-2 text-sm transition-colors relative ${
                      locale === "en" ? "text-foreground font-medium" : "text-neutral-500 hover:text-foreground"
                    }`}
                    aria-label="Switch to English"
                    aria-current={locale === "en" ? "true" : undefined}
                  >
                    EN
                    {locale === "en" && <span className="absolute bottom-0 left-0 right-0 h-0.5 bg-primary" />}
                  </Link>
                  <Link
                    href="/"
                    locale="zh"
                    className={`px-3 py-2 text-sm transition-colors relative ${
                      locale === "zh" ? "text-foreground font-medium" : "text-neutral-500 hover:text-foreground"
                    }`}
                    aria-label="切换到中文"
                    aria-current={locale === "zh" ? "true" : undefined}
                  >
                    中
                    {locale === "zh" && <span className="absolute bottom-0 left-0 right-0 h-0.5 bg-primary" />}
                  </Link>
                </nav>
              </div>
            </header>
            <main className="container mx-auto px-4 py-8">{children}</main>
            <footer className="border-t border-neutral-200 dark:border-neutral-800 mt-16">
              <div className="container mx-auto px-4 py-6 flex items-center justify-between text-sm text-neutral-500">
                <p>Free • No registration</p>
                <p>Built with Next.js & Rust</p>
              </div>
            </footer>
          </div>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
