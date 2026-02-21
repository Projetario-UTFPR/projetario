export const Rotas = Object.freeze({
  inicio: () => `/`,
  autenticacao: {
    login: () => `/autenticacao/login`,
    logout: () => `/autenticacao/logout`,
  },
  vagas: {
    listar: () => `/vagas`,
  },
  usuarios: {
    perfil: (idUsuario: string) => `${idUsuario}`,
  },
  professores: {
    projetos: {
      novoProjetoDeExtensao: () => `/professores/projetos/extensao/novo`,
    },
    vagas: {
      nova: () => `/professores/vagas/nova`,
    },
  },
});
