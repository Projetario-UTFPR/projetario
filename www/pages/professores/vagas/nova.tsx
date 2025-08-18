import { Head, useForm } from "@inertiajs/react";
import type { FormEvent } from "react";
import { toast } from "react-toastify";
import Button from "@/components/button";
import Form from "@/components/form";
import { H1 } from "@/components/h1";
import { Main } from "@/components/main";

type FormData = {
  id_projeto: string;
  horas_por_semana: number;
  imagem: string;
  quantidade: number;
  link_edital: string;
  conteudo: string;
  titulo: string;
  link_candidatura?: null | string;
  inscricoes_ate: string | Date;
  erro?: never;
};

const dateTimeIntoDateString = (date?: string | Date): string | undefined => {
  if (!date) return undefined;
  return new Date(date).toISOString().split("T")[0];
};

export default function CriarNovaVagaDeProjeto() {
  const { data, setData, errors, post, processing, reset } = useForm<FormData>({
    id_projeto: undefined as unknown as string,
    horas_por_semana: 0,
    imagem: "",
    quantidade: 0,
    link_edital: "",
    conteudo: "Descreva a vaga com o máximo de detalhes possível!",
    titulo: "",
    link_candidatura: null,
    inscricoes_ate: new Date(),
  });

  const handleSubmit = (event: FormEvent) => {
    event.preventDefault();

    post("/professores/vagas/criar", {
      onSuccess() {
        reset();
        toast("Vaga criada com sucesso!", { type: "success" });
      },
    });
  };

  return (
    <>
      <Head>
        <title>Criar nova vaga de projeto</title>
      </Head>

      <Main className="my-20">
        <H1 className="mb-6">Nova Vaga</H1>

        <section className="container-box">
          <form className="w-full" onSubmit={handleSubmit}>
            {errors.erro && (
              <Form.AlertaDeErro className="mb-3">
                {errors.erro}
              </Form.AlertaDeErro>
            )}

            <Form.Input
              value={data.titulo}
              label="Titulo"
              type="text"
              error={errors.titulo}
              name="titulo"
              placeholder="Título"
              required
              onInput={(titulo) => setData({ ...data, titulo })}
            />

            <Form.Input
              value={data.link_edital}
              label="Link do edital"
              type="text"
              error={errors.link_edital}
              name="edital"
              placeholder="https://..."
              required
              onInput={(link_edital) => setData({ ...data, link_edital })}
            />

            <Form.Input
              value={data.link_candidatura ?? undefined}
              label="Link do formulário de candidatura"
              type="text"
              error={errors.link_candidatura}
              name="candidatura"
              placeholder="https://..."
              onInput={(link_candidatura) =>
                setData({
                  ...data,
                  link_candidatura:
                    link_candidatura.length <= 0 ? null : link_candidatura,
                })
              }
            />

            <Form.Input
              label="Id do projeto"
              type="text"
              error={errors.id_projeto}
              name="projeto"
              placeholder="9cb6564e-9c9c-405b-a912-d156fb41a509"
              required
              onInput={(id_projeto) => setData({ ...data, id_projeto })}
            />

            <Form.Input
              value={data.horas_por_semana}
              label="Quantidade de horas por semana"
              type="number"
              error={errors.horas_por_semana}
              name="horas"
              placeholder="3"
              required
              onInput={(horas_por_semana) =>
                setData({ ...data, horas_por_semana: Number(horas_por_semana) })
              }
            />

            <Form.Input
              value={data.quantidade}
              label="Quatidade de vagas"
              type="number"
              error={errors.quantidade}
              name="qtd"
              placeholder="2"
              required
              observacao="A quantidade de vagas se refere somente àquelas destinadas a graduandos."
              onInput={(quantidade) =>
                setData({ ...data, quantidade: Number(quantidade) })
              }
            />

            <Form.Input
              value={data.imagem}
              label="Imagem de capa"
              type="text"
              error={errors.imagem}
              name="imagem"
              placeholder="https://..."
              required
              observacao="Coloque o URL da imagem que você quer utilizar como capa desta vaga."
              onInput={(imagem) => setData({ ...data, imagem })}
            />

            <Form.Input
              value={dateTimeIntoDateString(data.inscricoes_ate)}
              label="Data limite das inscrições"
              type="date"
              error={errors.inscricoes_ate}
              name="projeto"
              required
              onInput={(date) => {
                setData({
                  ...data,
                  inscricoes_ate: new Date(date),
                });
              }}
            />

            <Form.Editor
              label="Corpo da vaga"
              atualizarCoteudo={(conteudo) => setData({ ...data, conteudo })}
              error={errors.conteudo}
              value={data.conteudo}
              required
            />

            <Button.CallToAction type="submit" className="mt-6">
              Publicar
            </Button.CallToAction>
          </form>
        </section>
      </Main>
    </>
  );
}
666;
