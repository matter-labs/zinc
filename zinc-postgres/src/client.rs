//!
//! The Zinc PostgreSQL asynchronous client.
//!

use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

use crate::error::Error;
use crate::model::entry::insert::input::Input as EntryInsertInput;
use crate::model::entry::select::templates::Input as EntrySelectTemplatesInput;
use crate::model::entry::select::templates::Output as EntrySelectTemplatesOutput;
use crate::model::program::insert::input::Input as ProgramInsertInput;
use crate::model::program::select::all::Output as ProgramSelectAllOutput;
use crate::model::program::select::source::Input as ProgramSelectSourceInput;
use crate::model::program::select::source::Output as ProgramSelectSourceOutput;

///
/// The PostgreSQL asynchronous client adapter.
///
pub struct Client {
    /// The PostgreSQL connection pool.
    pool: Pool<Postgres>,
}

impl Client {
    ///
    /// Initializes a client instance.
    ///
    pub async fn new(
        host: String,
        port: u16,
        user: String,
        password: String,
        database: String,
    ) -> Result<Self, Error> {
        let url = format!(
            "{}://{}:{}@{}:{}/{}",
            zinc_const::postgresql::PROTOCOL,
            user,
            password,
            host,
            port,
            database
        );

        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(&url)
            .await?;

        Ok(Self { pool })
    }

    ///
    /// Selects programs from the `programs` table.
    ///
    pub async fn select_programs_all(&self) -> Result<Vec<ProgramSelectAllOutput>, Error> {
        const STATEMENT: &str = r#"
        SELECT
            id,
            name,
            version
        FROM contract.programs;
        "#;

        Ok(sqlx::query_as(STATEMENT).fetch_all(&self.pool).await?)
    }

    ///
    /// Select the program source code from the `programs` table.
    ///
    pub async fn select_program_source(
        &self,
        input: ProgramSelectSourceInput,
    ) -> Result<ProgramSelectSourceOutput, Error> {
        const STATEMENT: &str = r#"
        SELECT
            source
        FROM contract.programs
        WHERE
            id = $1;
        "#;

        Ok(sqlx::query_as(STATEMENT)
            .bind(&input.id)
            .fetch_one(&self.pool)
            .await?)
    }

    ///
    /// Inserts a program into the `programs` table.
    ///
    pub async fn insert_program(&self, input: ProgramInsertInput) -> Result<i32, Error> {
        const STATEMENT: &str = r#"
        INSERT INTO contract.programs (
            name,
            version,

            source,
            storage_type,

            proving_key,
            verifying_key,

            created_at
        ) VALUES (
            $1,
            $2,

            $3,
            $4,

            $5,
            $6,

            NOW()
        ) RETURNING id;
        "#;

        let row: (i32,) = sqlx::query_as(STATEMENT)
            .bind(input.name)
            .bind(input.version)
            .bind(input.source)
            .bind(input.storage_type)
            .bind(input.proving_key)
            .bind(input.verifying_key)
            .fetch_one(&self.pool)
            .await?;

        let program_id = row.0;

        Ok(program_id)
    }

    ///
    /// Inserts the program entries into the `entries` table.
    ///
    pub async fn insert_entries(&self, input: Vec<EntryInsertInput>) -> Result<(), Error> {
        const STATEMENT: &str = r#"
        INSERT INTO contract.entries (
            program_id,

            name,
            is_mutable,

            input_type,
            output_type
        ) VALUES (
            $1,

            $2,
            $3,

            $4,
            $5
        );
        "#;

        for entry in input.into_iter() {
            sqlx::query(STATEMENT)
                .bind(entry.program_id)
                .bind(entry.name)
                .bind(entry.is_mutable)
                .bind(entry.input_type)
                .bind(entry.output_type)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    ///
    /// Select the entry templates source code from the `entries` table.
    ///
    pub async fn select_entry_templates(
        &self,
        input: EntrySelectTemplatesInput,
    ) -> Result<EntrySelectTemplatesOutput, Error> {
        const STATEMENT: &str = r#"
        SELECT
            entries.input_type,
            entries.output_type,
            programs.storage_type
        FROM contract.entries
        LEFT JOIN contract.programs ON entries.program_id = programs.id
        WHERE
            entries.id = $1;
        "#;

        Ok(sqlx::query_as(STATEMENT)
            .bind(&input.id)
            .fetch_one(&self.pool)
            .await?)
    }
}
