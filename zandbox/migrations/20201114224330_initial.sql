CREATE SCHEMA IF NOT EXISTS zandbox;

CREATE TABLE IF NOT EXISTS zandbox.projects (
    name               TEXT,
    version            TEXT,

    zinc_version       TEXT NOT NULL,
    project            JSON NOT NULL,
    bytecode           BYTEA NOT NULL,
    verifying_key      BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY        (name, version)
);

CREATE TABLE IF NOT EXISTS zandbox.contracts (
    account_id         BIGINT,

    name               TEXT NOT NULL,
    version            TEXT NOT NULL,
    instance           TEXT NOT NULL,

    eth_address        BYTEA NOT NULL,
    eth_private_key    BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY        (account_id),
    CONSTRAINT fk_name_version
        FOREIGN KEY (name, version)
            REFERENCES zandbox.projects(name, version),
    CONSTRAINT unq_eth_address
        UNIQUE (eth_address),
    CONSTRAINT unq_name_version_instance
        UNIQUE (name, version, instance)
);

CREATE TABLE IF NOT EXISTS zandbox.fields (
    account_id         BIGINT,
    index              SMALLINT,

    name               TEXT NOT NULL,
    value              JSON NOT NULL,

    PRIMARY KEY        (account_id, index),
    CONSTRAINT fk_account_id
        FOREIGN KEY (account_id)
            REFERENCES zandbox.contracts(account_id)
);
