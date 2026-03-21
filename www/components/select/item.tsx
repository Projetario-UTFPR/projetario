import * as S from "@radix-ui/react-select";
import clsx from "clsx";

type Props = S.SelectItemProps & {
  containerClassName?: string;
};

export function SelectItem({ className, containerClassName, children, ...props }: Props) {
  return (
    <S.Item
      {...props}
      className={clsx(
        "py-1 px-2 rounded-xl relative overflow-hidden",
        "outline-none",
        "before:absolute before:inset-2 before:rounded-xl before:transition-all",
        "before:duration-100 before:bg-transparent",
        "hover:before:inset-0 hover:before:bg-yellow-500/50",
        "active:before:bg-yellow-500 focus:before:bg-yellow-500/50 focus:before:inset-0",
        "data-[state=checked]:before:bg-yellow-500 data-[state=checked]:before:inset-0",
        "dark:active:text-black data-[state=checked]:text-black",
        containerClassName,
      )}
    >
      <S.ItemText>
        <span className={clsx("flex items-center relative z-10 cursor-default", className)}>{children}</span>
      </S.ItemText>
    </S.Item>
  );
}
