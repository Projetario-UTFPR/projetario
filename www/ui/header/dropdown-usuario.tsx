import { Link, usePage } from "@inertiajs/react";
import * as Dropdown from "@radix-ui/react-dropdown-menu";
import clsx from "clsx";
import type { PropsWithChildren } from "react";
import { Avatar } from "@/components/avatar";
import { Hr } from "@/components/hr";
import { cargoEhMaiorOuIgual } from "@/core/utils/hierarquia-de-cargo";

export function DropdownUsuario() {
  const autenticacao = usePage().props.autenticacao;

  if (!autenticacao) return null;

  const usuario = autenticacao.usuario;

  return (
    <Dropdown.Root>
      <Dropdown.Trigger
        className={clsx(
          "flex gap-3 items-center border border-black/10 transition-all duration-100",
          "rounded-3xl bg-gray-200 pl-4 pr-[5px] py-[5px] font-bold",
          "dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-600",
        )}
      >
        {usuario.nome}
        <Avatar nomeDoUsuario={usuario.nome} />
      </Dropdown.Trigger>

      <Dropdown.Portal>
        <Dropdown.Content
          sideOffset={8}
          alignOffset={24}
          collisionPadding={24}
          className={clsx(
            "p-2 rounded-2xl flex flex-col gap-1.5 bg-white dark:bg-gray-800 border border-black/10",
            "max-w-[calc(100vw_-_48px)] min-w-48 drop-shadow-black/5 drop-shadow-2xl",
          )}
        >
          {cargoEhMaiorOuIgual(usuario.cargo, "Professor") && (
            <>
              <SecaoParaProfessoresSomente />
              <Hr />
            </>
          )}

          <DropdownItem asChild danger>
            <Link href="/autenticacao/logout" method="post">
              Deslogar
            </Link>
          </DropdownItem>
        </Dropdown.Content>
      </Dropdown.Portal>
    </Dropdown.Root>
  );
}

function SecaoParaProfessoresSomente() {
  return (
    <DropdownGroup label="Professores">
      <DropdownItem asChild>
        <Link href="/professores/projetos/extensao/novo">
          Novo projeto de extensão
        </Link>
      </DropdownItem>
      <DropdownItem asChild>
        <Link href="/professores/vagas/nova">Nova vaga de projeto</Link>
      </DropdownItem>
    </DropdownGroup>
  );
}

type DropdownGroupProps = PropsWithChildren<{
  label: string;
}>;

function DropdownGroup({ children, label }: DropdownGroupProps) {
  return (
    <Dropdown.Group>
      <DropdownLabel>{label}</DropdownLabel>
      <div className="flex flex-col gap-1.5">{children}</div>
    </Dropdown.Group>
  );
}

type DropdownItemProps = PropsWithChildren<{
  asChild?: boolean;
  className?: string;
  danger?: boolean;
}>;

function DropdownItem({
  children,
  asChild,
  className,
  danger,
}: DropdownItemProps) {
  return (
    <Dropdown.Item
      asChild={asChild}
      className={clsx(
        "px-4 py-1 rounded-3xl outline-none transition-all duration-100",
        "text-start leading-snug",
        danger
          ? [
              "text-red-500 bg-red-500/2 hover:bg-red-500/10 active:bg-red-500/15",
              "dark:text-red-400 dark:bg-red-300/5 dark:hover:bg-red-300/10 dark:active:bg-red-300/20",
            ]
          : [
              "hover:bg-gray-200 active:bg-gray-300 dark:hover:bg-white/5 dark:active:bg-white/10",
            ],
        className && className,
      )}
    >
      {children}
    </Dropdown.Item>
  );
}

function DropdownLabel({ children }: PropsWithChildren) {
  return (
    <Dropdown.Label className="px-4 text-sm mb-1 text-gray-600 dark:text-gray-300">
      {children}
    </Dropdown.Label>
  );
}
