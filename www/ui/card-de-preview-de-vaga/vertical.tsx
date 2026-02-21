import { Link } from "@inertiajs/react";
import clsx from "clsx";
import { Avatar } from "@/components/avatar";
import type { PreviewDeVaga } from "@/core/types/entidades/preview-de-vaga";
import { Rotas } from "@/rotas";
import { PrimeiroParagrafoDoCorpoClientSide } from "./primeiro-paragrafo-client-side";

type Props = { vaga: PreviewDeVaga; className?: string };

export function Vertical({ vaga, className }: Props) {
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
