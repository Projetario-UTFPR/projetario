import clsx from "clsx";
import type { PropsWithChildren } from "react";

type Props = PropsWithChildren<{ className?: string }>;

export function SelectContainer({ children, className }: Props) {
  return <div className={clsx("p-2 flex flex-col gap-0.5", className)}>{children}</div>;
}
