export const TEMAS_DO_SISTEMA = Object.freeze({
  Escuro: "escuro",
  Claro: "claro",
  Sistema: "sistema",
});

export type Tema = (typeof TEMAS_DO_SISTEMA)[keyof typeof TEMAS_DO_SISTEMA];
export type TemaDoSistema = Exclude<Tema, "sistema">;
export type TemaEstrito = TemaDoSistema;

export function atualizarHtmlComNovoTema(
  tema: Tema,
  temaPreferido: TemaDoSistema | null,
) {
  const ehTemaEscuro = resolvaTema(tema, temaPreferido) === "escuro";

  const metodo = ehTemaEscuro ? "add" : "remove";
  window.document.documentElement.classList[metodo]?.("dark");
}

export function resolvaTema(
  tema: Tema,
  temaDoSistema: TemaDoSistema | null,
): TemaEstrito {
  return tema === "escuro" || (tema === "sistema" && temaDoSistema === "escuro")
    ? TEMAS_DO_SISTEMA.Escuro
    : TEMAS_DO_SISTEMA.Claro;
}
