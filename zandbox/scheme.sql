CREATE SCHEMA IF NOT EXISTS zandbox;

CREATE TABLE IF NOT EXISTS zandbox.contracts (
    contract_id        BIGINT,

    name               TEXT NOT NULL,
    version            TEXT NOT NULL,

    source_code        JSON NOT NULL,
    storage_type       JSON NOT NULL,
    verifying_key      BYTEA NOT NULL,
    eth_address        BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY        (contract_id)
);

CREATE TABLE IF NOT EXISTS zandbox.methods (
    contract_id        BIGINT,
    name               TEXT,

    is_mutable         BOOLEAN NOT NULL,
    input_type         JSON NOT NULL,
    output_type        JSON NOT NULL,

    PRIMARY KEY        (contract_id, name),
    CONSTRAINT fk_contract_id
        FOREIGN KEY (contract_id)
            REFERENCES zandbox.contracts(contract_id)
);

CREATE TABLE IF NOT EXISTS zandbox.fields (
    contract_id        BIGINT,
    index              SMALLINT,

    name               TEXT NOT NULL,
    value              JSON NOT NULL,

    PRIMARY KEY        (contract_id, index),
    CONSTRAINT fk_contract_id
        FOREIGN KEY (contract_id)
            REFERENCES zandbox.contracts(contract_id)
);
