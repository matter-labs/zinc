//!
//! The transaction data, which is sent between Zandbox, Zargo, and front-ends.
//!

pub mod error;
pub mod msg;

use std::convert::TryInto;

use serde::Deserialize;
use serde::Serialize;

use zksync_types::tx::PackedEthSignature;
use zksync_types::tx::ZkSyncTx;

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
    pub ethereum_signature: EthereumSignature,
}

impl Transaction {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(tx: ZkSyncTx, signature: PackedEthSignature) -> Self {
        Self {
            tx,
            ethereum_signature: EthereumSignature::new(signature),
        }
    }
}

impl TryInto<Msg> for &Transaction {
    type Error = Error;

    fn try_into(self) -> Result<Msg, Self::Error> {
        match self.tx {
            ZkSyncTx::Transfer(ref transfer) => Ok(Msg::new(
                transfer.from,
                transfer.to,
                transfer.token,
                zksync::utils::closest_packable_token_amount(&transfer.amount),
            )),
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
