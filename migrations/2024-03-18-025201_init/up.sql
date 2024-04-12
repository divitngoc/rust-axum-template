-- Create the user table
CREATE TABLE swift_user
(
    id             UUID PRIMARY KEY,
    email          TEXT      NOT NULL UNIQUE,
    password       TEXT,
    is_super_admin BOOLEAN   NOT NULL DEFAULT false,
    first_name     TEXT      NOT NULL,
    last_name      TEXT,
    created_at     TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE organisation_user_role
(
    id   BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO organisation_user_role (name)
VALUES ('Admin'),
       ('User'),
       ('Read-Only');

CREATE TABLE organisation
(
    id          UUID PRIMARY KEY,
    owner       UUID      NOT NULL,
    name        TEXT      NOT NULL,
    is_archived BOOLEAN   NOT NULL DEFAULT false,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_organisation_owner FOREIGN KEY (owner) REFERENCES swift_user (id) ON DELETE CASCADE
);
CREATE INDEX idx_organisation_is_archived ON organisation(is_archived);

CREATE TABLE swift_user_accessible_organisation
(
    organisation_id UUID,
    swift_user_id   UUID,
    role_id         BIGINT NOT NULL,
    PRIMARY KEY (organisation_id, swift_user_id),
    CONSTRAINT fk_swift_user_cr_organisation_organisation_id FOREIGN KEY (organisation_id) REFERENCES organisation (id) ON DELETE CASCADE,
    CONSTRAINT fk_swift_user_cr_organisation_swift_user_id FOREIGN KEY (swift_user_id) REFERENCES swift_user (id) ON DELETE CASCADE,
    CONSTRAINT fk_swift_user_cr_organisation_role_id FOREIGN KEY (role_id) REFERENCES organisation_user_role (id)
);
CREATE INDEX idx_swift_user_accessible_organisation_organisation_id ON swift_user_accessible_organisation(organisation_id);
CREATE INDEX idx_swift_user_accessible_organisation_swift_user_id ON swift_user_accessible_organisation(swift_user_id);

CREATE TABLE application
(
    id              UUID PRIMARY KEY,
    organisation_id UUID      NOT NULL,
    name            TEXT      NOT NULL,
    description     TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_application_organisation FOREIGN KEY (organisation_id) REFERENCES organisation (id) ON DELETE CASCADE
);
CREATE INDEX idx_application_organisation_id ON application(organisation_id);