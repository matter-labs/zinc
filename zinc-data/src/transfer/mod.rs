//!
//! The transfer argument which is sent to the smart contract method.
//!

pub mod error;

use std::convert::TryFrom;

use serde_derive::Deserialize;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use zksync::web3::types::Address;
use zksync::zksync_models::FranklinTx;
use zksync::zksync_models::TokenLike;

use crate::transaction::Transaction;

use self::error::Error;

///
/// The transfer argument which is sent to the smart contract method.
///
#[derive(Debug, Deserialize)]
pub struct Transfer {
    /// The sender address.
    pub sender: Address,
    /// The recipient address.
    pub recipient: Address,
    /// The token ID to send.
    pub token_id: TokenLike,
    /// The amount to send.
    pub amount: num_old::BigUint,
}

impl Transfer {
    /// The required transaction argument name in the contract arguments.
    const ARGUMENT_NAME: &'static str = "tx";

    /// The required sender address field name in the transaction structure.
    const FIELD_NAME_SENDER: &'static str = "sender";

    /// The required recipient address field name in the transaction structure.
    const FIELD_NAME_RECIPIENT: &'static str = "recipient";

    /// The required toked ID field name in the transaction structure.
    const FIELD_NAME_TOKEN_ID: &'static str = "token_id";

    /// The required amount field name in the transaction structure.
    const FIELD_NAME_AMOUNT: &'static str = "amount";

    ///
    /// Validates the transfer against the signed transaction created from it.
    ///
    pub fn validate(
        &self,
        wallet: &zksync::Wallet,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        if let FranklinTx::Transfer(ref transfer) = transaction.tx {
            if self.sender != transfer.from {
                return Err(Error::Validation(Self::FIELD_NAME_SENDER));
            }

            if self.recipient != transfer.to {
                return Err(Error::Validation(Self::FIELD_NAME_RECIPIENT));
            }

            let token = wallet
                .tokens
                .resolve(self.token_id.to_owned())
                .ok_or(Error::TokenResolving(self.token_id.to_owned()))?;
            if token.id != transfer.token {
                return Err(Error::Validation(Self::FIELD_NAME_TOKEN_ID));
            }

            if self.amount != transfer.amount {
                return Err(Error::Validation(Self::FIELD_NAME_AMOUNT));
            }
        }

        Ok(())
    }

    ///
    /// Gets the transaction argument from the JSON.
    ///
    /// Should only be called for mutable methods (`call` command) where the transaction is mandatory.
    ///
    pub fn try_from_json(value: &JsonValue) -> Result<Self, Error> {
        match value {
            JsonValue::Object(map) => match map
                .get(Self::ARGUMENT_NAME)
                .cloned()
                .ok_or(Error::ArgumentMissing(Self::ARGUMENT_NAME))?
            {
                JsonValue::Object(map) => Transfer::try_from(map),
                _ => Err(Error::ArgumentInvalidFormat(Self::ARGUMENT_NAME)),
            },
            _ => Err(Error::ArgumentInvalidFormat(Self::ARGUMENT_NAME)),
        }
    }
}

impl TryFrom<JsonMap<String, JsonValue>> for Transfer {
    type Error = Error;

    ///
    /// Gets a single transaction argument from the inner JSON map.
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

        let token_id = value
            .remove(Self::FIELD_NAME_TOKEN_ID)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_TOKEN_ID))?;
        let token_id = token_id
            .as_str()
            .ok_or(Error::NotAString(Self::FIELD_NAME_TOKEN_ID))?;
        let token_id = token_id.into();

        let amount = value
            .remove(Self::FIELD_NAME_AMOUNT)
            .ok_or(Error::FieldMissing(Self::FIELD_NAME_AMOUNT))?;
        let amount = amount
            .as_str()
            .ok_or(Error::NotAString(Self::FIELD_NAME_AMOUNT))?;
        let amount: num_old::BigUint = zinc_utils::bigint_from_str(amount)
            .map_err(|error| Error::AmountInvalid(error.to_string()))?
            .to_biguint()
            .map(|value| num_old::BigUint::from_bytes_be(value.to_bytes_be().as_slice())) // TODO: remove when the SDK is updated
            .expect(zinc_const::panic::DATA_CONVERSION);

        Ok(Self {
            sender: from,
            recipient: to,
            token_id,
            amount,
        })
    }
}
