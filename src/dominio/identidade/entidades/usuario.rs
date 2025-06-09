use crate::dominio::identidade::enums::cargo::Cargo;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Usuario {
    pub(super) id: Uuid,
    pub(super) nome: String,
    pub(super) email: String,
    pub(super) senha_hash: String,
    pub(super) url_curriculo_lates: Option<String>,
    pub(super) registrada_em: NaiveDateTime,
    pub(super) atualizada_em: Option<NaiveDateTime>,
    pub(super) desativada_em: Option<NaiveDateTime>,
}

impl Usuario {
    pub(super) fn novo(
        nome: String,
        email: String,
        senha_hash: String,
        url_curriculo_lates: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            nome,
            email,
            senha_hash,
            url_curriculo_lates,
            atualizada_em: None,
            desativada_em: None,
            registrada_em: Utc::now().naive_utc(),
        }
    }

    /// Desativa um usuario permanentemente na plataforma, tornando impossível
    /// identificar-se como esta na plataforma.
    pub fn desativar(&mut self) {
        self.desativada_em = Some(Utc::now().naive_utc());
    }
}

// getters
impl Usuario {
    pub fn obtenha_id(&self) -> &Uuid {
        &self.id
    }

    pub fn obtenha_nome(&self) -> &str {
        &self.nome
    }

    pub fn obtenha_email(&self) -> &str {
        &self.email
    }

    pub fn obtenha_hash_da_senha(&self) -> Option<&str> {
        if !self.esta_ativo() {
            return None;
        }

        Some(&self.senha_hash)
    }

    pub fn obtenha_url_do_curriculo_lattes(&self) -> Option<&str> {
        self.url_curriculo_lates.as_deref()
    }

    pub fn obtenha_data_de_registro(&self) -> NaiveDateTime {
        self.registrada_em
    }

    pub fn obtenha_data_de_modificacao(&self) -> Option<NaiveDateTime> {
        self.atualizada_em
    }

    pub fn esta_ativo(&self) -> bool {
        self.desativada_em.is_none()
    }
}

// setters
//
// só deve atualizar uma propriedade se o novo valor for diferente do atual para evitar alterar a data de
// última modificação da entidade.
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
            return Err(ErroDeDominio::Integridade(
                "Não é possível alterar a senha de um usuário desativada.".into(),
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

    pub fn coloque_url_curriculo_lates(&mut self, url: String) {
        if let Some(ref url_atual) = self.url_curriculo_lates {
            if url.eq(url_atual) {
                return;
            }
        }

        self.url_curriculo_lates = Some(url);
        self.toque();
    }

    pub fn remova_url_curriculo_lates(&mut self) {
        if self.url_curriculo_lates.is_none() {
            return;
        }

        self.url_curriculo_lates = None;
        self.toque();
    }

    /// Marca a estrutura como modificada permanentemente.
    pub(super) fn toque(&mut self) {
        self.atualizada_em = Some(Utc::now().naive_utc());
    }
}

#[derive(Debug)]
/// `UsuarioModelo` é a representação completa da tabela "usuarios" do banco de dados.
pub struct UsuarioModelo {
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
