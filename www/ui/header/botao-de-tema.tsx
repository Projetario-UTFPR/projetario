import { MoonStarsIcon } from "@phosphor-icons/react/dist/ssr/MoonStars";
import { SunIcon } from "@phosphor-icons/react/dist/ssr/Sun";
import clsx from "clsx";

export type Tema = "claro" | "escuro";

export function BotaoDeTema() {
  const tema: Tema = "claro";
  const IconeTema = tema === "claro" ? SunIcon : MoonStarsIcon;

  return (
    <button
      type="button"
      className={clsx(
        "p-3 bg-yellow-800/10 rounded-3xl text-yellow-800",
        "dark:bg-yellow-500/5 text-yellow-500",
      )}
    >
      <IconeTema size={24} weight="fill" />
    </button>
  );
}
