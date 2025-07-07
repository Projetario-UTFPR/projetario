import type { Cargo } from "../enums/cargo";

export type UsuarioModelo = {
  id: string;
  nome: string;
  email: string;
  urlCurriculoLattes: string;
  cargo: Cargo;
  registradoEm: Date | string;
  atualizadoEm?: null | Date | string;
  registro_aluno?: null | string;
  periodo?: null | number;
};
