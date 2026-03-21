import * as S from "@radix-ui/react-select";
import clsx from "clsx";

type Props = S.SelectContentProps;

export function SelectContent({
  position = "popper",
  side = "bottom",
  align = "center",
  sideOffset = 4,
  alignOffset = 12,
  sticky = "partial",
  collisionPadding = 24,
  children,
  ...props
}: Props) {
  return (
    <S.Portal>
      <S.Content
        {...props}
        position={position}
        side={side}
        align={align}
        sideOffset={sideOffset}
        alignOffset={alignOffset}
        sticky={sticky}
        collisionPadding={collisionPadding}
        className={clsx(
          "overflow-hidden bg-white dark:bg-gray-800 shadow-md rounded-2xl border border-black/10",
          "w-(--radix-select-trigger-width)",
          "data-[side=bottom]:animate-slide-up-and-fade data-[side=left]:animate-slide-right-and-fade",
          "data-[side=right]:animate-slide-left-and-fade data-[side=top]:animate-slide-down-and-fade",
        )}
      >
        {children}
      </S.Content>
    </S.Portal>
  );
}
