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

export default function CriarNovaVagaDeProjeto() {
  const { data, setData, errors, post } = useForm<FormData>();

  const handleSubmit = (event: FormEvent) => {
    event.preventDefault();

    post("/professores/vagas/criar", {
      onSuccess() {
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
              label="Titulo"
              type="text"
              error={errors.titulo}
              name="titulo"
              placeholder="Título"
              required
              onInput={(titulo) => setData({ ...data, titulo })}
            />

            <Form.Input
              label="Link do edital"
              type="text"
              error={errors.link_edital}
              name="edital"
              placeholder="https://..."
              required
              onInput={(link_edital) => setData({ ...data, link_edital })}
            />

            <Form.Input
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
              label="Data limite das inscrições"
              type="date"
              error={errors.inscricoes_ate}
              name="projeto"
              required
              onInput={(date) => {
                const inscricoes_ate = new Date(date)
                  .toISOString()
                  .slice(0, 19);

                setData({
                  ...data,
                  inscricoes_ate,
                });
              }}
            />

            <Form.Editor
              label="Corpo da vaga"
              atualizarCoteudo={(conteudo) => setData({ ...data, conteudo })}
              error={errors.conteudo}
              initialValue="Descreva a vaga com o máximo de detalhes possível!"
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
