import type { Icon } from "@phosphor-icons/react/dist/lib/types";
import { ListDashesIcon } from "@phosphor-icons/react/dist/ssr/ListDashes";
import { SquaresFourIcon } from "@phosphor-icons/react/dist/ssr/SquaresFour";
import * as RadioGroup from "@radix-ui/react-radio-group";
import clsx from "clsx";
import { useId } from "react";

type Value = "horizontal" | "vertical";

type Props = {
  onValueChange?: (direcao: Value) => void;
  defaultValue?: Value;
};

export function DirecaoDosCardsDeVagas({ onValueChange, defaultValue }: Props) {
  return (
    <RadioGroup.Root
      required
      defaultValue={defaultValue}
      onValueChange={(value: Value) => onValueChange?.(value)}
      className="flex p-px rounded-full items-center justify-center bg-gray-300 dark:bg-gray-800"
    >
      <Item value="horizontal" icon={ListDashesIcon} label="Exibir vagas na horizontal" />
      <Item value="vertical" icon={SquaresFourIcon} label="Exibir vagas na vertical" />
    </RadioGroup.Root>
  );
}

function Item({ value, icon: I, label }: { value: Value; icon: Icon; label: string }) {
  const id = useId();
  return (
    <RadioGroup.Item
      id={id}
      value={value}
      title={label}
      className={clsx(
        "px-4 py-2 rounded-full transition-all duration-75 bg-transparent not-first:-ml-2",
        "data-[state=unchecked]:hover:bg-black/5 data-[state=unchecked]:dark:hover:bg-white/5",
        "data-[state=unchecked]:active:bg-black/10 data-[state=unchecked]:dark:active:bg-white/10",
        "dark:text-white data-[state=checked]:dark:bg-gray-600",
        "text-black data-[state=checked]:bg-gray-200",
        "relative z-0 data-[state=checked]:z-10",
      )}
    >
      <I size={24} weight="bold" />
      <label htmlFor={id} className="sr-only">
        {label}
      </label>
    </RadioGroup.Item>
  );
}
