-- Add up migration script here
ALTER TABLE "usuario"
MODIFY COLUMN 'senha_hash' VARCHAR(120);