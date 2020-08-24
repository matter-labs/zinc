//!
//! The Zinc database asynchronous client.
//!

use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

use crate::database::model::contract::insert::input::Input as ContractInsertInput;
use crate::database::model::field::insert::input::Input as FieldInsertInput;
use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
use crate::database::model::field::update::input::Input as FieldUpdateInput;
use crate::database::model::method::insert::input::Input as MethodInsertInput;
use crate::database::model::method::select::types::Input as MethodSelectTypesInput;
use crate::database::model::method::select::types::Output as MethodSelectTypesOutput;
use crate::database::model::template::insert::input::Input as TemplateInsertInput;
use crate::database::model::template::select::single::Input as TemplateSelectInput;
use crate::database::model::template::select::single::Output as TemplateSelectOutput;

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
    /// Select a template from the `templates` table.
    ///
    pub async fn select_template(
        &self,
        input: TemplateSelectInput,
    ) -> Result<TemplateSelectOutput, sqlx::Error> {
        const STATEMENT: &str = r#"
        SELECT
            bytecode, verifying_key
        FROM sandbox.templates
        WHERE
            account_id = $1;
        "#;

        Ok(sqlx::query_as(STATEMENT)
            .bind(&input.account_id)
            .fetch_one(&self.pool)
            .await?)
    }

    ///
    /// Inserts a template into the `templates` table.
    ///
    pub async fn insert_template(&self, input: TemplateInsertInput) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        INSERT INTO sandbox.templates (
            account_id,
            name,
            version,
            bytecode,
            storage_type,
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
        );
        "#;

        sqlx::query(STATEMENT)
            .bind(input.account_id)
            .bind(input.name)
            .bind(input.version)
            .bind(input.bytecode)
            .bind(input.storage_type)
            .bind(input.verifying_key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    ///
    /// Inserts the template methods into the `methods` table.
    ///
    pub async fn insert_methods(&self, input: Vec<MethodInsertInput>) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        INSERT INTO sandbox.methods (
            template_id,
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
                .bind(entry.template_id)
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
            templates.storage_type
        FROM sandbox.methods
        LEFT JOIN sandbox.templates ON methods.template_id = templates.account_id
        WHERE
            methods.template_id = $1 AND methods.name = $2;
        "#;

        Ok(sqlx::query_as(STATEMENT)
            .bind(&input.template_id)
            .bind(&input.name)
            .fetch_one(&self.pool)
            .await?)
    }

    ///
    /// Inserts a contract instance into the `contracts` table.
    ///
    pub async fn insert_contract(&self, input: ContractInsertInput) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        INSERT INTO sandbox.contracts (
            account_id,
            template_id,
            eth_address,
            created_at
        ) VALUES (
            $1,
            $2,
            $3,
            NOW()
        );
        "#;

        sqlx::query(STATEMENT)
            .bind(input.account_id)
            .bind(input.template_id)
            .bind(input.eth_address)
            .execute(&self.pool)
            .await?;

        Ok(())
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
        FROM sandbox.fields
        WHERE
            account_id = $1
        ORDER BY index;
        "#;

        Ok(sqlx::query_as(STATEMENT)
            .bind(input.account_id)
            .fetch_all(&self.pool)
            .await?)
    }

    ///
    /// Inserts contract storage fields into the `fields` table.
    ///
    pub async fn insert_fields(&self, input: Vec<FieldInsertInput>) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        INSERT INTO sandbox.fields (
            index,
            account_id,
            name,
            value
        ) VALUES (
            $1,
            $2,
            $3,
            $4
        );
        "#;

        for field in input.into_iter() {
            sqlx::query(STATEMENT)
                .bind(field.index)
                .bind(field.account_id)
                .bind(field.name)
                .bind(field.value)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    ///
    /// Updates contract storage fields in the `fields` table.
    ///
    pub async fn update_fields(&self, input: Vec<FieldUpdateInput>) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        UPDATE sandbox.fields
        SET
            value = $3
        WHERE
            index = $1
        AND account_id = $2;
        "#;

        for field in input.into_iter() {
            sqlx::query(STATEMENT)
                .bind(field.index)
                .bind(field.account_id)
                .bind(field.value)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}
