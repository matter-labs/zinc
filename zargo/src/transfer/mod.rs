//!
//! The transfer which is signed as sent through Zandbox directly to zkSync.
//!

pub mod error;

use std::convert::TryFrom;

use num_old::BigUint;
use serde_derive::Deserialize;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use zksync::web3::types::Address;
use zksync::web3::types::H256;
use zksync::zksync_models::node::tx::FranklinTx;
use zksync::zksync_models::node::tx::PackedEthSignature;
use zksync::zksync_models::node::TokenLike;
use zksync::zksync_models::node::TxFeeTypes;

use zinc_data::CallRequestBodyTransaction;

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
    pub amount: BigUint,
}

impl Transfer {
    pub fn try_into_batch(
        transfers: Vec<Self>,
        network: zksync::Network,
        signer_private_key: String,
    ) -> Result<Vec<CallRequestBodyTransaction>, Error> {
        let mut runtime = tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

        let signer_private_key: H256 = signer_private_key
            .parse()
            .map_err(Error::SenderPrivateKeyInvalid)?;
        let signer_address = PackedEthSignature::address_from_private_key(&signer_private_key)
            .map_err(Error::SenderAddressDeriving)?;

        let wallet_credentials =
            zksync::WalletCredentials::from_eth_pk(signer_address, signer_private_key, network)
                .expect(zinc_const::panic::DATA_CONVERSION);
        let wallet = runtime
            .block_on(zksync::Wallet::new(
                zksync::Provider::new(network),
                wallet_credentials,
            ))
            .map_err(Error::WalletInitialization)?;

        let mut batch = Vec::with_capacity(transfers.len());
        let mut nonce = runtime
            .block_on(wallet.provider.account_info(signer_address))
            .map_err(Error::AccountInfoRetrieving)?
            .committed
            .nonce;
        for transfer in transfers.into_iter() {
            let token = wallet
                .tokens
                .resolve(transfer.token_id.clone())
                .ok_or(Error::TokenNotFound)?;
            let fee = runtime
                .block_on(wallet.provider.get_tx_fee(
                    TxFeeTypes::Transfer,
                    signer_address,
                    transfer.token_id,
                ))
                .map_err(Error::FeeGetting)?
                .total_fee;

            let (transfer, signature) = wallet
                .signer
                .sign_transfer(token, transfer.amount, fee, transfer.recipient, nonce)
                .map_err(Error::TransferSigning)?;
            let signature = signature.expect(zinc_const::panic::DATA_CONVERSION);

            batch.push(CallRequestBodyTransaction::new(
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
        let amount: BigUint = amount.parse().map_err(Error::AmountInvalid)?;

        Ok(Self {
            sender: from,
            recipient: to,
            token_id,
            amount,
        })
    }
}
