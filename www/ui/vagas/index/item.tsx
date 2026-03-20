import * as Toggle from "@radix-ui/react-toggle";
import clsx from "clsx";
import type { TipoDeProjeto } from "@/core/types/enums/tipo-de-projeto";

type ItemProps = Omit<Toggle.ToggleProps, "value"> & {
  value: TipoDeProjeto;
  toggle: (value: TipoDeProjeto, pressed: boolean) => void;
};

export function Item({ value, children, toggle, ...props }: ItemProps) {
  return (
    <Toggle.Root
      value={value}
      {...props}
      className={clsx(
        "text-base font-regular underline capitalize leading-none transition-all duration-100",
        "px-2.5 py-1.25 rounded-full",

        "data-[state=off]:not-dark:bg-gray-200 data-[state=on]:not-dark:bg-yellow-500",
        "data-[state=off]:not-dark:hover:bg-gray-300 data-[state=off]:not-dark:active:bg-gray-400",
        "data-[state=on]:not-dark:hover:bg-yellow-600 data-[state=on]:not-dark:active:brightness-95",

        "data-[state=on]:dark:text-yellow-500 data-[state=off]:dark:text-gray-300",
        "data-[state=on]:dark:bg-yellow-500/10 data-[state=off]:dark:bg-gray-500/20",
        "data-[state=off]:dark:hover:bg-gray-500/30 data-[state=off]:dark:active:bg-gray-500/40",
        "data-[state=on]:dark:hover:bg-yellow-500/15 data-[state=on]:dark:active:bg-yellow-500/25",
      )}
      onPressedChange={(pressed) => toggle(value, pressed)}
    >
      {children}
    </Toggle.Root>
  );
}
