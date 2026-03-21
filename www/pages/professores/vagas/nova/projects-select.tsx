import { router, usePage } from "@inertiajs/react";
import type { PropsWithChildren } from "react";
import { Alerta } from "@/components/alerta";
import { AlertaDeErro } from "@/components/form/alerta-de-erro";
import { InputLabelSpan } from "@/components/form/label-span";
import Select from "@/components/select";
import type { Projeto } from "@/core/types/entidades/projeto";
import type { Paginacao } from "@/core/types/paginacao";
import type { RespostaIncertaDoServidor } from "@/core/types/resposta-incerta";

type ProjectsSelectProps = {
  projectId: string | undefined;
  setProjectId: (projectId: string) => void;
  error?: string;
};

function ProjectsSelectWrapper({ children, error }: PropsWithChildren<{ error?: string }>) {
  return (
    <label htmlFor="select-projetos" className="flex flex-col gap-2 mb-3">
      <InputLabelSpan required>Projetos</InputLabelSpan>

      {error && <AlertaDeErro>{error}</AlertaDeErro>}

      {children}
    </label>
  );
}

export function ProjectsSelect({ projectId, setProjectId, error }: ProjectsSelectProps) {
  type Projetos = RespostaIncertaDoServidor<Paginacao<Projeto>>;
  const projects = usePage().props.projetos as Projetos;

  const selectElement = projects.success ? (
    <ProjectsSelectSuccess projects={projects.data} selected={projectId ?? ""} onSelect={setProjectId} />
  ) : (
    <ProjectsSelectFailure error={projects.error} />
  );

  return <ProjectsSelectWrapper error={error}>{selectElement}</ProjectsSelectWrapper>;
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
      <button onClick={tryToRefetch} type="button" className="absolute right-2 bottom-1 text-xs underline">
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

function ProjectsSelectSuccess({ projects, selected, onSelect }: ProjectsSelectSuccessProps) {
  return (
    <Select.Root required name="select-de-projetos" value={selected} onValueChange={onSelect}>
      <Select.Trigger placeholder="Escolha um projeto" />

      <Select.Content>
        {projects.total > 0 ? (
          <Select.ViewportGroup>{projects.dados.map(ProjectItem)}</Select.ViewportGroup>
        ) : (
          <Alerta>Você ainda não possui nenhum projeto de extensão.</Alerta>
        )}
      </Select.Content>
    </Select.Root>
  );
}

function ProjectItem(project: Projeto) {
  return (
    <Select.Item key={`projects-select-project-item-id-${project.id}`} value={project.id} autoFocus={false}>
      {project.titulo}
    </Select.Item>
  );
}

export function ProjectsSelectSkeleton() {
  return (
    <ProjectsSelectWrapper>
      <Select.Skeleton />
    </ProjectsSelectWrapper>
  );
}
