//!
//! The Zandbox server daemon contract storage utils.
//!

use zksync_eth_signer::PrivateKeySigner;
use zksync_types::TokenLike;

use zinc_build::ContractFieldType;
use zinc_build::ContractFieldValue;
use zinc_build::ContractFieldValue as BuildContractFieldValue;

use crate::database::model::field::insert::Input as FieldInsertInput;
use crate::database::model::field::select::Output as FieldSelectOutput;
use crate::database::model::field::update::Input as FieldUpdateInput;

///
/// The Zandbox contract storage wrapper.
///
#[derive(Debug, Clone)]
pub struct Storage {
    /// The contract storage fields.
    pub fields: Vec<ContractFieldValue>,
}

impl Storage {
    ///
    /// Populates the storage with the default data.
    ///
    pub fn new(types: &[ContractFieldType]) -> Self {
        let mut fields = Vec::with_capacity(types.len());

        for r#type in types.iter() {
            fields.push(BuildContractFieldValue::new(
                r#type.name.to_owned(),
                zinc_build::Value::new(r#type.r#type.to_owned()),
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
        database_fields: Vec<FieldSelectOutput>,
        types: &[ContractFieldType],
        address: zksync_types::Address,
        wallet: &zksync::Wallet<PrivateKeySigner>,
    ) -> Result<Self, zksync::error::ClientError> {
        let mut fields = Vec::with_capacity(database_fields.len());

        fields.push(BuildContractFieldValue::new(
            zinc_const::contract::FIELD_NAME_ADDRESS.to_owned(),
            zinc_build::Value::try_from_typed_json(
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
                .resolve(TokenLike::Symbol(symbol))
                .ok_or(zksync::error::ClientError::UnknownToken)?;
            balances.push(serde_json::json!({
                "key": token.address,
                "value": balance.0.to_string(),
            }));
        }
        fields.push(BuildContractFieldValue::new(
            zinc_const::contract::FIELD_NAME_BALANCES.to_owned(),
            zinc_build::Value::try_from_typed_json(
                serde_json::Value::Array(balances),
                types[zinc_const::contract::FIELD_INDEX_BALANCES]
                    .r#type
                    .to_owned(),
            )
            .expect(zinc_const::panic::DATA_CONVERSION),
            true,
            true,
        ));

        for (mut index, FieldSelectOutput { name, value }) in
            database_fields.into_iter().enumerate()
        {
            index += zinc_const::contract::IMPLICIT_FIELDS_COUNT;

            let r#type = types[index].r#type.to_owned();
            let value = zinc_build::Value::try_from_typed_json(value, r#type)
                .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
            fields.push(BuildContractFieldValue::new(
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
    pub fn from_build(build: zinc_build::Value) -> Self {
        match build {
            zinc_build::Value::Contract(fields) => Self { fields },
            _ => panic!(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION),
        }
    }

    ///
    /// Converts the storage into the INSERT query database representation.
    ///
    pub fn into_database_insert(
        self,
        account_id: zksync_types::AccountId,
    ) -> Vec<FieldInsertInput> {
        self.fields
            .into_iter()
            .enumerate()
            .filter_map(|(index, field)| match index {
                zinc_const::contract::FIELD_INDEX_ADDRESS => None,
                zinc_const::contract::FIELD_INDEX_BALANCES => None,
                index => Some(FieldInsertInput::new(
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
    ) -> Vec<FieldUpdateInput> {
        self.fields
            .into_iter()
            .enumerate()
            .filter_map(|(index, field)| match index {
                zinc_const::contract::FIELD_INDEX_ADDRESS => None,
                zinc_const::contract::FIELD_INDEX_BALANCES => None,
                index => Some(FieldUpdateInput::new(
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
    pub fn into_build(self) -> zinc_build::Value {
        zinc_build::Value::Contract(self.fields)
    }

    ///
    /// Wraps the fields with the VM value type, filtering out the private fields.
    ///
    pub fn into_public_build(self) -> zinc_build::Value {
        zinc_build::Value::Contract(
            self.fields
                .into_iter()
                .filter(|field| field.is_public)
                .collect(),
        )
    }
}
