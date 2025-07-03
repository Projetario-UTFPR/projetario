import { Link } from "@inertiajs/react";
import clsx from "clsx";
import Form from "@/components/form";
import { Main } from "@/components/main";

export default function Login() {
  return (
    <Main className="max-w-sm">
      <h1>Login</h1>

      <section className="container-box">
        <form className="flex-5/6">
          <Form.Input
            label="Identificação do Usuário"
            type="text"
            placeholder="Seu e-mail institucional ou RA"
          />

          <Form.Input label="Senha" type="password" placeholder="●●●●●●●●" />

          <div className="mt-6 flex items-center gap-3">
            <button
              type="submit"
              className="bg-yellow-500 px-6 py-3 rounded-3xl font-medium"
            >
              Entrar
            </button>

            <Link
              href="/projetos"
              className="bg-gray-300 px-6 py-3 rounded-3xl font-medium"
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
          <p>Utilizar o nome de usuário e senha de seu e-mail institucional.</p>
        </div>
      </section>
    </Main>
  );
}
