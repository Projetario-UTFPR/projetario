import * as Slot from "@radix-ui/react-slot";
import clsx from "clsx";
import type { ButtonProps } from ".";

export function CallToAction({ asChild, className, ...props }: ButtonProps) {
  const Component = asChild ? Slot.Root : "button";

  return (
    <Component
      {...props}
      className={clsx(
        "flex items-center gap-2",
        "transition-all duration-100 bg-yellow-500 px-6 py-3 rounded-3xl font-medium",
        "hover:brightness-95 active:brightness-90",
        "disabled:saturate-0",
        className && className,
      )}
    />
  );
}
