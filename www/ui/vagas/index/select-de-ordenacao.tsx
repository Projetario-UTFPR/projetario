import { ArrowsDownUpIcon } from "@phosphor-icons/react/dist/ssr/ArrowsDownUp";
import Select from "@/components/select";

const tituloDoSelect = "Alterar Ordenação";

const MAPA_VALOR_ORDENACAO = {
  "datetime-decreasing": ["data", "desc"] as const,
  "datetime-increasing": ["data", "asc"] as const,
  "title-decreasing": ["titulo", "desc"] as const,
  "title-increasing": ["titulo", "asc"] as const,
} as const;

type Value = "datetime-increasing" | "datetime-decreasing" | "title-increasing" | "title-decreasing";

type Props = {
  defaultValue?: Value;
  onSelect?: (ordenacao: readonly ["data" | "titulo", "asc" | "desc"] | [null, null]) => void;
};

function resolvaOrdenacaoSelecionada(value: Value) {
  return MAPA_VALOR_ORDENACAO[value] ?? [null, null];
}

export function SelectDeOrdenacao({ defaultValue, onSelect }: Props) {
  return (
    <Select.Root
      defaultValue={defaultValue}
      onValueChange={(valor: Value) => onSelect?.(resolvaOrdenacaoSelecionada(valor))}
    >
      <Select.Trigger
        className="shrink-0 self-stretch"
        icon={ArrowsDownUpIcon}
        placeholder={tituloDoSelect}
      />
      <Select.Content>
        <Select.ViewportGroup>
          <Select.Header title={tituloDoSelect} />
          <Select.Item value="datetime-decreasing">Mais antigas</Select.Item>
          <Select.Item value="datetime-increasing">Mais recentes</Select.Item>
          <Select.Item value="title-increasing">Título (A-Z)</Select.Item>
          <Select.Item value="title-decreasing">Título (Z-A)</Select.Item>
        </Select.ViewportGroup>
      </Select.Content>
    </Select.Root>
  );
}
