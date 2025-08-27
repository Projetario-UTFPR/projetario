export const TEMAS_DO_SISTEMA = Object.freeze({
  Escuro: "escuro",
  Claro: "claro",
  Sistema: "sistema",
});

export type Tema = (typeof TEMAS_DO_SISTEMA)[keyof typeof TEMAS_DO_SISTEMA];
export type TemaDoSistema = Exclude<Tema, "sistema">;

export function atualizarHtmlComNovoTema(
  tema: Tema,
  temaPreferido: TemaDoSistema | null,
) {
  const ehTemaEscuro =
    tema === "escuro" || (tema === "sistema" && temaPreferido === "escuro");

  const metodo = ehTemaEscuro ? "add" : "remove";
  window.document.documentElement.classList[metodo]?.("dark");
}
