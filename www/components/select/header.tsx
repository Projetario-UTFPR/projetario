import * as S from "@radix-ui/react-select";

type Props = {
  title: string;
};

export function SelectHeader({ title }: Props) {
  return (
    <>
      <S.Label className="mx-2 text-gray-600 dark:text-gray-300">{title}</S.Label>
      <S.Separator className="bg-gray-300 dark:bg-white/10 h-px m-2" />
    </>
  );
}
