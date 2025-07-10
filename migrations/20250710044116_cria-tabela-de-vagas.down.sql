-- Add down migration script here
DROP INDEX IF EXISTS idx_vaga_coordenador;
DROP INDEX IF EXISTS idx_vaga_projeto;
DROP TABLE IF EXISTS vaga;