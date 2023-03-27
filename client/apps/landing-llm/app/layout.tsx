import type { Metadata } from "next";
import { KoHo } from "next/font/google";

import { Analytics } from "@vercel/analytics/react";

import "./globals.css";

const koho = KoHo({ weight: ["400", "500", "600", "700"], subsets: ["latin"] });

let title = "Mookbark";
let description = "Manage and talk to your bookmarks.";
let ogimage = "https://mookbark.run/og-image.png";
let url = "https://www.mookbark.run";
let sitename = "mookbark.run";
export const metadata: Metadata = {
  metadataBase: new URL(url),
  title,
  description,
  applicationName: "Mookbark",
  referrer: "origin-when-cross-origin",
  keywords: ["Bookmark", "Mookbark", "Manager", "AI", "Chat", "Talk"],
  authors: [{ name: "Furkan Kalaycioglu", url: "https://www.furkankly.dev" }],
  creator: "Furkan Kalaycioglu",
  publisher: "Furkan Kalaycioglu",
  openGraph: {
    images: [ogimage],
    title,
    description,
    url: url,
    siteName: sitename,
    locale: "en_US",
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    images: [ogimage],
    title,
    description,
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="h-full">
      <title>Mookbark</title>
      <body className={koho.className + " h-full bg-yellow-900"}>
        <Analytics />
        {children}
      </body>
    </html>
  );
}
