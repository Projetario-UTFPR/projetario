import type { TipoDeProjeto } from "../enums/tipo-de-projeto";

export type Projeto = {
  id: string;
  titulo: string;
  descricao: string;
  tipo: TipoDeProjeto;
  registradoEm: Date | string;
  iniciadoEm: Date | string;
  atualizadoEm: Date | string | null;
  canceladoEm: Date | string | null;
  concluidoEm: Date | string | null;
};
