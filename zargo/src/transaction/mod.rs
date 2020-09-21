//!
//! The transaction which is signed as sent through Zandbox directly to zkSync.
//!

pub mod error;

use std::convert::TryFrom;

use num_old::BigUint;
use serde_derive::Deserialize;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use zksync::web3::types::Address;
use zksync::web3::types::H256;
use zksync::zksync_models::node::tx::PackedEthSignature;
use zksync::zksync_models::node::tx::Transfer;
use zksync::zksync_models::node::TokenLike;

use self::error::Error;

///
/// The transaction which is signed as sent through Zandbox directly to zkSync.
///
#[derive(Debug, Deserialize)]
pub struct Transaction {
    /// The sender address.
    pub from: Address,
    /// The recipient address.
    pub to: Address,
    /// The token ID to send.
    pub token_id: TokenLike,
    /// The amount to send.
    pub amount: BigUint,
}

impl Transaction {
    pub fn try_into_transfer(
        self,
        network: zksync::Network,
        signer_private_key: String,
    ) -> Result<(Transfer, PackedEthSignature), Error> {
        let mut runtime = tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

        let signer_private_key: H256 = signer_private_key
            .parse()
            .map_err(Error::SenderPrivateKeyInvalid)?;
        let signer_address = PackedEthSignature::address_from_private_key(&signer_private_key)
            .map_err(Error::SenderAddressDeriving)?;

        if signer_address != self.from {
            return Err(Error::SenderAddressPrivateKeyMismatch);
        }

        let wallet_credentials =
            zksync::WalletCredentials::from_eth_pk(signer_address, signer_private_key)
                .expect(zinc_const::panic::DATA_CONVERSION);
        let wallet = runtime
            .block_on(zksync::Wallet::new(
                zksync::Provider::new(network),
                wallet_credentials,
            ))
            .map_err(Error::WalletInitialization)?;
        let nonce = runtime
            .block_on(wallet.provider.account_info(signer_address))
            .map_err(Error::AccountInfoRetrieving)?
            .committed
            .nonce;
        let token = wallet
            .tokens
            .resolve(self.token_id)
            .ok_or(Error::TokenIdInvalid)?;
        let (transfer, signature) = wallet
            .signer
            .sign_transfer(
                token,
                self.amount,
                BigUint::from(1_000_000_000_000_000u64),
                self.to,
                nonce,
            )
            .map_err(Error::TransactionSigning)?;
        let signature = signature.expect(zinc_const::panic::DATA_CONVERSION);

        Ok((transfer, signature))
    }
}

impl TryFrom<JsonMap<String, JsonValue>> for Transaction {
    type Error = Error;

    fn try_from(mut value: JsonMap<String, JsonValue>) -> Result<Self, Self::Error> {
        const FIELD_NAME_FROM: &str = "from";
        const FIELD_NAME_TO: &str = "to";
        const FIELD_NAME_TOKEN_ID: &str = "token_id";
        const FIELD_NAME_AMOUNT: &str = "amount";

        let from = value
            .remove(FIELD_NAME_FROM)
            .ok_or(Error::FieldMissing(FIELD_NAME_FROM))?;
        let from = from.as_str().ok_or(Error::NotAString(FIELD_NAME_FROM))?;
        let from: Address = from[2..].parse().map_err(Error::SenderAddressInvalid)?;

        let to = value
            .remove(FIELD_NAME_TO)
            .ok_or(Error::FieldMissing(FIELD_NAME_TO))?;
        let to = to.as_str().ok_or(Error::NotAString(FIELD_NAME_TO))?;
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
            from,
            to,
            token_id,
            amount,
        })
    }
}
