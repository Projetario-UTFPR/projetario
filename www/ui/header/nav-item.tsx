import { Link, usePage } from "@inertiajs/react";
import clsx from "clsx";

type Props = {
  label: string;
  href: string;
};

export function NavItem({ label, href }: Props) {
  const estaAtivo = usePage().url === href;

  return (
    <Link
      data-active={estaAtivo}
      className={clsx(
        "transition-all duration-150 px-6 font-medium flex items-center justify-center rounded-xl",
        "data-[active=false]:hover:bg-black/5 data-[active=true]:text-gray-600",
        "dark:text-white data-[active=false]:bg-white/5 data-[active=true]:dark:text-gray-500",
        "dark:text-shadow-[0_2px_2px_var(--tw-text-shadow-color)] dark:text-shadow-black/15",
        "data-[active=false]:active:brightness-90",
      )}
      href={href}
    >
      {label}
    </Link>
  );
}
