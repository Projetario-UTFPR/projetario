import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = PropsWithChildren<{
  className?: string;
}>;

export function H1({ children, className }: Props) {
  return (
    <h1 className={clsx("text-3xl font-medium", className && className)}>
      {children}
    </h1>
  );
}
