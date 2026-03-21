import type { Icon } from "@phosphor-icons/react/dist/lib/types";
import { CaretDownIcon } from "@phosphor-icons/react/dist/ssr/CaretDown";
import * as S from "@radix-ui/react-select";
import clsx from "clsx";

type Props = Omit<S.SelectTriggerProps, "children"> & {
  placeholder: string;
  icon?: Icon;
};

export function SelectTrigger({ className, placeholder, icon: I, ...props }: Props) {
  return (
    <S.Trigger
      {...props}
      className={clsx("group text-input leading-0 flex items-center justify-between gap-2.5", className)}
    >
      {I && <I weight="bold" size={24} />}
      <S.Value placeholder={placeholder} className="placeholder:text-gray-300" />
      <S.Icon>
        <CaretDownIcon
          size={16}
          weight="bold"
          className="transition-all text-gray-500 group-data-[state=open]:rotate-180"
        />
      </S.Icon>
    </S.Trigger>
  );
}
