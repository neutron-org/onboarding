"use client";

import { Counter } from "@/components/Counter";
import { WalletButton } from "@/components/WalletButton";

export default function Home() {
  return (
    <main className="flex flex-col gap-8 items-center justify-center h-screen">
      <WalletButton />
      <Counter />
    </main>
  );
}
