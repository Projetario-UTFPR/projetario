-- Add down migration script here
ALTER TABLE vaga ALTER COLUMN titulo SET NOT NULL;
