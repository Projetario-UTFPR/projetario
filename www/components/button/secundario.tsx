import * as Slot from "@radix-ui/react-slot";
import clsx from "clsx";
import type { ButtonProps } from ".";

export function Secundario({ asChild, className, ...props }: ButtonProps) {
  const Component = asChild ? Slot.Root : "button";

  return (
    <Component
      className={clsx(
        "flex items-center gap-2 disabled:opacity-25",
        "transition-all duration-100 bg-gray-300 px-6 py-3 rounded-3xl font-medium",
        "not-dark:hover:bg-gray-400 active:brightness-95",
        "dark:bg-gray-800 dark:text-white dark:hover:brightness-95 dark:active:brightness-90",
        className && className,
      )}
      {...props}
    />
  );
}
