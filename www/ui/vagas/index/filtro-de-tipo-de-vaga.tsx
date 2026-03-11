import * as Toggle from "@radix-ui/react-toggle";
import clsx from "clsx";
import { useEffect, useState } from "react";
import type { TipoDeProjeto } from "@/core/types/enums/tipo-de-projeto";

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

type ItemProps = Omit<Toggle.ToggleProps, "value"> & {
  value: TipoDeProjeto;
  toggle: (value: TipoDeProjeto, pressed: boolean) => void;
};

function Item({ value, children, toggle, ...props }: ItemProps) {
  return (
    <Toggle.Root
      value={value}
      {...props}
      className={clsx(
        "text-base font-regular underline capitalize leading-none transition-all duration-100",
        "px-2.5 py-[5px] rounded-full",

        "data-[state=off]:not-dark:bg-gray-200 data-[state=on]:not-dark:bg-yellow-500",
        "data-[state=off]:not-dark:hover:bg-gray-300 data-[state=off]:not-dark:active:bg-gray-400",
        "data-[state=on]:not-dark:hover:bg-yellow-600 data-[state=on]:not-dark:active:brightness-95",

        "data-[state=on]:dark:text-yellow-500 data-[state=off]:dark:text-gray-300",
        "data-[state=on]:dark:bg-yellow-500/10 data-[state=off]:dark:bg-gray-500/20",
        "data-[state=off]:dark:hover:bg-gray-500/30 data-[state=off]:dark:active:bg-gray-500/40",
        "data-[state=on]:dark:hover:bg-yellow-500/15 data-[state=on]:dark:active:bg-yellow-500/25",
      )}
      onPressedChange={(pressed) => toggle(value, pressed)}
    >
      {children}
    </Toggle.Root>
  );
}
