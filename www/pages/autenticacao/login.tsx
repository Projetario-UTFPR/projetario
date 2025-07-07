import { Head, Link, useForm, usePage } from "@inertiajs/react";
import clsx from "clsx";
import type { FormEvent } from "react";
import Form from "@/components/form";
import { H1 } from "@/components/h1";
import { Main } from "@/components/main";

const numbersOnlyRegex = /^[0-9]+$/;

function identificarRegistroAlunoOuEmailInstitucional(
  identificacao: string,
): { email: string } | { registro_aluno: string } {
  if (
    identificacao.charAt(0) === "a" &&
    numbersOnlyRegex.test(identificacao.substring(1))
  ) {
    return { registro_aluno: identificacao };
  }

  return {
    email: identificacao,
  };
}

type LoginForm = {
  registro_aluno?: string;
  email?: string;
  senha: string;
  // outros erros que podem ser enviados pelo controller
  error?: never;
};

export default function Login() {
  const props = usePage().props;
  const { errors, data, setData, post } = useForm<LoginForm>();

  const handleSubmit = (event: FormEvent) => {
    event.preventDefault();
    post("/autenticacao/login");
  };

  return (
    <>
      <Head>
        <title>Login</title>
      </Head>
      <Main className="max-w-sm my-20">
        <H1 className="mb-6">Login</H1>

        <section className="container-box">
          {props.autenticacao ? (
            <div className="flex flex-col gap-2">
              <h2>
                Você está autenticado como {props.autenticacao.usuario.nome}!
              </h2>

              <div className="flex gap-3">
                <Link
                  href="/"
                  className="underline decoration-wavy text-yellow-800 dark:text-yellow-500"
                >
                  Volte para o início
                </Link>

                <Link
                  href="/autenticacao/logout"
                  method="post"
                  className="underline decoration-wavy text-red-500 dark:text-red-300"
                >
                  Deslogar
                </Link>
              </div>
            </div>
          ) : (
            <>
              <form className="flex-5/6" onSubmit={handleSubmit}>
                {errors.error && (
                  <Form.AlertaDeErro>{errors.error}</Form.AlertaDeErro>
                )}
                <Form.Input
                  label="Identificação do Usuário"
                  type="text"
                  placeholder="Seu e-mail institucional ou RA"
                  error={errors.registro_aluno || errors.email}
                  onInput={(identificacao) => {
                    setData({
                      senha: data.senha,
                      ...identificarRegistroAlunoOuEmailInstitucional(
                        identificacao,
                      ),
                    });
                  }}
                />

                <Form.Input
                  label="Senha"
                  type="password"
                  placeholder="●●●●●●●●"
                  error={errors.senha}
                  onInput={(senha) => setData({ ...data, senha })}
                />

                <div className="mt-6 flex items-center gap-3">
                  <button
                    type="submit"
                    className={clsx(
                      "transition-all duration-100 bg-yellow-500 px-6 py-3 rounded-3xl font-medium",
                      "hover:brightness-95 active:brightness-90",
                    )}
                  >
                    Entrar
                  </button>

                  <Link
                    href="/projetos"
                    className={clsx(
                      "transition-all duration-100 bg-gray-300 px-6 py-3 rounded-3xl font-medium",
                      "hover:bg-gray-400 active:brightness-95",
                    )}
                  >
                    Ficar como visitante
                  </Link>
                </div>
              </form>

              <div className="bg-gray-200 p-3 rounded-lg self-start max-w-3xs">
                <h2>
                  <strong>Alunos</strong>
                </h2>
                <p className="mb-3">
                  Utilizar o número do RA precedido da letra "a".
                </p>

                <h2>
                  <strong>Professores</strong>
                </h2>
                <p>
                  Utilizar o nome de usuário e senha de seu e-mail
                  institucional.
                </p>
              </div>
            </>
          )}
        </section>
      </Main>
    </>
  );
}
