//!
//! The Zandbox server daemon contract storage utils.
//!

use num::BigInt;
use num::BigUint;
use num::ToPrimitive;
use num::Zero;
use serde_json::json;

use zksync_types::TokenLike;

use zinc_build::ContractFieldType;
use zinc_build::ContractFieldValue;
use zinc_build::ContractFieldValue as BuildContractFieldValue;
use zinc_build::ScalarValue;
use zinc_build::Value as BuildValue;

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
    /// The `tokens` field index, where the ordered token ID array is stored.
    const TOKEN_ID_FIELD_INDEX: usize = 2;

    ///
    /// Populates the storage with the default data.
    ///
    pub fn new(types: &[ContractFieldType]) -> Self {
        let mut fields = Vec::with_capacity(types.len());

        for r#type in types.iter() {
            fields.push(BuildContractFieldValue::new(
                r#type.name.to_owned(),
                BuildValue::new(r#type.r#type.to_owned()),
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
    /// The `balances` field at the index `1` is built from the zkSync account info using the
    /// `tokens` field at the index `2` to preserve the correct balance order.
    ///
    pub async fn new_with_data(
        database_fields: Vec<FieldSelectOutput>,
        types: &[ContractFieldType],
        address: zksync_types::Address,
        wallet: &zksync::Wallet,
    ) -> Result<Self, zksync::error::ClientError> {
        let mut fields = Vec::with_capacity(database_fields.len());

        fields.push(BuildContractFieldValue::new(
            zinc_const::contract::FIELD_NAME_ADDRESS.to_owned(),
            BuildValue::try_from_typed_json(
                serde_json::to_value(address).expect(zinc_const::panic::DATA_CONVERSION),
                types[zinc_const::contract::FIELD_INDEX_ADDRESS]
                    .r#type
                    .to_owned(),
            )
            .expect(zinc_const::panic::DATA_CONVERSION),
            true,
            true,
        ));

        fields.push(BuildContractFieldValue::new(
            zinc_const::contract::FIELD_NAME_BALANCES.to_owned(),
            BuildValue::try_from_typed_json(
                json!(["0", "0"]),
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
            let value = BuildValue::try_from_typed_json(value, r#type)
                .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
            fields.push(BuildContractFieldValue::new(
                name,
                value,
                types[index].is_public,
                types[index].is_implicit,
            ));
        }

        let token_ids = match fields
            .get(Self::TOKEN_ID_FIELD_INDEX)
            .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
            .value
        {
            BuildValue::Array(ref array) => {
                let mut token_ids = Vec::with_capacity(array.len());
                for element in array.iter() {
                    match element {
                        BuildValue::Scalar(scalar) => token_ids.push(
                            scalar
                                .to_bigint()
                                .to_u16()
                                .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
                        ),
                        _ => panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
                    }
                }
                token_ids
            }
            _ => panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
        };

        let account_info = wallet.account_info().await?;

        match fields
            .get_mut(zinc_const::contract::FIELD_INDEX_BALANCES)
            .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
            .value
        {
            BuildValue::Array(ref mut array) => {
                for (index, element) in array.iter_mut().enumerate() {
                    match element {
                        BuildValue::Scalar(ref mut scalar) => match scalar {
                            ScalarValue::Integer(ref mut value, _) => {
                                let token = wallet
                                    .tokens
                                    .resolve(TokenLike::Id(token_ids[index]))
                                    .expect(
                                        zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION,
                                    );
                                let balance = account_info
                                    .committed
                                    .balances
                                    .get(token.symbol.as_str())
                                    .map(|balance| balance.0.to_owned())
                                    .map(zinc_utils::num_compat_forward)
                                    .unwrap_or_else(BigUint::zero);

                                *value = BigInt::from_biguint(num::bigint::Sign::Plus, balance);
                            }
                            _ => panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
                        },
                        _ => panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
                    }
                }
            }
            _ => panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
        }

        Ok(Self { fields })
    }

    ///
    /// The build type adapter.
    ///
    pub fn from_build(build: BuildValue) -> Self {
        match build {
            BuildValue::Contract(fields) => Self { fields },
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
    pub fn into_build(self) -> BuildValue {
        BuildValue::Contract(self.fields)
    }

    ///
    /// Wraps the fields with the VM value type, filtering out the private fields.
    ///
    pub fn into_public_build(self) -> BuildValue {
        BuildValue::Contract(
            self.fields
                .into_iter()
                .filter(|field| field.is_public)
                .collect(),
        )
    }
}
