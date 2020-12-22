//!
//! The Zinc database asynchronous client.
//!

use std::ops::DerefMut;

use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use sqlx::Transaction;

use crate::database::error::Error;
use crate::database::model;

///
/// The shortcut database result type.
///
pub type Result<T> = std::result::Result<T, Error>;

///
/// The database asynchronous client adapter.
///
#[derive(Clone)]
pub struct Client {
    /// The database connection pool.
    pool: Pool<Postgres>,
}

impl Client {
    ///
    /// Initializes a client instance.
    ///
    pub async fn new(connection_uri: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(connection_uri)
            .await?;

        Ok(Self { pool })
    }

    ///
    /// Initializes a transaction.
    ///
    pub async fn new_transaction(&self) -> Result<Transaction<'static, Postgres>> {
        Ok(self.pool.begin().await?)
    }

    ///
    /// Inserts a project into the `projects` table.
    ///
    pub async fn insert_project(
        &self,
        input: model::project::insert_one::Input,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<()> {
        const STATEMENT: &str = r#"
        INSERT INTO zandbox.projects (
            name,
            version,

            zinc_version,
            project,
            bytecode,
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

        let query = sqlx::query(STATEMENT)
            .bind(input.name)
            .bind(input.version.to_string())
            .bind(input.zinc_version.to_string())
            .bind(serde_json::to_value(&input.project).expect(zinc_const::panic::DATA_CONVERSION))
            .bind(input.bytecode)
            .bind(input.verifying_key);

        match transaction {
            Some(transaction) => query.execute(transaction).await,
            None => query.execute(&self.pool).await,
        }
        .map_err(|error| (error, "project"))?;

        Ok(())
    }

    ///
    /// Selects a project from the `projects` table.
    ///
    pub async fn select_project(
        &self,
        input: model::project::select_one::Input,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<model::project::select_one::Output> {
        const STATEMENT: &str = r#"
        SELECT
            name,
            version,

            zinc_version,
            project,
            bytecode,
            verifying_key
        FROM zandbox.projects
        WHERE
            name = $1 AND version = $2;
        "#;

        let query = sqlx::query_as(STATEMENT)
            .bind(input.name)
            .bind(input.version.to_string());

        Ok(match transaction {
            Some(transaction) => query.fetch_one(transaction).await,
            None => query.fetch_one(&self.pool).await,
        }
        .map_err(|error| (error, "project"))?)
    }

    ///
    /// Selects a project source code from the `projects` table.
    ///
    pub async fn select_project_source(
        &self,
        input: model::project::select_source::Input,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<model::project::select_source::Output> {
        const STATEMENT: &str = r#"
        SELECT
            name,
            version,

            zinc_version,
            project
        FROM zandbox.projects
        WHERE
            name = $1 AND version = $2;
        "#;

        let query = sqlx::query_as(STATEMENT)
            .bind(input.name)
            .bind(input.version.to_string());

        Ok(match transaction {
            Some(transaction) => query.fetch_one(transaction).await,
            None => query.fetch_one(&self.pool).await,
        }
        .map_err(|error| (error, "project"))?)
    }

    ///
    /// Selects projects metadata from the `projects` table.
    ///
    pub async fn select_projects_metadata(
        &self,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<Vec<model::project::select_metadata::Output>> {
        const STATEMENT: &str = r#"
        SELECT
            name,
            version
        FROM zandbox.projects
        ORDER BY
            name,
            version;
        "#;

        let query = sqlx::query_as(STATEMENT);

        Ok(match transaction {
            Some(transaction) => query.fetch_all(transaction).await?,
            None => query.fetch_all(&self.pool).await?,
        })
    }

    ///
    /// Inserts a contract into the `contracts` table.
    ///
    pub async fn insert_contract(
        &self,
        input: model::contract::insert_one::Input,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<()> {
        const STATEMENT: &str = r#"
        INSERT INTO zandbox.contracts (
            account_id,

            name,
            version,
            instance,

            eth_address,
            eth_private_key,

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

        let query = sqlx::query(STATEMENT)
            .bind(input.account_id as i64)
            .bind(input.name)
            .bind(input.version.to_string())
            .bind(input.instance)
            .bind(<[u8; zinc_const::size::ETH_ADDRESS]>::from(input.eth_address).to_vec())
            .bind(<[u8; zinc_const::size::ETH_PRIVATE_KEY]>::from(input.eth_private_key).to_vec());

        match transaction {
            Some(transaction) => query.execute(transaction).await,
            None => query.execute(&self.pool).await,
        }
        .map_err(|error| (error, "contract"))?;

        Ok(())
    }

    ///
    /// Selects a contract from the `contracts` table.
    ///
    pub async fn select_contract(
        &self,
        input: model::contract::select_one::Input,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<model::contract::select_one::Output> {
        const STATEMENT: &str = r#"
        SELECT
            account_id,
            
            name,
            version,
            instance,

            eth_address,
            eth_private_key
        FROM zandbox.contracts
        WHERE
            eth_address = $1;
        "#;

        let query = sqlx::query_as(STATEMENT)
            .bind(<[u8; zinc_const::size::ETH_ADDRESS]>::from(input.eth_address).to_vec());

        Ok(match transaction {
            Some(transaction) => query.fetch_one(transaction).await,
            None => query.fetch_one(&self.pool).await,
        }
        .map_err(|error| (error, "contract"))?)
    }

    ///
    /// Select the Curve contracts from the `contracts` table.
    ///
    pub async fn select_contracts_curve(
        &self,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<Vec<model::contract::select_curve::Output>> {
        const STATEMENT: &str = r#"
        SELECT
            eth_address,

            name,
            version,
            instance
        FROM zandbox.contracts
        WHERE
            name = 'curve'
        ORDER BY created_at;
        "#;

        let query = sqlx::query_as(STATEMENT);

        Ok(match transaction {
            Some(transaction) => query.fetch_all(transaction).await?,
            None => query.fetch_all(&self.pool).await?,
        })
    }

    ///
    /// Inserts contract storage fields into the `fields` table.
    ///
    pub async fn insert_fields(
        &self,
        input: Vec<model::field::insert::Input>,
        mut transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<()> {
        const STATEMENT: &str = r#"
        INSERT INTO zandbox.fields (
            account_id,
            index,

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
            let query = sqlx::query(STATEMENT)
                .bind(field.account_id)
                .bind(field.index)
                .bind(field.name)
                .bind(field.value);

            match transaction {
                Some(ref mut transaction) => query.execute(transaction.deref_mut()).await?,
                None => query.execute(&self.pool).await?,
            };
        }

        Ok(())
    }

    ///
    /// Selects contract storage fields from the `fields` table.
    ///
    pub async fn select_fields(
        &self,
        input: model::field::select::Input,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<Vec<model::field::select::Output>> {
        const STATEMENT: &str = r#"
        SELECT
            name,
            value
        FROM zandbox.fields
        WHERE
            account_id = $1
        ORDER BY index;
        "#;

        let query = sqlx::query_as(STATEMENT).bind(input.account_id);

        Ok(match transaction {
            Some(transaction) => query.fetch_all(transaction).await?,
            None => query.fetch_all(&self.pool).await?,
        })
    }

    ///
    /// Updates contract storage fields in the `fields` table.
    ///
    pub async fn update_fields(
        &self,
        input: Vec<model::field::update::Input>,
        mut transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<()> {
        const STATEMENT: &str = r#"
        UPDATE zandbox.fields
        SET
            value = $3
        WHERE
            index = $2
        AND account_id = $1;
        "#;

        for field in input.into_iter() {
            let query = sqlx::query(STATEMENT)
                .bind(field.account_id)
                .bind(field.index)
                .bind(field.value);

            match transaction {
                Some(ref mut transaction) => query.execute(transaction.deref_mut()).await?,
                None => query.execute(&self.pool).await?,
            };
        }

        Ok(())
    }

    ///
    /// Deletes the `projects` table contents.
    ///
    /// WARNING: only for integration tests!
    ///
    pub async fn delete_projects(
        &self,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<()> {
        const STATEMENT: &str = r#"
        DELETE FROM zandbox.projects;
        "#;

        let query = sqlx::query(STATEMENT);

        match transaction {
            Some(transaction) => query.execute(transaction).await?,
            None => query.execute(&self.pool).await?,
        };

        Ok(())
    }

    ///
    /// Deletes the `contracts` table contents.
    ///
    /// WARNING: only for integration tests!
    ///
    pub async fn delete_contracts(
        &self,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<()> {
        const STATEMENT: &str = r#"
        DELETE FROM zandbox.contracts;
        "#;

        let query = sqlx::query(STATEMENT);

        match transaction {
            Some(transaction) => query.execute(transaction).await?,
            None => query.execute(&self.pool).await?,
        };

        Ok(())
    }

    ///
    /// Deletes the `fields` table contents.
    ///
    /// WARNING: only for integration tests!
    ///
    pub async fn delete_fields(
        &self,
        transaction: Option<&mut Transaction<'static, Postgres>>,
    ) -> Result<()> {
        const STATEMENT: &str = r#"
        DELETE FROM zandbox.fields;
        "#;

        let query = sqlx::query(STATEMENT);

        match transaction {
            Some(transaction) => query.execute(transaction).await?,
            None => query.execute(&self.pool).await?,
        };

        Ok(())
    }
}
