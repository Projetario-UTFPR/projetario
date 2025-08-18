export type Paginacao<Entidade> = {
  total: number;
  paginaAtual: number;
  ultimaPagina: number;
  primeiraPagina: number;
  porPagina: number;
  dados: Array<Entidade>;
};
