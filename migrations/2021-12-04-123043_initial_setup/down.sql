DROP TABLE users;
DROP TABLE sessions;
DROP TABLE vaults;

REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM keyvault_app;
REVOKE ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public FROM keyvault_app;
REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM keyvault_auth;
REVOKE ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public FROM keyvault_auth;