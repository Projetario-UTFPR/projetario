import { MagnifyingGlassIcon } from "@phosphor-icons/react/dist/ssr/MagnifyingGlass";
import clsx from "clsx";
import type { PropsWithChildren } from "react";

type RootProps = PropsWithChildren;

function Root({ children }: RootProps) {
  return (
    <div
      className={clsx(
        "self-stretch flex items-center px-4 py-2 rounded-3xl border border-black/20 dark:border-white/20 w-full",
        "ring-0 has-[input:focus-visible]:ring-4 ring-yellow-500/40 transition-all will-change-[box-shadow] duration-100",
      )}
    >
      {children}
    </div>
  );
}

type InputProps = {
  onInput: (value: string | null) => void;
};

function Input({ onInput }: InputProps) {
  return (
    <label className="flex items-center gap-2.5 w-full">
      <MagnifyingGlassIcon size={24} weight="bold" />
      <input
        type="text"
        placeholder="Encontre qualquer projeto"
        className={clsx("w-full outline-none")}
        onInput={(event) => onInput(event.currentTarget.value || null)}
      />
    </label>
  );
}

export default {
  Root,
  Input,
};
