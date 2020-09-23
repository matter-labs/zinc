CREATE SCHEMA IF NOT EXISTS zandbox;

CREATE TABLE IF NOT EXISTS zandbox.contracts (
    address            BYTEA NOT NULL,

    name               TEXT NOT NULL,
    version            TEXT NOT NULL,
    instance           TEXT NOT NULL,

    zinc_version       TEXT NOT NULL,
    source_code        JSON NOT NULL,
    bytecode           BYTEA NOT NULL,
    verifying_key      BYTEA NOT NULL,

    account_id         BIGINT,
    eth_private_key    BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY        (address),

    CONSTRAINT unq_name_version_instance
        UNIQUE (name, version, instance)
);

CREATE TABLE IF NOT EXISTS zandbox.fields (
    address            BYTEA NOT NULL,
    index              SMALLINT NOT NULL,

    name               TEXT NOT NULL,
    value              JSON NOT NULL,

    PRIMARY KEY        (address, index),

    CONSTRAINT fk_address
        FOREIGN KEY (address)
            REFERENCES zandbox.contracts(address)
);
