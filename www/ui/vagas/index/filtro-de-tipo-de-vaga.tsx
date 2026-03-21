import * as ToggleGroup from "@radix-ui/react-toggle-group";
import { useState } from "react";
import type { TipoDeProjeto } from "@/core/types/enums/tipo-de-projeto";
import { Item } from "./item";

type Props = {
  onValueChange: (value: TipoDeProjeto | null) => void;
  /**
   * The search params key that represents the values of this filter.
   */
  paramKey: string;
};

const TIPOS_DE_PROJETO = ["Extensao", "IniciacaoCientifica"] as const satisfies TipoDeProjeto[];

function resolvaFiltragemExclusivaPorTipoDeVaga(_tiposPermitidos: TipoDeProjeto[]): TipoDeProjeto | null {
  const tiposPermitidos = new Set(_tiposPermitidos);

  if (tiposPermitidos.has("Extensao") && tiposPermitidos.has("IniciacaoCientifica")) return null;
  if (tiposPermitidos.size === 0) return null;

  const tipoRestante = tiposPermitidos.values().toArray()[0];
  return tipoRestante;
}

function obtenhaListaInicialDeTiposSelecionados(paramKey: string): TipoDeProjeto[] {
  const searchParams = new URLSearchParams(window.location.search);
  const tipoValidoSelecionado = TIPOS_DE_PROJETO.find((tipo) => tipo === searchParams.get(paramKey));

  if (tipoValidoSelecionado) return [tipoValidoSelecionado];
  return [...TIPOS_DE_PROJETO];
}

export function FiltroDeTipoDeVaga({ onValueChange, paramKey }: Props) {
  const [tiposSelecionados, setTiposSelecionados] = useState<TipoDeProjeto[]>(
    obtenhaListaInicialDeTiposSelecionados(paramKey),
  );

  const atualizarTiposSelecionados = (novosTipos: TipoDeProjeto[]) => {
    const tipoExclusivo = resolvaFiltragemExclusivaPorTipoDeVaga(novosTipos);
    setTiposSelecionados(novosTipos);
    onValueChange(tipoExclusivo);
  };

  return (
    <ToggleGroup.Root
      type="multiple"
      defaultValue={tiposSelecionados}
      onValueChange={atualizarTiposSelecionados}
      className="shrink-0 flex items-center justify-center gap-1.25"
    >
      <Item value="Extensao">Extensão</Item>
      <Item value="IniciacaoCientifica">Iniciação científica</Item>
    </ToggleGroup.Root>
  );
}
