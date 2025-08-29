import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = {
  className?: string;
};

export function Main({ children, className }: PropsWithChildren<Props>) {
  return (
    <main
      className={clsx(
        "flex-1 mx-auto w-[calc(100%_-_48px)] max-w-lg h-auto",
        className && className,
      )}
    >
      {children}
    </main>
  );
}
