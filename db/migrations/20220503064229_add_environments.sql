-- migrate:up
ALTER TABLE secrets ADD COLUMN environment_id INT NOT NULL DEFAULT 0;

CREATE TABLE environments (
    id SERIAL PRIMARY KEY, 
    vault_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    CONSTRAINT fk_vault
        FOREIGN KEY (vault_id)
        REFERENCES vaults(id) 
        ON DELETE CASCADE
);

COMMENT ON TABLE environments IS 'Contains the environments of secrets we store in a vault';
COMMENT ON COLUMN environments.vault_id IS 'The vault these environments belong to';
COMMENT ON COLUMN environments.name IS 'A user generated name for the environment';

GRANT SELECT, INSERT, UPDATE, DELETE ON environments TO cloak;
GRANT USAGE, SELECT ON environments_id_seq TO cloak;

GRANT SELECT ON environments TO cloak_readonly;
GRANT SELECT ON environments_id_seq TO cloak_readonly;

ALTER TABLE service_accounts ADD COLUMN environment_id INT;

-- migrate:down
ALTER TABLE secrets DROP COLUMN environment_id;
ALTER TABLE service_accounts DROP COLUMN environment_id;
DROP TABLE environments;