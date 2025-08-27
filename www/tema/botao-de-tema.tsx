import { type Page, type PageProps, router } from "@inertiajs/core";
import { usePage } from "@inertiajs/react";
import type { Icon } from "@phosphor-icons/react/dist/lib/types";
import { LaptopIcon } from "@phosphor-icons/react/dist/ssr/Laptop";
import { MoonStarsIcon } from "@phosphor-icons/react/dist/ssr/MoonStars";
import { SunIcon } from "@phosphor-icons/react/dist/ssr/Sun";
import clsx from "clsx";
import { useEffect, useState } from "react";
import { atualizarHtmlComNovoTema, TEMAS_DO_SISTEMA, type Tema } from "@/tema";

function onSuccess(page: Page<PageProps>) {
  atualizarHtmlComNovoTema(page.props.temaPreferido, page.props.temaSistema);
}

export function BotaoDeTema() {
  const [processando, coloqueProcessando] = useState(false);
  const tema = usePage().props.temaPreferido;
  const IconeTema = icone(tema);
  const only = ["temaSistema", "temaPreferido"];

  const alterarTema = () => {
    router.post("/acoes/tema/proximo", undefined, {
      errorBag: "temaPrincipal",
      only,
      onBefore: () => coloqueProcessando(true),
      onFinish: () => coloqueProcessando(false),
      onSuccess,
    });
  };

  useEffect(() => {
    const themeMatch = window.matchMedia("(prefers-color-scheme: dark)");

    const cb = () => {
      router.post(
        "/acoes/tema/sistema",
        {
          tema: themeMatch.matches
            ? TEMAS_DO_SISTEMA.Escuro
            : TEMAS_DO_SISTEMA.Claro,
        },
        {
          errorBag: "temaSistema",
          only,
          onBefore: () => coloqueProcessando(true),
          onFinish: () => coloqueProcessando(false),
          onSuccess,
        },
      );
    };

    cb();

    themeMatch.addEventListener("change", cb);

    return () => {
      themeMatch.removeEventListener("change", cb);
    };
  }, []);

  return (
    <button
      type="button"
      className={clsx(
        "p-3 bg-yellow-800/10 rounded-3xl text-yellow-800",
        "dark:bg-yellow-500/5 text-yellow-500 disabled:opacity-50",
      )}
      disabled={processando}
      aria-busy={processando}
      onClick={alterarTema}
    >
      <IconeTema size={24} weight="fill" />
    </button>
  );
}

function icone(tema: Tema): Icon {
  switch (tema) {
    case "claro":
      return SunIcon;
    case "escuro":
      return MoonStarsIcon;
    case "sistema":
      return LaptopIcon;
  }
}
