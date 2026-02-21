import clsx from "clsx";
import { memo, useMemo } from "react";

type PrimeiroParagrafoDoCorpoProps = {
  html: string;
  className?: string;
};

function removaImagensEObtenhaPrimeiroParagrafoDeTexto(paragrafos: NodeListOf<HTMLParagraphElement>) {
  return Array.from(paragrafos).find((paragrafo) => {
    paragrafo.querySelectorAll(":is(img, svg)").forEach((node) => node.remove());
    const paragrafoFicouVazio = !paragrafo.textContent.trim();
    return !paragrafoFicouVazio;
  });
}

export const PrimeiroParagrafoDoCorpoClientSide = memo(({ html, className }: PrimeiroParagrafoDoCorpoProps) => {
  const primeiroParagrafo = useMemo(() => {
    const container = document.createElement("div");
    container.innerHTML = html;
    const paragrafos = container.querySelectorAll("p");
    return removaImagensEObtenhaPrimeiroParagrafoDeTexto(paragrafos);
  }, [html]);

  if (!primeiroParagrafo) return null;

  return (
    <div
      className={clsx("text-black/70 dark:text-white/70", className)}
      // biome-ignore lint/security/noDangerouslySetInnerHtml: O conteúdo HTML é definido somente por professores, logo, deve ser seguro.
      dangerouslySetInnerHTML={{ __html: primeiroParagrafo.outerHTML }}
    />
  );
});
