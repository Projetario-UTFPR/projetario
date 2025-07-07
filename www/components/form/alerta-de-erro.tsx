import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = PropsWithChildren<{
  className?: string;
}>;
export function AlertaDeErro({ children, className }: Props) {
  return (
    <span
      className={clsx(
        "px-3 py-1.5 rounded-2xl text-red-500 bg-red-500/5 block",
        className && className,
      )}
    >
      {children}
    </span>
  );
}
