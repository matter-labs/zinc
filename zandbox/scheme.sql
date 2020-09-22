CREATE SCHEMA IF NOT EXISTS zandbox;

CREATE TABLE IF NOT EXISTS zandbox.contracts (
    account_id         BIGINT NOT NULL,
    name               TEXT NOT NULL,
    version            TEXT NOT NULL,
    instance           TEXT NOT NULL,

    zinc_version       TEXT NOT NULL,
    source_code        JSON NOT NULL,
    bytecode           BYTEA NOT NULL,
    verifying_key      BYTEA NOT NULL,

    eth_private_key    BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY        (account_id)
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
