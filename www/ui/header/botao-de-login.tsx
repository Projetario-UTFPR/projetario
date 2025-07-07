import { Link, usePage } from "@inertiajs/react";
import clsx from "clsx";

export function BotaoDeLogin() {
  const { autenticacao } = usePage().props;

  if (autenticacao) return null;

  return (
    <Link
      href={"/autenticacao/login"}
      className={clsx(
        "transition-all duration-100 px-8 py-4 rounded-3xl text-white font-bold",
        "text-shadow-black/15 text-shadow-[1px_2px_4px_var(--tw-text-shadow-color)] leading-none",
        "bg-yellow-800 not-dark:hover:brightness-95 not-dark:active:brightness-90",
        "dark:border dark:border-white/10 dark:bg-yellow-500/10 dark:text-yellow-500",
        "dark:hover:bg-yellow-500/20 dark:active:bg-yellow-500/25",
      )}
    >
      Login
    </Link>
  );
}
