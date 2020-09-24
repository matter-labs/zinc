//!
//! The Zinc database asynchronous client.
//!

use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

use crate::database::model::contract::insert::new::input::Input as ContractInsertNewInput;
use crate::database::model::contract::select::all::output::Output as ContractSelectAllOutput;
use crate::database::model::contract::select::curve::output::Output as ContractSelectCurveOutput;
use crate::database::model::contract::update::account_id::input::Input as ContractUpdateAccountIdInput;
use crate::database::model::field::insert::input::Input as FieldInsertInput;
use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
use crate::database::model::field::update::input::Input as FieldUpdateInput;

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
    pub async fn select_contracts(&self) -> Result<Vec<ContractSelectAllOutput>, sqlx::Error> {
        const STATEMENT: &str = r#"
        SELECT
            address,
            
            name,
            version,
            instance,
            
            bytecode,
            
            eth_private_key
        FROM zandbox.contracts
        ORDER BY created_at;
        "#;

        Ok(sqlx::query_as(STATEMENT).fetch_all(&self.pool).await?)
    }

    ///
    /// Select the Curve contracts from the `contracts` table.
    ///
    pub async fn select_contracts_curve(
        &self,
    ) -> Result<Vec<ContractSelectCurveOutput>, sqlx::Error> {
        const STATEMENT: &str = r#"
        SELECT
            address,
            
            name,
            version,
            instance
        FROM zandbox.contracts
        WHERE
            name = 'curve'
        ORDER BY created_at;
        "#;

        Ok(sqlx::query_as(STATEMENT).fetch_all(&self.pool).await?)
    }

    ///
    /// Inserts a contract instance into the `contracts` table.
    ///
    pub async fn insert_contract(&self, input: ContractInsertNewInput) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        INSERT INTO zandbox.contracts (
            address,

            name,
            version,
            instance,

            zinc_version,
            source_code,
            bytecode,
            verifying_key,

            eth_private_key,

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
            NOW()
        );
        "#;

        sqlx::query(STATEMENT)
            .bind(<[u8; zinc_const::size::ETH_ADDRESS]>::from(input.address).to_vec())
            .bind(input.name)
            .bind(input.version)
            .bind(input.instance)
            .bind(input.zinc_version)
            .bind(input.source_code)
            .bind(input.bytecode)
            .bind(input.verifying_key)
            .bind(<[u8; zinc_const::size::ETH_PRIVATE_KEY]>::from(input.eth_private_key).to_vec())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    ///
    /// Sets the contract zkSync account ID in the `contracts` table.
    ///
    pub async fn update_contract_account_id(
        &self,
        input: ContractUpdateAccountIdInput,
    ) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        UPDATE zandbox.contracts
        SET
            account_id = $2
        WHERE
            address = $1;
        "#;

        sqlx::query(STATEMENT)
            .bind(<[u8; zinc_const::size::ETH_ADDRESS]>::from(input.address).to_vec())
            .bind(input.account_id)
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
        FROM zandbox.fields
        WHERE
            address = $1
        ORDER BY index;
        "#;

        Ok(sqlx::query_as(STATEMENT)
            .bind(<[u8; zinc_const::size::ETH_ADDRESS]>::from(input.address).to_vec())
            .fetch_all(&self.pool)
            .await?)
    }

    ///
    /// Inserts contract storage fields into the `fields` table.
    ///
    pub async fn insert_fields(&self, input: Vec<FieldInsertInput>) -> Result<(), sqlx::Error> {
        const STATEMENT: &str = r#"
        INSERT INTO zandbox.fields (
            address,
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
            sqlx::query(STATEMENT)
                .bind(<[u8; zinc_const::size::ETH_ADDRESS]>::from(field.address).to_vec())
                .bind(field.index)
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
        UPDATE zandbox.fields
        SET
            value = $3
        WHERE
            index = $2
        AND address = $1;
        "#;

        for field in input.into_iter() {
            sqlx::query(STATEMENT)
                .bind(<[u8; zinc_const::size::ETH_ADDRESS]>::from(field.address).to_vec())
                .bind(field.index)
                .bind(field.value)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}
