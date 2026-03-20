import { Link } from "@inertiajs/react";
import clsx from "clsx";
import { Avatar } from "@/components/avatar";
import type { PreviewDeVaga } from "@/core/types/entidades/preview-de-vaga";
import { Rotas } from "@/rotas";
import { PrimeiroParagrafoDoCorpoClientSide } from "./primeiro-paragrafo-client-side";

type Props = { vaga: PreviewDeVaga; className?: string };

export function CardDePreviewDeVagaVertical({ vaga, className }: Props) {
  return (
    <article className={clsx("container-box px-5 py-4 flex flex-col gap-6 shadow-none", className)}>
      <img
        src={vaga.imagem}
        alt={`Capa do projeto ${vaga.titulo}`}
        className="rounded-2xl select-none"
        draggable={false}
      />

      <div>
        <span className="text-xl font-medium block">{vaga.titulo}</span>

        <span
          className={clsx(
            "block mb-4 text-base font-medium",
            vaga.tipoDoProjeto === "Extensao" && "text-lime-600 dark:text-lime-500",
            vaga.tipoDoProjeto === "IniciacaoCientifica" && "text-sky-600 dark:text-sky-500",
          )}
        >
          {vaga.tipoDoProjeto}
        </span>

        <PrimeiroParagrafoDoCorpoClientSide className="mb-6" html={vaga.conteudo} />

        <div>
          <Link href={Rotas.usuarios.perfil(vaga.coordenador.id)} className="flex items-center gap-3">
            <Avatar nomeDoUsuario={vaga.coordenador.nome} />
            <span>{vaga.coordenador.nome}</span>
          </Link>
        </div>
      </div>
    </article>
  );
}

export namespace CardDePreviewDeVagaVertical {
  export function Skeleton() {
    const nbsp = "\u00A0";

    return (
      <div inert className="container-box px-5 py-4 flex flex-col gap-6 shadow-none items-start">
        <div className="rounded-2xl h-48 dark:bg-white/5 animate-pulse w-full" />

        <div>
          <div className="text-xl font-medium wrap-anywhere dark:bg-white/10 animate-pulse text-transparent rounded-2xl mb-2">
            {nbsp.repeat(70)}
          </div>

          <span className="mb-4 rounded-full text-base font-medium text-transparent bg-white/5">
            {nbsp.repeat(25)}
          </span>
        </div>

        <div className="flex flex-col gap-1 w-full">
          <div className="text-transparent bg-white/5 rounded-full w-full">{nbsp}</div>
          <div className="text-transparent bg-white/5 rounded-full w-4/5">{nbsp}</div>
          <div className="text-transparent bg-white/5 rounded-full w-full">{nbsp}</div>
          <div className="text-transparent bg-white/5 rounded-full w-1/3">{nbsp}</div>
        </div>

        <div className="flex items-center gap-3">
          <div className="rounded-full size-10 dark:bg-slate-700 animate-pulse"></div>
          <div className="text-transparent rounded-full bg-white/10">{nbsp.repeat(40)}</div>
        </div>
      </div>
    );
  }
}
