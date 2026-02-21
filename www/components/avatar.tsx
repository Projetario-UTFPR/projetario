import * as PAvatar from "@radix-ui/react-avatar";
import clsx from "clsx";

type Props = {
  nomeDoUsuario: string;
  imagem?: string;
  className?: string;
};

export function Avatar({ nomeDoUsuario, imagem, className }: Props) {
  const iniciaisDoNomeDoUsuario = nomeDoUsuario
    .split(" ")
    .map((nome) => nome[0])
    .join("");

  return (
    <PAvatar.Root
      className={clsx(
        "size-10 rounded-full select-none outline-none overflow-hidden",
        "drop-shadow-black/5 drop-shadow-md",
        className,
      )}
    >
      <PAvatar.Image src={imagem} />
      <PAvatar.Fallback
        className={clsx(
          "leading-1 flex size-full items-center justify-center",
          "bg-slate-600 text-[15px] font-medium text-white",
        )}
      >
        {iniciaisDoNomeDoUsuario}
      </PAvatar.Fallback>
    </PAvatar.Root>
  );
}
