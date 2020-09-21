//!
//! The method input arguments arguments file.
//!

use serde_derive::Deserialize;

use zksync::zksync_models::node::tx::Transfer;

///
/// The transaction which is signed as sent directly to zkSync.
///
#[derive(Debug, Deserialize)]
pub struct Transaction {
    /// The sender address.
    pub from: String,
    /// The recipient address.
    pub to: String,
    /// The token ID to send.
    pub token_id: String,
    /// The amount to send.
    pub amount: String,
}

impl Transaction {
    pub fn try_into_transfer(self, signer_private_key: String) -> Result<Transfer, ()> {
        let signer_private_key: H256 = signer_private_key.parse().unwrap();
        let signer_address = PackedEthSignature::address_from_private_key(&signer_private_key).unwrap();
        let transaction_from: H160 = transaction.from.parse().unwrap();
        assert!(signer_address == transaction_from, "Private key does not match the transaction sender address");
        let transaction_to: H160 = transaction.to.parse().unwrap();
        let transaction_amount: BigUint = transaction.amount.parse().unwrap();

        let wallet_credentials = zksync::WalletCredentials::from_eth_pk(signer_address, signer_private_key).unwrap();
        let wallet = zksync::Wallet::new(zksync::Provider::new(network), wallet_credentials).await.unwrap();
        let nonce = wallet.provider.account_info(signer_address).await.unwrap().committed.nonce;
        let token_like = transaction.token_id.into();
        let token = wallet
            .tokens
            .resolve(token_like).unwrap();
        let (transfer, signature) = wallet.signer.sign_transfer(
            token,
            transaction_amount,
            BigUint::zero(),
            transaction_to,
            nonce,
        ).unwrap();
    }
}
