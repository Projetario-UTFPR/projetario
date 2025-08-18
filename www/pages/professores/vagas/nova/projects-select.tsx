import { router, usePage } from "@inertiajs/react";
import { CaretDownIcon } from "@phosphor-icons/react/dist/ssr/CaretDown";
import * as S from "@radix-ui/react-select";
import clsx from "clsx";
import type { PropsWithChildren } from "react";
import { AlertaDeErro } from "@/components/form/alerta-de-erro";
import { InputLabelSpan } from "@/components/form/label-span";
import type { Projeto } from "@/core/types/entidades/projeto";
import type { Paginacao } from "@/core/types/paginacao";
import type { RespostaIncertaDoServidor } from "@/core/types/resposta-incerta";

type ProjectsSelectProps = {
  projectId: string | undefined;
  setProjectId: (projectId: string) => void;
  error?: string;
};

function ProjectsSelectWrapper({
  children,
  error,
}: PropsWithChildren<{ error?: string }>) {
  return (
    <label htmlFor="select-projetos" className="flex flex-col gap-2 mb-3">
      <InputLabelSpan required>Projetos</InputLabelSpan>

      {error && <AlertaDeErro>{error}</AlertaDeErro>}

      {children}
    </label>
  );
}

export function ProjectsSelect({
  projectId,
  setProjectId,
  error,
}: ProjectsSelectProps) {
  type Projetos = RespostaIncertaDoServidor<Paginacao<Projeto>>;
  const projects = usePage().props.projetos as Projetos;

  const selectElement = projects.success ? (
    <ProjectsSelectSuccess
      projects={projects.data}
      selected={projectId ?? ""}
      onSelect={setProjectId}
    />
  ) : (
    <ProjectsSelectFailure error={projects.error} />
  );

  return (
    <ProjectsSelectWrapper error={error}>{selectElement}</ProjectsSelectWrapper>
  );
}

function ProjectsSelectFailure({ error }: { error: string }) {
  const pageUrl = usePage().url;
  const tryToRefetch = () => {
    router.visit(pageUrl, {
      only: ["projetos"],
      async: true,
      preserveState: true,
      preserveScroll: true,
      preserveUrl: true,
    });
  };

  return (
    <AlertaDeErro className="relative p-4!">
      {error}
      <button
        onClick={tryToRefetch}
        type="button"
        className="absolute right-2 bottom-1 text-xs underline"
      >
        Tentar novamente
      </button>
    </AlertaDeErro>
  );
}

type ProjectsSelectSuccessProps = {
  projects: Paginacao<Projeto>;
  selected: string | undefined;
  onSelect: (value: string) => void;
};

function ProjectsSelectSuccess({
  projects,
  selected,
  onSelect,
}: ProjectsSelectSuccessProps) {
  return (
    <S.Root
      required
      name="select-de-projetos"
      value={selected}
      onValueChange={onSelect}
    >
      <S.Trigger className="group text-input leading-0 flex items-center justify-between">
        <S.Value
          placeholder="Escolha um projeto"
          className="placeholder:text-gray-300"
        />
        <S.Icon>
          <CaretDownIcon
            size={16}
            weight="bold"
            className="transition-all text-gray-500 group-data-[state=open]:rotate-180"
          />
        </S.Icon>
      </S.Trigger>

      <S.Portal>
        <S.Content
          position="popper"
          side="bottom"
          align="center"
          sideOffset={4}
          alignOffset={12}
          sticky="partial"
          collisionPadding={24}
          className={clsx(
            "overflow-hidden bg-white shadow-md rounded-2xl border border-black/10",
            "w-[var(--radix-select-trigger-width)] p-2 flex flex-col gap-0.5",
            "data-[side=bottom]:animate-slide-up-and-fade data-[side=left]:animate-slide-right-and-fade",
            "data-[side=right]:animate-slide-left-and-fade data-[side=top]:animate-slide-down-and-fade",
          )}
        >
          {projects.dados.map(ProjectItem)}
        </S.Content>
      </S.Portal>
    </S.Root>
  );
}

function ProjectItem(project: Projeto) {
  return (
    <S.Item
      key={`projects-select-project-item-id-${project.id}`}
      value={project.id}
      autoFocus={false}
      className={clsx(
        "py-1 px-2 rounded-xl relative overflow-hidden",
        "outline-none",
        "before:absolute before:inset-2 before:rounded-xl before:transition-all",
        "before:duration-100 before:bg-transparent",
        "hover:before:inset-0 hover:before:bg-yellow-500/50",
        "active:before:bg-yellow-500 focus:before:bg-yellow-500/50 focus:before:inset-0",
        "data-[state=checked]:before:bg-yellow-500 data-[state=checked]:before:inset-0",
      )}
    >
      <S.ItemText>
        <div className="block relative z-10 cursor-default">
          {project.titulo}
        </div>
      </S.ItemText>
    </S.Item>
  );
}

export function ProjectsSelectSkeleton() {
  return (
    <ProjectsSelectWrapper>
      <div className="text-input animate-pulse bg-gray-200 h-9" />
    </ProjectsSelectWrapper>
  );
}
