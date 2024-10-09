import { CosmosKitProvider } from "@/contexts/CosmosKit";
import { QueryProvider } from "@/contexts/Query";
import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Minimal UI",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <QueryProvider>
          <CosmosKitProvider>
            {children}
          </CosmosKitProvider>
        </QueryProvider>
      </body>
    </html>
  );
}
