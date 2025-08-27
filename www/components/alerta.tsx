import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = PropsWithChildren<{
  className?: string;
}>;
export function Alerta({ children, className }: Props) {
  return (
    <span
      className={clsx(
        "px-3 py-1.5 rounded-2xl text-yellow-800 bg-yellow-500/10 block",
        className && className,
      )}
    >
      {children}
    </span>
  );
}
