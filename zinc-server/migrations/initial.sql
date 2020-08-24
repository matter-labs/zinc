CREATE SCHEMA IF NOT EXISTS sandbox;

CREATE TABLE IF NOT EXISTS sandbox.templates (
    account_id         INTEGER,

    name               TEXT NOT NULL,
    version            TEXT NOT NULL,

    bytecode           BYTEA NOT NULL,
    storage_type       JSON NOT NULL,
    verifying_key      BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY (account_id),
    UNIQUE (name, version)
);

CREATE TABLE IF NOT EXISTS sandbox.methods (
    template_id        INTEGER,
    name               TEXT,

    is_mutable         BOOLEAN NOT NULL,

    input_type         JSON NOT NULL,
    output_type        JSON NOT NULL,

    PRIMARY KEY        (template_id, name),
    CONSTRAINT fk_template_id
        FOREIGN KEY (template_id)
            REFERENCES sandbox.templates(account_id)
);

CREATE TABLE IF NOT EXISTS sandbox.contracts (
    account_id         INTEGER,

    template_id        INTEGER NOT NULL,
    eth_address        BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY        (account_id),
    CONSTRAINT fk_template_id
        FOREIGN KEY (template_id)
            REFERENCES sandbox.templates(account_id)
);

CREATE TABLE IF NOT EXISTS sandbox.fields (
    index              INTEGER,
    account_id         INTEGER,

    name               TEXT NOT NULL,
    value              JSON NOT NULL,

    PRIMARY KEY        (index, account_id),
    CONSTRAINT fk_account_id
        FOREIGN KEY (account_id)
            REFERENCES sandbox.contracts(account_id)
);
