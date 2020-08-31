CREATE SCHEMA IF NOT EXISTS sandbox;

CREATE TABLE IF NOT EXISTS sandbox.contracts (
    account_id         INTEGER,

    name               TEXT NOT NULL,
    version            TEXT NOT NULL,

    source_code        JSON NOT NULL,
    bytecode           BYTEA NOT NULL,
    storage_type       JSON NOT NULL,
    verifying_key      BYTEA NOT NULL,
    eth_address        BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY        (account_id),
    CONSTRAINT unq_name_version
        UNIQUE (name, version)
);

CREATE TABLE IF NOT EXISTS sandbox.methods (
    contract_id        INTEGER,
    name               TEXT,

    is_mutable         BOOLEAN NOT NULL,
    input_type         JSON NOT NULL,
    output_type        JSON NOT NULL,

    PRIMARY KEY        (contract_id, name),
    CONSTRAINT fk_contract_id
        FOREIGN KEY (contract_id)
            REFERENCES sandbox.contracts(account_id)
);

CREATE TABLE IF NOT EXISTS sandbox.fields (
    contract_id        INTEGER,
    index              INTEGER,

    name               TEXT NOT NULL,
    value              JSON NOT NULL,

    PRIMARY KEY        (contract_id, index),
    CONSTRAINT fk_contract_id
        FOREIGN KEY (contract_id)
            REFERENCES sandbox.contracts(account_id)
);
