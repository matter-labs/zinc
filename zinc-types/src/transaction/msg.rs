//!
//! The transaction argument which is sent to the smart contract method.
//!

use std::convert::TryFrom;

use num::BigInt;
use serde::Deserialize;
use serde::Serialize;

use zksync_types::Address;

use crate::transaction::error::Error;

///
/// The transaction argument which is sent to the smart contract method.
///
/// Represented by the implicit `zksync::msg` variable.
///
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    ///
    /// A shortcut constructor for BigInt's.
    ///
    /// # Panics
    /// If any BigInt is negative or larger than 20 bytes.
    ///
    pub fn new_from_bigints(
        sender: BigInt,
        recipient: BigInt,
        token_address: BigInt,
        amount: BigInt,
    ) -> Self {
        Self {
            sender: crate::address_from_slice(sender.to_bytes_be().1.as_slice()),
            recipient: crate::address_from_slice(recipient.to_bytes_be().1.as_slice()),
            token_address: crate::address_from_slice(token_address.to_bytes_be().1.as_slice()),
            amount: amount
                .to_biguint()
                .map(crate::num_compat_backward)
                .expect(zinc_const::panic::DATA_CONVERSION),
        }
    }
}

impl TryFrom<&serde_json::Value> for Msg {
    type Error = Error;

    ///
    /// Checks if the `msg` JSON value is a map as passed it to the next handler below.
    ///
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Object(map) => Msg::try_from(map.to_owned()),
            value => Err(Error::ArgumentInvalidFormat(value.to_owned())),
        }
    }
}

impl TryFrom<serde_json::Map<String, serde_json::Value>> for Msg {
    type Error = Error;

    ///
    /// Parses the transaction from the inner JSON map.
    ///
    fn try_from(
        mut value: serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, Self::Error> {
        let sender = value
            .remove(Self::FIELD_NAME_SENDER)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_SENDER))?;
        let sender = sender
            .as_str()
            .ok_or(Error::FieldNotAString(Self::FIELD_NAME_SENDER))?;
        let sender: Address = sender[2..]
            .parse()
            .map_err(|error| Error::FieldParsingHex(Self::FIELD_NAME_SENDER, error))?;

        let recipient = value
            .remove(Self::FIELD_NAME_RECIPIENT)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_RECIPIENT))?;
        let recipient = recipient
            .as_str()
            .ok_or(Error::FieldNotAString(Self::FIELD_NAME_RECIPIENT))?;
        let recipient: Address = recipient[2..]
            .parse()
            .map_err(|error| Error::FieldParsingHex(Self::FIELD_NAME_SENDER, error))?;

        let token_address = value
            .remove(Self::FIELD_NAME_TOKEN_ADDRESS)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_TOKEN_ADDRESS))?;
        let token_address = token_address
            .as_str()
            .ok_or(Error::FieldNotAString(Self::FIELD_NAME_TOKEN_ADDRESS))?;
        let token_address: Address = token_address[2..]
            .parse()
            .map_err(|error| Error::FieldParsingHex(Self::FIELD_NAME_SENDER, error))?;

        let amount = value
            .remove(Self::FIELD_NAME_AMOUNT)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_AMOUNT))?;
        let amount = amount
            .as_str()
            .ok_or(Error::FieldNotAString(Self::FIELD_NAME_AMOUNT))?;
        let amount: num_old::BigUint = zinc_math::bigint_from_str(amount)
            .map_err(|error| Error::FieldParsingLongInteger(Self::FIELD_NAME_SENDER, error))?
            .to_biguint()
            .map(crate::utils::num_compat_backward)
            .expect(zinc_const::panic::DATA_CONVERSION);

        Ok(Self {
            sender,
            recipient,
            token_address,
            amount,
        })
    }
}
