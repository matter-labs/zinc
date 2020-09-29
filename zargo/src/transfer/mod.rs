//!
//! The transfer which is signed as sent through Zandbox directly to zkSync.
//!

pub mod error;

use std::convert::TryFrom;

use num_old::Zero;
use serde_derive::Deserialize;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use zksync::web3::types::Address;
use zksync::zksync_models::FranklinTx;
use zksync::zksync_models::TokenLike;
use zksync::zksync_models::TxFeeTypes;

use zinc_data::Transaction;

use self::error::Error;

///
/// The transfer which is signed as sent through Zandbox directly to zkSync.
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
    ///
    /// Initializes a new initial zero transfer to assign an account ID to a newly created contract.
    ///
    pub fn new_initial(wallet: &zksync::Wallet, recipient: Address) -> Result<Transaction, Error> {
        let mut runtime = tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

        let token_id = 0;
        let token_like = TokenLike::Id(token_id);
        let token = wallet
            .tokens
            .resolve(TokenLike::Id(token_id))
            .ok_or(Error::TokenNotFound)?;

        let amount = num_old::BigUint::zero();
        let fee = runtime
            .block_on(
                wallet
                    .provider
                    .get_tx_fee(TxFeeTypes::Transfer, recipient, token_like),
            )
            .map_err(Error::FeeGetting)?
            .total_fee;
        let nonce = runtime
            .block_on(wallet.provider.account_info(wallet.signer.address))
            .map_err(Error::AccountInfoRetrieving)?
            .committed
            .nonce;

        let (transfer, signature) = wallet
            .signer
            .sign_transfer(token, amount, fee, recipient, nonce)
            .map_err(Error::TransferSigning)?;
        let signature = signature.expect(zinc_const::panic::DATA_CONVERSION);

        Ok(Transaction::new(
            FranklinTx::Transfer(Box::new(transfer)),
            signature,
        ))
    }

    ///
    /// Converts an array of input transfers into an array of signed zkSync transactions.
    ///
    pub fn try_into_batch(
        transfers: Vec<Self>,
        wallet: &zksync::Wallet,
        fee_multiplier: u64,
    ) -> Result<Vec<Transaction>, Error> {
        let mut runtime = tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

        let mut batch = Vec::with_capacity(transfers.len());
        let mut nonce = runtime
            .block_on(wallet.provider.account_info(wallet.signer.address))
            .map_err(Error::AccountInfoRetrieving)?
            .committed
            .nonce;
        for transfer in transfers.into_iter() {
            let token = wallet
                .tokens
                .resolve(transfer.token_id.clone())
                .ok_or(Error::TokenNotFound)?;
            let amount = zksync::utils::closest_packable_token_amount(&transfer.amount);
            let fee = runtime
                .block_on(wallet.provider.get_tx_fee(
                    TxFeeTypes::Transfer,
                    wallet.signer.address,
                    transfer.token_id,
                ))
                .map_err(Error::FeeGetting)?
                .total_fee
                * num_old::BigUint::from(fee_multiplier);

            let (transfer, signature) = wallet
                .signer
                .sign_transfer(token, amount, fee, transfer.recipient, nonce)
                .map_err(Error::TransferSigning)?;
            let signature = signature.expect(zinc_const::panic::DATA_CONVERSION);

            batch.push(Transaction::new(
                FranklinTx::Transfer(Box::new(transfer)),
                signature,
            ));

            nonce += 1;
        }

        Ok(batch)
    }
}

impl TryFrom<JsonMap<String, JsonValue>> for Transfer {
    type Error = Error;

    fn try_from(mut value: JsonMap<String, JsonValue>) -> Result<Self, Self::Error> {
        const FIELD_NAME_SENDER: &str = "sender";
        const FIELD_NAME_RECIPIENT: &str = "recipient";
        const FIELD_NAME_TOKEN_ID: &str = "token_id";
        const FIELD_NAME_AMOUNT: &str = "amount";

        let from = value
            .remove(FIELD_NAME_SENDER)
            .ok_or(Error::FieldMissing(FIELD_NAME_SENDER))?;
        let from = from.as_str().ok_or(Error::NotAString(FIELD_NAME_SENDER))?;
        let from: Address = from[2..].parse().map_err(Error::SenderAddressInvalid)?;

        let to = value
            .remove(FIELD_NAME_RECIPIENT)
            .ok_or(Error::FieldMissing(FIELD_NAME_RECIPIENT))?;
        let to = to.as_str().ok_or(Error::NotAString(FIELD_NAME_RECIPIENT))?;
        let to: Address = to[2..].parse().map_err(Error::RecipientAddressInvalid)?;

        let token_id = value
            .remove(FIELD_NAME_TOKEN_ID)
            .ok_or(Error::FieldMissing(FIELD_NAME_TOKEN_ID))?;
        let token_id = token_id
            .as_str()
            .ok_or(Error::NotAString(FIELD_NAME_TOKEN_ID))?;
        let token_id = token_id.into();

        let amount = value
            .remove(FIELD_NAME_AMOUNT)
            .ok_or(Error::FieldMissing(FIELD_NAME_AMOUNT))?;
        let amount = amount
            .as_str()
            .ok_or(Error::NotAString(FIELD_NAME_AMOUNT))?;
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
