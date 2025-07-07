import * as Slot from "@radix-ui/react-slot";
import clsx from "clsx";
import type { ButtonProps } from ".";

export function Secundario({ asChild, className, ...props }: ButtonProps) {
  const Component = asChild ? Slot.Root : "button";

  return (
    <Component
      className={clsx(
        "transition-all duration-100 bg-gray-300 px-6 py-3 rounded-3xl font-medium",
        "hover:bg-gray-400 active:brightness-95",
        className && className,
      )}
      {...props}
    />
  );
}
