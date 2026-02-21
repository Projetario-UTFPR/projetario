import type { PreviewDeVaga } from "@/core/types/entidades/preview-de-vaga";
import { Vertical } from "./vertical";

type CardDeVagaProps = {
  vaga: PreviewDeVaga;
  variacao?: "default" | "outline";
};

export function CardDePreviewDeVaga({ vaga, variacao = "default" }: CardDeVagaProps) {
  if (variacao === "default") {
    return <Vertical vaga={vaga} />;
  }

  return <div></div>;
}
