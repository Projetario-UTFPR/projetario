import { Head, Link, useForm, usePage } from "@inertiajs/react";
import { PlusIcon } from "@phosphor-icons/react/dist/ssr/Plus";
import type { SubmitEventHandler } from "react";
import { toast } from "react-toastify";
import Button from "@/components/button";
import Form from "@/components/form";
import { H1 } from "@/components/h1";
import { Main } from "@/components/main";
import { Rotas } from "@/rotas";
import { resolvaTema } from "@/tema";

type FormData = {
  titulo: string;
  descricao: string;
  data_de_inicio?: string;
  erro?: never;
};

export default function NovoProjetoDeExtensao() {
  const props = usePage().props;
  const tema = resolvaTema(props.temaPreferido, props.temaSistema);
  const { submit, data, setData, errors, processing } = useForm<FormData>(
    "post",
    "/professores/projetos/extensao/criar_e_associar",
    {
      descricao: "<p>Descreva o projeto em detalhes.",
      titulo: "",
    },
  );

  const handleSubmit: SubmitEventHandler = (event) => {
    event.preventDefault();

    submit({
      onSuccess: (page) => {
        const mensagem = page.props.flash?.mensagemSucesso ?? "Projeto criado com sucesso!";

        toast(mensagem, { type: "success" });
      },
    });
  };

  return (
    <>
      <Head>
        <title>Criar novo projeto de extensão</title>
      </Head>

      <Main className="my-20">
        <H1 className="mb-6">Novo Projeto de Extensão</H1>

        <section className="container-box">
          <form className="w-full" onSubmit={handleSubmit}>
            {errors.erro && <Form.AlertaDeErro className="mb-3">{errors.erro}</Form.AlertaDeErro>}

            <Form.Input
              label="Título do Projeto"
              placeholder="Lorem Ipsum"
              type="text"
              name="titulo"
              onInput={(titulo) => setData({ ...data, titulo })}
              error={errors.titulo}
              required
            />

            <Form.Input
              label="Data de Início"
              placeholder="DD/MM/AAAA"
              type="date"
              onInput={(value) =>
                setData({
                  ...data,
                  data_de_inicio: value ? new Date(value).toJSON().slice(0, 10) : undefined,
                })
              }
              error={errors.data_de_inicio}
              observacao="Apenas preencha esse campo se você quiser adicionar uma data customizada."
            />

            <Form.Editor
              required
              tema={tema}
              label="Conteúdo"
              erro={errors.descricao}
              value={data.descricao}
              atualizarConteudo={(descricao) => setData({ ...data, descricao })}
            />

            <div className="flex items-center gap-3 mt-6">
              <Button.CallToAction type="submit" disabled={processing}>
                <PlusIcon size={16} weight="bold" />
                Criar
              </Button.CallToAction>
              <Button.Secundario asChild>
                <Link href={Rotas.inicio()}>Voltar</Link>
              </Button.Secundario>
            </div>
          </form>
        </section>
      </Main>
    </>
  );
}
