export type RespostaIncertaDoServidor<T> =
  | {
      success: true;
      data: T;
    }
  | {
      success: false;
      error: string;
    };
