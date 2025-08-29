import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = PropsWithChildren<{
  className?: string;
}>;
export function AlertaDeErro({ children, className }: Props) {
  return (
    <span
      className={clsx(
        "px-3 py-1.5 rounded-2xl text-red-500 dark:text-red-300 bg-red-500/5 dark:bg-red-500/10 block",
        className && className,
      )}
    >
      {children}
    </span>
  );
}
