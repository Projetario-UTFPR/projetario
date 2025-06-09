-- Add up migration script here
CREATE TYPE cargo_e AS ENUM ('aluno', 'professor', 'administrador');
CREATE TYPE tipo_de_projeto_e AS ENUM ('extensao', 'iniciacao_cientifica');
CREATE TYPE tipo_coordenacao_e AS ENUM ('coordenador', 'vice_coordenador');

CREATE TABLE "usuario" (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    -- tamanho máximo sugerido pelo governo
    -- veja: https://www.gov.br/pf/pt-br/assuntos/passaporte/ajuda/duvidas_/formulario/formulario-nome-completo-nao-cabe
    nome                    VARCHAR(80)     NOT NULL,
    email                   VARCHAR(320)    NOT NULL,
    senha_hash              VARCHAR(64)     NOT NULL,
    url_curriculo_lattes    VARCHAR(200),
    cargo                   cargo_e         NOT NULL    DEFAULT 'aluno',
    registrado_em           TIMESTAMP       NOT NULL    DEFAULT now(),
    atualizado_em           TIMESTAMP,
    desativado_em           TIMESTAMP,
    registro_aluno          VARCHAR(100),
    periodo                 SMALLINT,

    CHECK (periodo IS NULL OR (periodo >= 1 AND periodo <= 20)),

    CONSTRAINT usuario_pk       PRIMARY KEY (id),
    CONSTRAINT ra_unico         UNIQUE (registro_aluno),
    CONSTRAINT email_unico      UNIQUE (email)
);

CREATE TABLE "projeto" (
    id                  UUID                NOT NULL    DEFAULT gen_random_uuid(),
    titulo              VARCHAR(200)        NOT NULL,
    descricao           TEXT                NOT NULL    DEFAULT '',
    tipo                tipo_de_projeto_e   NOT NULL,
    registrado_em       TIMESTAMP           NOT NULL    DEFAULT now(),
    iniciado_em         DATE                NOT NULL    DEFAULT now(),
    atualizado_em       TIMESTAMP,
    cancelado_em        TIMESTAMP,
    concluido_em        DATE,

    CONSTRAINT projeto_pk   PRIMARY KEY (id)
);

CREATE TABLE "coordenador_projeto" (
    id_coordenador      UUID                NOT NULL,
    id_projeto          UUID                NOT NULL,
    tipo                tipo_coordenacao_e  NOT NULL,
    iniciado_em         DATE                NOT NULL    DEFAULT now(),
    terminado_em        DATE,

    CONSTRAINT id_coordenador_fk                FOREIGN KEY (id_coordenador) REFERENCES usuario(id) ON DELETE CASCADE,
    CONSTRAINT id_projeto_fk                    FOREIGN KEY (id_projeto)     REFERENCES projeto(id) ON DELETE CASCADE,
    CONSTRAINT coordenador_projeto_pk           PRIMARY KEY (id_coordenador, id_projeto, tipo, iniciado_em)
);

-- garante que só haja 1 coordenador e 1 vice-coordenador ativos em um projeto simultaneamente
CREATE UNIQUE INDEX coordenadores_unicos_por_projeto
    ON coordenador_projeto (id_coordenador, id_projeto, tipo)
    WHERE terminado_em IS NULL;

-- acelera a busca por coordenadores em projetos
CREATE INDEX idx_coordenador_projeto
    ON coordenador_projeto (id_coordenador, id_projeto);
