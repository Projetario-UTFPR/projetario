import { Link, usePage } from "@inertiajs/react";
import * as Avatar from "@radix-ui/react-avatar";
import * as Dropdown from "@radix-ui/react-dropdown-menu";
import clsx from "clsx";
import { type PropsWithChildren, useMemo } from "react";
import { cargoEhMaiorOuIgual } from "@/core/utils/hierarquia-de-cargo";

export function DropdownUsuario() {
  const autenticacao = usePage().props.autenticacao;

  const iniciaisDoNomeDoUsuario = useMemo(() => {
    return autenticacao?.usuario.nome
      .split(" ")
      .map((nome) => nome[0])
      .join("");
  }, [autenticacao]);

  if (!autenticacao) return null;

  const usuario = autenticacao.usuario;

  return (
    <Dropdown.Root>
      <Dropdown.Trigger
        className={clsx(
          "flex gap-3 items-center border border-black/10 transition-all duration-100",
          "rounded-3xl bg-gray-200 pl-4 pr-[5px] py-[5px] font-bold",
          "dark:bg-gray-800",
          "hover:bg-gray-300 dark:hover:bg-gray-600",
        )}
      >
        {usuario.nome}
        <Avatar.Root
          className={clsx(
            "size-10 rounded-full select-none outline-none overflow-hidden",
            "drop-shadow-black/5 drop-shadow-md",
          )}
        >
          <Avatar.Fallback
            className={clsx(
              "leading-1 flex size-full items-center justify-center",
              "bg-slate-600 text-[15px] font-medium text-white",
            )}
          >
            {iniciaisDoNomeDoUsuario}
          </Avatar.Fallback>
        </Avatar.Root>
      </Dropdown.Trigger>

      <Dropdown.Portal>
        <Dropdown.Content
          sideOffset={8}
          alignOffset={24}
          collisionPadding={24}
          className={clsx(
            "p-2 rounded-2xl flex flex-col gap-1.5 bg-white border border-black/10",
            "max-w-[calc(100vw_-_48px)] min-w-48 drop-shadow-black/5 drop-shadow-2xl",
          )}
        >
          {cargoEhMaiorOuIgual(usuario.cargo, "Professor") && (
            <>
              <SecaoParaProfessoresSomente />
              <hr className="text-gray-300" />
            </>
          )}

          <DropdownItem asChild>
            <Link
              href="/autenticacao/logout"
              method="post"
              className="text-red-500 bg-red-500/2 hover:bg-red-500/10 active:bg-red-500/15"
            >
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
}>;

function DropdownItem({ children, asChild, className }: DropdownItemProps) {
  return (
    <Dropdown.Item
      asChild={asChild}
      className={clsx(
        "px-4 py-1 rounded-3xl outline-none transition-all duration-100",
        "text-start hover:bg-gray-200 active:bg-gray-300 leading-snug",
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
