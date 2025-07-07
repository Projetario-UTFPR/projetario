import type { PropsWithChildren } from "react";

export function AlertaDeErro({ children }: PropsWithChildren) {
  return (
    <span className="px-3 py-1.5 rounded-2xl text-red-500 bg-red-500/5 block">
      {children}
    </span>
  );
}
