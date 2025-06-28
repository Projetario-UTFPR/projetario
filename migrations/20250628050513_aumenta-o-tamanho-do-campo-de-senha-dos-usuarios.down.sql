-- Add down migration script here
ALTER TABLE "usuario"
MODIFY COLUMN 'senha_hash' VARCHAR(64);