//!
//! The Zandbox server daemon contract storage utils.
//!

pub mod keeper;

use crate::database::model;

///
/// The Zandbox contract storage wrapper.
///
#[derive(Debug, Clone)]
pub struct Storage {
    /// The contract storage fields.
    pub fields: Vec<zinc_types::ContractFieldValue>,
}

impl Storage {
    ///
    /// Populates the storage with the default data.
    ///
    pub fn new(types: &[zinc_types::ContractFieldType]) -> Self {
        let mut fields = Vec::with_capacity(types.len());

        for r#type in types.iter() {
            fields.push(zinc_types::ContractFieldValue::new(
                r#type.name.to_owned(),
                zinc_types::Value::new(r#type.r#type.to_owned()),
                r#type.is_public,
                r#type.is_implicit,
            ));
        }

        Self { fields }
    }

    ///
    /// Populates the storage with the database data and data from other sources.
    ///
    /// The `address` field at the index `0` is taken from the Zandbox in-memory cache.
    /// The `balances` field at the index `1` is populated from the zkSync account info.
    ///
    pub async fn new_with_data(
        database_fields: Vec<model::field::select::Output>,
        types: &[zinc_types::ContractFieldType],
        address: zksync_types::Address,
        wallet: &zksync::Wallet<zksync_eth_signer::PrivateKeySigner, zksync::RpcProvider>,
    ) -> Result<Self, zksync::error::ClientError> {
        let mut fields = Vec::with_capacity(database_fields.len());

        fields.push(zinc_types::ContractFieldValue::new(
            zinc_const::contract::FIELD_NAME_ADDRESS.to_owned(),
            zinc_types::Value::try_from_typed_json(
                serde_json::to_value(address).expect(zinc_const::panic::DATA_CONVERSION),
                types[zinc_const::contract::FIELD_INDEX_ADDRESS]
                    .r#type
                    .to_owned(),
            )
            .expect(zinc_const::panic::DATA_CONVERSION),
            true,
            true,
        ));

        let account_info = wallet.account_info().await?;
        let mut balances = Vec::with_capacity(account_info.committed.balances.len());
        for (symbol, balance) in account_info.committed.balances.into_iter() {
            let token = wallet
                .tokens
                .resolve(zksync_types::TokenLike::Symbol(symbol))
                .ok_or(zksync::error::ClientError::UnknownToken)?;
            balances.push((token.address, balance.0.to_string()));
        }
        balances.sort_by_key(|(address, _balance)| *address);
        fields.push(zinc_types::ContractFieldValue::new(
            zinc_const::contract::FIELD_NAME_BALANCES.to_owned(),
            zinc_types::Value::try_from_typed_json(
                serde_json::Value::Array(balances.into_iter().map(|(address, balance)| serde_json::json!({"key": address, "value": balance})).collect()),
                types[zinc_const::contract::FIELD_INDEX_BALANCES]
                    .r#type
                    .to_owned(),
            )
            .expect(zinc_const::panic::DATA_CONVERSION),
            true,
            true,
        ));

        for (mut index, model::field::select::Output { name, value }) in
            database_fields.into_iter().enumerate()
        {
            index += zinc_const::contract::IMPLICIT_FIELDS_COUNT;

            let r#type = types[index].r#type.to_owned();
            let value = zinc_types::Value::try_from_typed_json(value, r#type)
                .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
            fields.push(zinc_types::ContractFieldValue::new(
                name,
                value,
                types[index].is_public,
                types[index].is_implicit,
            ));
        }

        Ok(Self { fields })
    }

    ///
    /// The build type adapter.
    ///
    pub fn from_build(build: zinc_types::Value) -> Self {
        match build {
            zinc_types::Value::Contract(fields) => Self { fields },
            _ => panic!(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION),
        }
    }

    ///
    /// Converts the storage into the INSERT query database representation.
    ///
    pub fn into_database_insert(
        self,
        account_id: zksync_types::AccountId,
    ) -> Vec<model::field::insert::Input> {
        self.fields
            .into_iter()
            .enumerate()
            .filter_map(|(index, field)| match index {
                zinc_const::contract::FIELD_INDEX_ADDRESS => None,
                zinc_const::contract::FIELD_INDEX_BALANCES => None,
                index => Some(model::field::insert::Input::new(
                    account_id,
                    index as i16,
                    field.name,
                    field.value.into_json(),
                )),
            })
            .collect()
    }

    ///
    /// Converts the storage into the UPDATE query database representation.
    ///
    pub fn into_database_update(
        self,
        account_id: zksync_types::AccountId,
    ) -> Vec<model::field::update::Input> {
        self.fields
            .into_iter()
            .enumerate()
            .filter_map(|(index, field)| match index {
                zinc_const::contract::FIELD_INDEX_ADDRESS => None,
                zinc_const::contract::FIELD_INDEX_BALANCES => None,
                index => Some(model::field::update::Input::new(
                    account_id,
                    index as i16,
                    field.value.into_json(),
                )),
            })
            .collect()
    }

    ///
    /// Wraps the fields with the VM value type.
    ///
    pub fn into_build(self) -> zinc_types::Value {
        zinc_types::Value::Contract(self.fields)
    }

    ///
    /// Wraps the fields with the VM value type, filtering out the private fields.
    ///
    pub fn into_public_build(self) -> zinc_types::Value {
        zinc_types::Value::Contract(
            self.fields
                .into_iter()
                .filter(|field| field.is_public)
                .collect(),
        )
    }
}
