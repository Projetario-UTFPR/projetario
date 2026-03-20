import * as Toggle from "@radix-ui/react-toggle";
import clsx from "clsx";
import { useEffect, useState } from "react";
import type { TipoDeProjeto } from "@/core/types/enums/tipo-de-projeto";
import { Item } from "./item";

type Props = {
  onValueChange: (value: TipoDeProjeto | null) => void;
};

function resolvaFiltragemExclusivaPorTipoDeVaga(_tiposPermitidos: TipoDeProjeto[]): TipoDeProjeto | null {
  const tiposPermitidos = new Set(_tiposPermitidos);

  if (tiposPermitidos.has("Extensao") && tiposPermitidos.has("IniciacaoCientifica")) return null;
  if (tiposPermitidos.size === 0) return null;

  const tipoRestante = tiposPermitidos.values().toArray()[0];
  return tipoRestante;
}

export function FiltroDeTipoDeVaga({ onValueChange }: Props) {
  const [tiposSelecionados, setTiposSelecionados] = useState<TipoDeProjeto[]>([]);

  useEffect(() => {
    const tipoExclusivo = resolvaFiltragemExclusivaPorTipoDeVaga(tiposSelecionados);
    onValueChange(tipoExclusivo);
  }, [tiposSelecionados, onValueChange]);

  const toggle = (value: TipoDeProjeto, pressed: boolean) => {
    if (pressed) return setTiposSelecionados((tipos) => [...tipos, value]);
    setTiposSelecionados((tipos) => tipos.filter((tipo) => tipo !== value));
  };

  return (
    <div className="shrink-0 flex items-center justify-center gap-[5px]">
      <Item value="Extensao" pressed={tiposSelecionados.includes("Extensao")} toggle={toggle}>
        Extensão
      </Item>
      <Item
        value="IniciacaoCientifica"
        pressed={tiposSelecionados.includes("IniciacaoCientifica")}
        toggle={toggle}
      >
        Iniciação científica
      </Item>
    </div>
  );
}
