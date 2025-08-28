import type { PropsWithChildren } from "react";
import { ToastContainer } from "react-toastify";
import { Footer } from "@/ui/footer";
import { Header } from "@/ui/header";

export function DefaultLayout({ children }: PropsWithChildren) {
  return (
    <>
      <Header />
      {children}
      <Footer />
      <ToastContainer />
    </>
  );
}
