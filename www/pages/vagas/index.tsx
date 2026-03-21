import type { PageProps } from "@inertiajs/core";
import { Deferred, router, usePage } from "@inertiajs/react";
import { FunnelIcon } from "@phosphor-icons/react/dist/ssr/Funnel";
import clsx from "clsx";
import { parseAsInteger, parseAsIsoDateTime, parseAsString, parseAsStringEnum, useQueryStates } from "nuqs";
import { useState } from "react";
import { useDebouncedCallback } from "use-debounce";
import { Alerta } from "@/components/alerta";
import Button from "@/components/button";
import { AlertaDeErro } from "@/components/form/alerta-de-erro";
import { H1 } from "@/components/h1";
import { Main } from "@/components/main";
import type { PreviewDeVaga } from "@/core/types/entidades/preview-de-vaga";
import type { TipoDeProjeto } from "@/core/types/enums/tipo-de-projeto";
import type { Paginacao } from "@/core/types/paginacao";
import type { RespostaIncertaDoServidor } from "@/core/types/resposta-incerta";
import { parseU8 } from "@/lib/nuqs";
import { CardDePreviewDeVaga } from "@/ui/card-de-preview-de-vaga";
import { DirecaoDosCardsDeVagas } from "@/ui/vagas/index/direcao-dos-cards-de-vagas";
import { FiltroDeTipoDeVaga } from "@/ui/vagas/index/filtro-de-tipo-de-vaga";
import FiltroDeTitulo from "@/ui/vagas/index/filtro-de-titulo";
import { SelectDeOrdenacao } from "@/ui/vagas/index/select-de-ordenacao";

type Props = PageProps & {
  vagas: RespostaIncertaDoServidor<Paginacao<PreviewDeVaga>>;
};

const DIRECAO_FAVORITA_KEY = "projetario_utfpr_vacancies_direction";

const obtenhaPrefereCardsHorizontais = () => window.localStorage.getItem(DIRECAO_FAVORITA_KEY) === "true";
const salvePrefereCardsHorizontais = (value: boolean) =>
  window.localStorage.setItem(DIRECAO_FAVORITA_KEY, value.toString());

export default function ListarVagas() {
  const [cardsHorizontais, _setCardsHorizontais] = useState(obtenhaPrefereCardsHorizontais());

  const setCardsHorizontais = (value: boolean) => {
    _setCardsHorizontais(value);
    salvePrefereCardsHorizontais(value);
  };

  const [filters, _setFilters] = useQueryStates(
    {
      titulo: parseAsString,
      coordenador: parseAsString,
      tipo: parseAsStringEnum(["Extensao", "IniciacaoCientifica"] satisfies TipoDeProjeto[]),
      dp_data: parseAsIsoDateTime,
      dp_lim: parseAsStringEnum(["ate", "apos"]),
      direcao_ord: parseAsStringEnum(["asc", "desc"]),
      ordenar_por: parseAsStringEnum(["titulo", "data"]),
      pagina: parseAsInteger,
      qtd_por_pagina: parseU8,
    },
    { shallow: true },
  );

  const aplicarFiltros = useDebouncedCallback(() => {
    const nonNull = ([_, value]: [string, unknown]) => value !== null;
    const entries = Object.entries(filters).filter(nonNull) as string[][];
    const searchParams = new URLSearchParams(entries).toString();
    router.get(`?${searchParams}`, undefined, { preserveState: true, replace: true });
  }, 300);

  const setFilters = (_filters: Partial<typeof filters>) => {
    _setFilters(_filters);
    aplicarFiltros();
  };

  return (
    <Main className="mt-20">
      <H1 className="mb-6">Projetos e pesquisas</H1>

      <search className="mb-6 container-box p-6 flex flexrow gap-6 items-center">
        <Button.Secundario>
          <FunnelIcon size={24} weight="bold" />
          Filtros
        </Button.Secundario>

        <FiltroDeTitulo.Root>
          <FiltroDeTitulo.Input onInput={(titulo) => setFilters({ titulo })} />
          <FiltroDeTipoDeVaga
            onValueChange={(tipo) => {
              setFilters({ tipo });
            }}
            paramKey="tipo"
          />
        </FiltroDeTitulo.Root>

        <SelectDeOrdenacao
          defaultValue="datetime-decreasing"
          onSelect={([ordenar_por, direcao_ord]) => setFilters({ ordenar_por, direcao_ord })}
        />

        <DirecaoDosCardsDeVagas
          defaultValue={obtenhaPrefereCardsHorizontais() ? "horizontal" : "vertical"}
          onValueChange={(direcao) => setCardsHorizontais(direcao === "horizontal")}
        />
      </search>

      <Deferred data="vagas" fallback={<ListagemDeVagasSkeleton cardsHorizontais={cardsHorizontais} />}>
        <div className="mb-6">
          <ListagemDeVagas cardsHorizontais={cardsHorizontais} />
        </div>
      </Deferred>
    </Main>
  );
}

function ListagemDeVagasSkeleton({ cardsHorizontais }: { cardsHorizontais: boolean }) {
  return (
    <div className={clsx("flex gap-6.25", cardsHorizontais && "flex-col")}>
      <CardDePreviewDeVaga.Skeleton direcao={cardsHorizontais ? "horizontal" : "vertical"} />
      <CardDePreviewDeVaga.Skeleton direcao={cardsHorizontais ? "horizontal" : "vertical"} />
      <CardDePreviewDeVaga.Skeleton direcao={cardsHorizontais ? "horizontal" : "vertical"} />
    </div>
  );
}

function ListagemDeVagas({ cardsHorizontais }: { cardsHorizontais: boolean }) {
  const page = usePage<Props>();
  const vagasResponse = page.props.vagas;

  if (!vagasResponse.success) {
    return <AlertaDeErro>{vagasResponse.error}</AlertaDeErro>;
  }

  const { dados: vagas, ..._paginacao } = vagasResponse.data;

  if (vagas.length <= 0) {
    return <Alerta>Não existem vagas abertas no momento.</Alerta>;
  }

  return (
    <div
      className={clsx(
        "gap-6.25",
        cardsHorizontais ? "flex flex-col" : "grid grid-flow-row grid-cols-3 justify-between items-start",
      )}
    >
      {vagas.map((vaga) => (
        <CardDePreviewDeVaga
          vaga={vaga}
          key={`card-de-vaga-${vaga.id}`}
          direcao={cardsHorizontais ? "horizontal" : "vertical"}
        />
      ))}
    </div>
  );
}
