import type { TipoDeProjeto } from "../enums/tipo-de-projeto";
import type { UsuarioModelo } from "./usuario-modelo";

export type PreviewDeVaga = {
  id: string;
  imagem: string;
  titulo: string;
  conteudo: string;
  iniciadaEm: Date | string;
  tipoDoProjeto: TipoDeProjeto;
  coordenador: UsuarioModelo;
  viceCoordenador: UsuarioModelo;
};
