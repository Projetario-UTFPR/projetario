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
  const tipoJaSelecionado = searchParams.get(paramKey);

  if (!tipoJaSelecionado) return [...TIPOS_DE_PROJETO];

  const tipoJaSelecionadoEhValido = TIPOS_DE_PROJETO.includes(tipoJaSelecionado as TipoDeProjeto);

  if (!tipoJaSelecionadoEhValido) return [...TIPOS_DE_PROJETO];

  return [tipoJaSelecionado as TipoDeProjeto];
}

export function FiltroDeTipoDeVaga({ onValueChange, paramKey }: Props) {
  const [tiposSelecionados, setTiposSelecionados] = useState<TipoDeProjeto[]>(
    obtenhaListaInicialDeTiposSelecionados(paramKey),
  );

  const calculeNovosTiposSelecionados = (value: TipoDeProjeto, pressed: boolean) => {
    if (pressed) return [...tiposSelecionados, value];
    return tiposSelecionados.filter((tipo) => tipo !== value);
  };

  const atualizarTiposSelecionados = (value: TipoDeProjeto, pressed: boolean) => {
    const novosTipos = calculeNovosTiposSelecionados(value, pressed);
    const tipoExclusivo = resolvaFiltragemExclusivaPorTipoDeVaga(novosTipos);
    setTiposSelecionados(novosTipos);
    onValueChange(tipoExclusivo);
  };

  return (
    <div className="shrink-0 flex items-center justify-center gap-1.25">
      <Item
        value="Extensao"
        pressed={tiposSelecionados.includes("Extensao")}
        toggle={atualizarTiposSelecionados}
      >
        Extensão
      </Item>
      <Item
        value="IniciacaoCientifica"
        pressed={tiposSelecionados.includes("IniciacaoCientifica")}
        toggle={atualizarTiposSelecionados}
      >
        Iniciação científica
      </Item>
    </div>
  );
}
