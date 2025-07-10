-- Add down migration script here

DROP INDEX IF EXISTS idx_coordenador_projeto;
DROP INDEX IF EXISTS coordenadores_unicos_por_projeto;

DROP TABLE IF EXISTS "coordenador_projeto";
DROP TABLE IF EXISTS "projeto";
DROP TABLE IF EXISTS "usuario";

DROP TYPE IF EXISTS tipo_coordenacao_e;
DROP TYPE IF EXISTS tipo_de_projeto_e;
DROP TYPE IF EXISTS cargo_e;
