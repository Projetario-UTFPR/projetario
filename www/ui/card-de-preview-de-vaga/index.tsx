import { Link } from "@inertiajs/react";
import clsx from "clsx";
import { Avatar } from "@/components/avatar";
import type { PreviewDeVaga } from "@/core/types/entidades/preview-de-vaga";
import { Rotas } from "@/rotas";
import { PrimeiroParagrafoDoCorpoClientSide } from "./primeiro-paragrafo-client-side";

type CardDeVagaProps = { vaga: PreviewDeVaga; className?: string; direcao?: "horizontal" | "vertical" };

export function CardDePreviewDeVaga({ vaga, className, direcao = "vertical" }: CardDeVagaProps) {
  return (
    <article
      className={clsx(
        "container-box px-5 py-4 gap-6 shadow-none",
        direcao === "vertical" && "flex flex-col",
        direcao === "horizontal" && "grid grid-cols-2",
        className,
      )}
    >
      <img
        src={vaga.imagem}
        alt={`Capa do projeto ${vaga.titulo}`}
        className="rounded-2xl select-none object-cover w-full"
        draggable={false}
      />

      <div className="flex flex-col gap-6">
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
      </div>
    </article>
  );
}

export namespace CardDePreviewDeVaga {
  type SkeletonProps = { className?: string; direcao?: CardDeVagaProps["direcao"] };

  export function Skeleton({ className, direcao = "vertical" }: SkeletonProps) {
    const nbsp = "\u00A0";

    return (
      <div
        inert
        className={clsx(
          "container-box px-5 py-4 gap-6 shadow-none items-start",
          direcao === "vertical" && "flex flex-col",
          direcao === "horizontal" && "grid grid-cols-2",
          className,
        )}
      >
        <div
          className={clsx(
            "rounded-2xl h-48 bg-black/5 dark:bg-white/5 animate-pulse w-full",
            direcao === "horizontal" && "h-full",
          )}
        />

        <div className="flex flex-col gap-6">
          <div>
            <div className="text-xl font-medium wrap-anywhere bg-black/10 dark:bg-white/10 animate-pulse text-transparent rounded-2xl mb-2">
              {nbsp.repeat(70)}
            </div>

            <span className="mb-4 rounded-full text-base font-medium text-transparent bg-black/5 dark:bg-white/5">
              {nbsp.repeat(25)}
            </span>
          </div>

          <div className="flex flex-col gap-1 w-full">
            <div className="text-transparent bg-black/5 dark:bg-white/5 rounded-full w-full">{nbsp}</div>
            <div className="text-transparent bg-black/5 dark:bg-white/5 rounded-full w-4/5">{nbsp}</div>
            <div className="text-transparent bg-black/5 dark:bg-white/5 rounded-full w-full">{nbsp}</div>
            <div className="text-transparent bg-black/5 dark:bg-white/5 rounded-full w-1/3">{nbsp}</div>
          </div>

          <div className="flex items-center gap-3">
            <div className="rounded-full size-10 bg-slate-200 dark:bg-slate-700 animate-pulse"></div>
            <div className="text-transparent rounded-full bg-black/10 dark:bg-white/10">
              {nbsp.repeat(40)}
            </div>
          </div>
        </div>
      </div>
    );
  }
}
