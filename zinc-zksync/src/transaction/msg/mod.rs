//!
//! The transaction argument which is sent to the smart contract method.
//!

pub mod error;

use std::convert::TryFrom;

use serde::Deserialize;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use zksync_types::Address;

use self::error::Error;

///
/// The transaction argument which is sent to the smart contract method.
///
/// Represented by the implicit `zksync::msg` variable.
///
#[derive(Debug, Clone, Deserialize)]
pub struct Msg {
    /// The sender address.
    pub sender: Address,
    /// The recipient address.
    pub recipient: Address,
    /// The token address to send.
    pub token_address: Address,
    /// The amount to send.
    pub amount: num_old::BigUint,
}

impl Default for Msg {
    fn default() -> Self {
        Self {
            sender: Address::default(),
            recipient: Address::default(),
            token_address: Address::default(),
            amount: num_old::BigUint::default(),
        }
    }
}

impl Msg {
    /// The required sender address field name in the transaction structure.
    const FIELD_NAME_SENDER: &'static str = "sender";

    /// The required recipient address field name in the transaction structure.
    const FIELD_NAME_RECIPIENT: &'static str = "recipient";

    /// The required toked address field name in the transaction structure.
    const FIELD_NAME_TOKEN_ADDRESS: &'static str = "token_address";

    /// The required amount field name in the transaction structure.
    const FIELD_NAME_AMOUNT: &'static str = "amount";

    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        sender: Address,
        recipient: Address,
        token_address: Address,
        amount: num_old::BigUint,
    ) -> Self {
        Self {
            sender,
            recipient,
            token_address,
            amount,
        }
    }
}

impl TryFrom<&JsonValue> for Msg {
    type Error = Error;

    ///
    /// Checks if the `msg` JSON value is a map as passed it to the next handler below.
    ///
    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value {
            JsonValue::Object(map) => Msg::try_from(map.to_owned()),
            _ => Err(Error::ArgumentInvalidFormat(
                zinc_const::contract::TRANSACTION_VARIABLE_NAME,
            )),
        }
    }
}

impl TryFrom<JsonMap<String, JsonValue>> for Msg {
    type Error = Error;

    ///
    /// Parses the transaction from the inner JSON map.
    ///
    fn try_from(mut value: JsonMap<String, JsonValue>) -> Result<Self, Self::Error> {
        let from = value
            .remove(Self::FIELD_NAME_SENDER)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_SENDER))?;
        let from = from
            .as_str()
            .ok_or(Error::NotAString(Self::FIELD_NAME_SENDER))?;
        let from: Address = from[2..].parse().map_err(Error::SenderAddressInvalid)?;

        let to = value
            .remove(Self::FIELD_NAME_RECIPIENT)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_RECIPIENT))?;
        let to = to
            .as_str()
            .ok_or(Error::NotAString(Self::FIELD_NAME_RECIPIENT))?;
        let to: Address = to[2..].parse().map_err(Error::RecipientAddressInvalid)?;

        let token_address = value
            .remove(Self::FIELD_NAME_TOKEN_ADDRESS)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_TOKEN_ADDRESS))?;
        let token_address = token_address
            .as_str()
            .ok_or(Error::NotAString(Self::FIELD_NAME_TOKEN_ADDRESS))?;
        let token_address: Address = token_address[2..]
            .parse()
            .map_err(Error::TokenAddressInvalid)?;

        let amount = value
            .remove(Self::FIELD_NAME_AMOUNT)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_AMOUNT))?;
        let amount = amount
            .as_str()
            .ok_or(Error::NotAString(Self::FIELD_NAME_AMOUNT))?;
        let amount: num_old::BigUint = zinc_math::bigint_from_str(amount)
            .map_err(Error::AmountInvalid)?
            .to_biguint()
            .map(crate::utils::num_compat_backward)
            .expect(zinc_const::panic::DATA_CONVERSION);

        Ok(Self {
            sender: from,
            recipient: to,
            token_address,
            amount,
        })
    }
}
