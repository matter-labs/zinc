CREATE SCHEMA IF NOT EXISTS contract;

CREATE TABLE IF NOT EXISTS contract.programs (
    id                 SERIAL,

    name               TEXT NOT NULL,
    version            TEXT NOT NULL,

    source             JSON NOT NULL,
    storage_type       JSON NOT NULL,

    proving_key        BYTEA NOT NULL,
    verifying_key      BYTEA NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY (id),
    UNIQUE (name, version)
);

CREATE TABLE IF NOT EXISTS contract.entries (
    id                 SERIAL,

    program_id         INTEGER,

    name               TEXT NOT NULL,
    is_mutable         BOOLEAN,

    input_type         JSON NOT NULL,
    output_type        JSON NOT NULL,

    PRIMARY KEY        (id),
    CONSTRAINT fk_program_id
        FOREIGN KEY (program_id)
            REFERENCES contract.programs(id)
);

CREATE TABLE IF NOT EXISTS contract.instances (
    id                 SERIAL,

    program_id         INTEGER NOT NULL,
    owner_address      VARCHAR(40) NOT NULL,

    created_at         TIMESTAMP NOT NULL,

    PRIMARY KEY        (id),
    CONSTRAINT fk_program_id
        FOREIGN KEY (program_id)
            REFERENCES contract.programs(id)
);

CREATE TABLE IF NOT EXISTS contract.fields (
    index              INTEGER,
    instance_id        INTEGER,

    name               TEXT NOT NULL,
    value              JSON NOT NULL,

    PRIMARY KEY        (index, instance_id),
    CONSTRAINT fk_instance_id
        FOREIGN KEY (instance_id)
            REFERENCES contract.instances(id)
);
