import type { PropsWithChildren } from "react";
import { ToastContainer } from "react-toastify";
import { Header } from "@/ui/header";

export function DefaultLayout({ children }: PropsWithChildren) {
  return (
    <>
      <Header />
      {children}
      <ToastContainer />
    </>
  );
}
