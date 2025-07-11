use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::dominio::identidade::enums::cargo::Cargo;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

#[derive(Debug, FromRow, Clone, PartialEq)]
pub struct Usuario {
    pub(super) id: Uuid,
    pub(super) nome: String,
    pub(super) email: String,
    pub(super) senha_hash: String,
    pub(super) url_curriculo_lattes: Option<String>,
    pub(super) registrado_em: NaiveDateTime,
    pub(super) atualizado_em: Option<NaiveDateTime>,
    pub(super) desativado_em: Option<NaiveDateTime>,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
/// `UsuarioModelo` é a representação completa da tabela "usuarios" do banco de dados.
pub struct UsuarioModelo {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub url_curriculo_lattes: Option<String>,
    pub cargo: Cargo,
    pub registrado_em: NaiveDateTime,
    pub atualizado_em: Option<NaiveDateTime>,
    pub desativado_em: Option<NaiveDateTime>,
    pub registro_aluno: Option<String>,
    pub periodo: Option<i16>,
}

impl Usuario {
    pub(super) fn novo(
        nome: String,
        email: String,
        senha_hash: String,
        url_curriculo_lattes: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            nome,
            email,
            senha_hash,
            url_curriculo_lattes,
            atualizado_em: None,
            desativado_em: None,
            registrado_em: Utc::now().naive_utc(),
        }
    }

    /// Desativa um usuario permanentemente na plataforma, tornando impossível
    /// identificar-se como esta na plataforma.
    pub fn desativar(&mut self) { self.desativado_em = Some(Utc::now().naive_utc()); }
}

// getters
impl Usuario {
    pub fn obtenha_id(&self) -> &Uuid { &self.id }

    pub fn obtenha_nome(&self) -> &str { &self.nome }

    pub fn obtenha_email(&self) -> &str { &self.email }

    pub fn obtenha_hash_da_senha(&self) -> Option<&str> {
        if !self.esta_ativo() {
            return None;
        }

        Some(&self.senha_hash)
    }

    pub fn obtenha_url_do_curriculo_lattes(&self) -> Option<&str> {
        self.url_curriculo_lattes.as_deref()
    }

    pub fn obtenha_data_de_registro(&self) -> NaiveDateTime { self.registrado_em }

    pub fn obtenha_data_de_modificacao(&self) -> Option<NaiveDateTime> { self.atualizado_em }

    pub fn esta_ativo(&self) -> bool { self.desativado_em.is_none() }
}

// setters
//
// só deve atualizar uma propriedade se o novo valor for diferente do atual para evitar
// alterar a data de última modificação da entidade.
impl Usuario {
    pub fn coloque_nome(&mut self, nome: String) {
        if self.nome == nome {
            return;
        }

        self.nome = nome;
        self.toque();
    }

    pub fn coloque_senha(&mut self, hash: String) -> Result<(), ErroDeDominio> {
        if !self.esta_ativo() {
            return Err(ErroDeDominio::integridade(
                "Não é possível alterar a senha de um usuário desativada.",
            ));
        }

        self.senha_hash = hash;
        self.toque();

        Ok(())
    }

    pub fn coloque_email(&mut self, email: String) {
        if self.email == email {
            return;
        }

        self.email = email;
        self.toque();
    }

    pub fn coloque_url_curriculo_lattes(&mut self, url: String) {
        if let Some(ref url_atual) = self.url_curriculo_lattes {
            if url.eq(url_atual) {
                return;
            }
        }

        self.url_curriculo_lattes = Some(url);
        self.toque();
    }

    pub fn remova_url_curriculo_lattes(&mut self) {
        if self.url_curriculo_lattes.is_none() {
            return;
        }

        self.url_curriculo_lattes = None;
        self.toque();
    }

    /// Marca a estrutura como modificada permanentemente.
    pub(super) fn toque(&mut self) { self.atualizado_em = Some(Utc::now().naive_utc()); }
}
