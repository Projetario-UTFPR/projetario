use crate::erros::erro_de_dominio::ErroDeDominio;

pub type ResultadoDominio<T> = Result<T, ErroDeDominio>;
