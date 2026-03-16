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
            <header className="border-b">
              <div className="container mx-auto px-4 py-4 flex items-center justify-between">
                <h1 className="text-xl font-semibold">
                  📺 <span className="hidden sm:inline">YouTube Transcript Tool</span>
                </h1>
                <div className="flex gap-2">
                  <Link
                    href="/"
                    locale="en"
                    className={`px-3 py-1 rounded text-sm ${locale === "en" ? "bg-primary text-primary-foreground" : "hover:bg-muted"}`}
                  >
                    EN
                  </Link>
                  <Link
                    href="/"
                    locale="zh"
                    className={`px-3 py-1 rounded text-sm ${locale === "zh" ? "bg-primary text-primary-foreground" : "hover:bg-muted"}`}
                  >
                    中
                  </Link>
                </div>
              </div>
            </header>
            <main className="container mx-auto px-4 py-8">{children}</main>
            <footer className="border-t mt-12">
              <div className="container mx-auto px-4 py-6 text-center text-sm text-muted-foreground">
                <p>100% Free • No Registration Required</p>
              </div>
            </footer>
          </div>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
