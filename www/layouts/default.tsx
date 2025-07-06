import type { PropsWithChildren } from "react";
import { Header } from "@/ui/header";

export function DefaultLayout({ children }: PropsWithChildren) {
  return (
    <>
      <Header />
      {children}
    </>
  );
}
