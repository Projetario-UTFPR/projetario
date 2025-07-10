-- Add up migration script here
CREATE TABLE vaga (
    id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
    id_projeto          UUID            NOT NULL,
    id_coordenador      UUID            NOT NULL,
    id_vice_coordenador UUID,
    horas_por_semana    SMALLINT        NOT NULL,
    imagem              VARCHAR(200),
    quantidade          SMALLINT        NOT NULL,
    link_edital         VARCHAR(300)    NOT NULL,
    link_candidatura    VARCHAR(300),
    titulo              VARCHAR(100)    NOT NULL,
    conteudo            TEXT            NOT NULL,
    iniciada_em         DATE            NOT NULL,
    inscricoes_ate      TIMESTAMP       NOT NULL,
    cancelada_em        TIMESTAMP,
    atualizada_em       TIMESTAMP,

    CONSTRAINT vaga_pk PRIMARY KEY (id),
    CONSTRAINT projeto_fk FOREIGN KEY (id_projeto) REFERENCES projeto(id) ON DELETE CASCADE,
    CONSTRAINT coordenador_fk FOREIGN KEY (id_coordenador) REFERENCES usuario(id) ON DELETE CASCADE,
    CONSTRAINT vice_coordenador_fk FOREIGN KEY (id_vice_coordenador) REFERENCES usuario(id) ON DELETE SET NULL
);

CREATE INDEX idx_vaga_projeto ON vaga (id_projeto);
CREATE INDEX idx_vaga_coordenador ON vaga (id_coordenador);
