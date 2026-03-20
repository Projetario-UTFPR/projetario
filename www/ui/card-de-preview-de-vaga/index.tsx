import type { PreviewDeVaga } from "@/core/types/entidades/preview-de-vaga";
import { CardDePreviewDeVagaVertical } from "./vertical";

type CardDeVagaProps = {
  vaga: PreviewDeVaga;
  variacao?: "vertical" | "horizontal";
};

export function CardDePreviewDeVaga({ vaga, variacao = "vertical" }: CardDeVagaProps) {
  if (variacao === "vertical") {
    return <CardDePreviewDeVagaVertical vaga={vaga} />;
  }

  return <div></div>;
}

export namespace CardDePreviewDeVaga {
  type Props = { variacao?: CardDeVagaProps["variacao"] };

  export function Skeleton({ variacao = "vertical" }: Props) {
    if (variacao === "vertical") return <CardDePreviewDeVagaVertical.Skeleton />;
  }
}
