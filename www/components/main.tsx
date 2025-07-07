import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = {
  className?: string;
};

export function Main({ children, className }: PropsWithChildren<Props>) {
  return (
    <main
      className={clsx(
        "mx-auto w-[calc(100%_-_48px)] max-w-lg",
        className && className,
      )}
    >
      {children}
    </main>
  );
}
