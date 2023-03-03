-- migrate:up

-- The very simplest RBAC implementation, the roles get added to the organisation_users table
-- as users are added to an org.

CREATE TYPE role AS ENUM (
    'Administrator', 
    'Collaborator', 
    'SystemAdministrator'
);
COMMENT ON TYPE role IS 'Users have roles, they can be managers or administrators etc.';

CREATE TYPE permission AS ENUM (
    -- The ManageTeam permission gives the user thee ability to invite team members, 
    -- delete team members and chnage the team name
    'ManageTeam'
);
COMMENT ON TYPE permission IS 'A permission gives the user the ability to do something. i.e. Manage users.';

CREATE TABLE roles_permissions (
    role role NOT NULL,
    permission permission NOT NULL,

    PRIMARY KEY (role, permission)
);
COMMENT ON TABLE roles_permissions IS 'Maps roles to permissions. i.e. a role can have multiple permissions.';

INSERT INTO roles_permissions VALUES('Administrator', 'ManageTeam');


-- Give access to the application user.
GRANT SELECT ON roles_permissions TO cloak_application;

-- Give access to the readonly user
GRANT SELECT ON roles_permissions TO cloak_readonly;

-- migrate:down
DROP TABLE roles_permissions;
DROP TYPE role;
DROP TYPE permission;