import type { PageProps } from "@inertiajs/core";
import { Deferred, router, usePage } from "@inertiajs/react";
import { FunnelIcon } from "@phosphor-icons/react/dist/ssr/Funnel";
import { FunnelSimpleIcon } from "@phosphor-icons/react/dist/ssr/FunnelSimple";
import { ListDashesIcon } from "@phosphor-icons/react/dist/ssr/ListDashes";
import { MagnifyingGlassIcon } from "@phosphor-icons/react/dist/ssr/MagnifyingGlass";
import { SquaresFourIcon } from "@phosphor-icons/react/dist/ssr/SquaresFour";
import { parseAsInteger, parseAsString, parseAsStringEnum, useQueryStates } from "nuqs";
import { useId } from "react";
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
import { FiltroDeTipoDeVaga } from "@/ui/vagas/index/filtro-de-tipo-de-vaga";

type Props = PageProps & {
  vagas: RespostaIncertaDoServidor<Paginacao<PreviewDeVaga>>;
};

export default function ListarVagas() {
  const selectId = useId();

  const [filters, _setFilters] = useQueryStates(
    {
      direcao_ord: parseAsStringEnum(["asc", "desc"]),
      ordenar_por: parseAsStringEnum(["titulo", "data"]),
      filtro: parseAsString,
      pagina: parseAsInteger,
      qtd_por_pagina: parseU8,
      filtrar_por: parseAsStringEnum(["titulo", "coordenador"]),
      tipo: parseAsStringEnum(["Extensao", "IniciacaoCientifica"] satisfies TipoDeProjeto[]),
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

        <div className="flex items-center px-4 py-2 rounded-3xl border border-black/20 dark:border-white/20 w-full">
          <label className="flex items-center gap-2.5 w-full">
            <MagnifyingGlassIcon size={24} weight="bold" />
            <input type="text" placeholder="Encontre qualquer projeto" className="w-full" />
          </label>

          <FiltroDeTipoDeVaga
            onValueChange={(tipo) => {
              setFilters({ tipo });
            }}
            paramKey="tipo"
          />
        </div>

        <label htmlFor={selectId} className="inline-flex items-center gap-4">
          <span>
            <FunnelSimpleIcon size={24} weight="bold" />
          </span>
          <select name="ordenacao" id={selectId}>
            <option value="">Alterar ordenação</option>

            <hr />

            <option value="data">Data de publicação</option>
            <option value="titulo">Ordem alfabética do título</option>
          </select>
          {/* <CaretDownIcon size={20} weight="bold" /> */}
        </label>

        <div>
          <button type="button">
            <ListDashesIcon size={24} weight="bold" />
          </button>
          <button type="button">
            <SquaresFourIcon size={24} weight="bold" />
          </button>
        </div>
      </search>

      <Deferred data="vagas" fallback={ListagemDeVagasSkeleton}>
        <div className="mb-6">
          <ListagemDeVagas />
        </div>
      </Deferred>
    </Main>
  );
}

function ListagemDeVagasSkeleton() {
  return (
    <div className="flex gap-6.25">
      <CardDePreviewDeVaga.Skeleton />
      <CardDePreviewDeVaga.Skeleton />
      <CardDePreviewDeVaga.Skeleton />
    </div>
  );
}

function ListagemDeVagas() {
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
    <div className="grid grid-flow-row grid-cols-3 gap-6.25 justify-between items-start">
      {vagas.map((vaga) => (
        <CardDePreviewDeVaga vaga={vaga} key={`card-de-vaga-${vaga.id}`} />
      ))}
    </div>
  );
}
