use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::dominio::identidade::enums::cargo::Cargo;

#[derive(Debug)]
pub struct Pessoa {
    id: Uuid,
    nome: String,
    email: String,
    senha_hash: String,
    curriculo_lates_url: Option<String>,
    registrada_em: NaiveDateTime,
    atualizada_em: Option<NaiveDateTime>,
    ativa: bool,
}

#[derive(Debug)]
/// `PessoaModelo` é a representação completa da tabela "pessoas" do banco de dados.
pub struct PessoaModelo {
    id: Uuid,
    nome: String,
    email: String,
    senha_hash: String,
    curriculo_lates_url: Option<String>,
    cargo: Cargo,
    registrada_em: NaiveDateTime,
    atualizada_em: Option<NaiveDateTime>,
    ativa: bool,
    registro_aluno: Option<String>,
    periodo: Option<i16>,
}
