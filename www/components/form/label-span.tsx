import type { PropsWithChildren } from "react";

type Props = PropsWithChildren<{
  className?: string;
  required?: boolean;
}>;

export function InputLabelSpan({ children, className, required }: Props) {
  return (
    <span className={className}>
      {children}
      {required && <sup className="text-red-500">*</sup>}
    </span>
  );
}
