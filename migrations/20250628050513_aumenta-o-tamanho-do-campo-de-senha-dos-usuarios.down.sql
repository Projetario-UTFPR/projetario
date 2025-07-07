-- Add down migration script here
ALTER TABLE "usuario"
ALTER COLUMN "senha_hash" TYPE VARCHAR(64);
