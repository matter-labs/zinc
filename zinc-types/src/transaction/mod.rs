//!
//! The transaction data, which is sent between Zandbox, Zargo, and front-ends.
//!

pub mod error;
pub mod msg;

use serde::Deserialize;
use serde::Serialize;

use zksync_types::tx::PackedEthSignature;
use zksync_types::tx::ZkSyncTx;
use zksync_types::TokenLike;

use self::error::Error;
use self::msg::Msg;

///
/// The transaction, understandable by zkSync, front-end, Zandbox, and Zargo.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The transaction itself.
    pub tx: ZkSyncTx,
    /// The Ethereum signature of the transaction.
    pub ethereum_signature: Option<EthereumSignature>,
}

impl Transaction {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(tx: ZkSyncTx, signature: Option<PackedEthSignature>) -> Self {
        Self {
            tx,
            ethereum_signature: signature.map(EthereumSignature::new),
        }
    }

    ///
    /// Converts the transaction into an intrinsic `zksync::msg` variable representation.
    ///
    pub fn try_to_msg(
        &self,
        wallet: &zksync::Wallet<zksync_eth_signer::PrivateKeySigner, zksync::RpcProvider>,
    ) -> Result<Msg, Error> {
        match self.tx {
            ZkSyncTx::Transfer(ref transfer) => {
                let token = wallet
                    .tokens
                    .resolve(TokenLike::Id(transfer.token))
                    .ok_or(Error::UnsupportedToken(transfer.token))?;

                Ok(Msg::new(
                    transfer.from,
                    transfer.to,
                    token.address,
                    zksync::utils::closest_packable_token_amount(&transfer.amount),
                ))
            }
            ZkSyncTx::Withdraw(..) => Err(Error::UnsupportedTransaction("Withdraw")),
            ZkSyncTx::Close(..) => Err(Error::UnsupportedTransaction("Close")),
            ZkSyncTx::ChangePubKey(..) => Err(Error::UnsupportedTransaction("ChangePubKey")),
            ZkSyncTx::ForcedExit(..) => Err(Error::UnsupportedTransaction("ForcedExit")),
        }
    }
}

///
/// The transaction Ethereum signature.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthereumSignature {
    /// The default signature type.
    pub r#type: String,
    /// The signature as a hex string.
    pub signature: PackedEthSignature,
}

impl EthereumSignature {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(signature: PackedEthSignature) -> Self {
        Self {
            r#type: "EthereumSignature".to_owned(),
            signature,
        }
    }
}
