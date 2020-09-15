//!
//! The Zinc database asynchronous client.
//!

use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

use crate::database::model::contract::insert::input::Input as ContractInsertInput;
use crate::database::model::contract::select::output::Output as ContractSelectOutput;
use crate::database::model::field::insert::input::Input as FieldInsertInput;
use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
use crate::database::model::field::update::input::Input as FieldUpdateInput;
use crate::database::model::method::insert::input::Input as MethodInsertInput;
use crate::database::model::method::select::types::Input as MethodSelectTypesInput;
use crate::database::model::method::select::types::Output as MethodSelectTypesOutput;

///
/// The database asynchronous client adapter.
///
pub struct Client {
    /// The database connection pool.
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
    ) -> Result<Self, sqlx::Error> {
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
    /// Select the contracts from the `contracts` table.
    ///
    pub async fn select_contracts(&self) -> Result<Vec<ContractSelectOutput>, sqlx::Error> {
        const STATEMENT: &str = r#"
        SELECT
            contract_id,
            name,
            version,
            bytecode,
            eth_address,
            private_key
        FROM zandbox.contracts
        ORDER BY contract_id;
        "#;

        Ok(sqlx::query_as(STATEMENT).fetch_all(&self.pool).await?)
    }

    ///
    /// Inserts a contract instance into the `contracts` table.
    ///
    pub async fn insert_contract(&self, input: ContractInsertInput) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        INSERT INTO zandbox.contracts (
            contract_id,

            name,
            version,

            zinc_version,
            source_code,
            bytecode,

            verifying_key,
            eth_address,
            public_key,
            private_key,
            created_at
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            NOW()
        );
        "#;

        sqlx::query(STATEMENT)
            .bind(input.contract_id)
            .bind(input.name)
            .bind(input.version)
            .bind(input.zinc_version)
            .bind(input.source_code)
            .bind(input.bytecode)
            .bind(input.verifying_key)
            .bind(input.eth_address.to_vec())
            .bind(input.public_key.to_vec())
            .bind(input.private_key.to_vec())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    ///
    /// Inserts the template methods into the `methods` table.
    ///
    pub async fn insert_methods(&self, input: Vec<MethodInsertInput>) -> Result<(), sqlx::Error> {
        let mut statement = String::with_capacity(input.len() * 256);
        statement.push_str(
            r#"
        INSERT INTO zandbox.methods (
            contract_id,
            name,

            is_mutable,
            input_type,
            output_type
        ) VALUES "#,
        );
        let method_count = input.len();
        for (index, method) in input.into_iter().enumerate() {
            statement.push_str(
                format!(
                    "({},'{}','{}','{}','{}'){}",
                    method.contract_id,
                    method.name,
                    if method.is_mutable { 't' } else { 'f' },
                    method.input_type,
                    method.output_type,
                    if index == method_count - 1 { ";" } else { "," }
                )
                .as_str(),
            );
        }

        sqlx::query(statement.as_str()).execute(&self.pool).await?;

        Ok(())
    }

    ///
    /// Select the method types from the `methods` table.
    ///
    pub async fn select_method_types(
        &self,
        input: MethodSelectTypesInput,
    ) -> Result<MethodSelectTypesOutput, sqlx::Error> {
        const STATEMENT: &str = r#"
        SELECT
            methods.input_type,
            methods.output_type,
            contracts.storage_type
        FROM zandbox.methods
        LEFT JOIN zandbox.contracts ON methods.contract_id = contracts.contract_id
        WHERE
            methods.contract_id = $1 AND methods.name = $2;
        "#;

        Ok(sqlx::query_as(STATEMENT)
            .bind(&input.contract_id)
            .bind(&input.name)
            .fetch_one(&self.pool)
            .await?)
    }

    ///
    /// Selects contract storage fields from the `fields` table.
    ///
    pub async fn select_fields(
        &self,
        input: FieldSelectInput,
    ) -> Result<Vec<FieldSelectOutput>, sqlx::Error> {
        const STATEMENT: &str = r#"
        SELECT
            name,
            value
        FROM zandbox.fields
        WHERE
            contract_id = $1
        ORDER BY index;
        "#;

        Ok(sqlx::query_as(STATEMENT)
            .bind(input.contract_id)
            .fetch_all(&self.pool)
            .await?)
    }

    ///
    /// Inserts contract storage fields into the `fields` table.
    ///
    pub async fn insert_fields(&self, input: Vec<FieldInsertInput>) -> Result<(), sqlx::Error> {
        let mut statement = String::with_capacity(input.len() * 256);
        statement.push_str(
            r#"
        INSERT INTO zandbox.fields (
            contract_id,
            index,

            name,
            value
        ) VALUES "#,
        );
        let field_count = input.len();
        for (index, field) in input.into_iter().enumerate() {
            statement.push_str(
                format!(
                    "({},{},'{}','{}'){}",
                    field.contract_id,
                    field.index,
                    field.name,
                    field.value,
                    if index == field_count - 1 { ";" } else { "," }
                )
                .as_str(),
            );
        }

        sqlx::query(statement.as_str()).execute(&self.pool).await?;

        Ok(())
    }

    ///
    /// Updates contract storage fields in the `fields` table.
    ///
    pub async fn update_fields(&self, input: Vec<FieldUpdateInput>) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        UPDATE zandbox.fields
        SET
            value = $3
        WHERE
            index = $1
        AND contract_id = $2;
        "#;

        for field in input.into_iter() {
            sqlx::query(STATEMENT)
                .bind(field.index)
                .bind(field.contract_id)
                .bind(field.value)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}
