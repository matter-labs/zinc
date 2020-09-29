//!
//! The transaction data, which is sent between Zandbox, Zargo, and front-ends.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use zksync::zksync_models::tx::PackedEthSignature;
use zksync::zksync_models::FranklinTx;

///
/// The transaction, understandable by zkSync, front-end, Zandbox, and Zargo.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The transaction itself.
    pub tx: FranklinTx,
    /// The Ethereum signature of the transaction.
    pub ethereum_signature: EthereumSignature,
}

impl Transaction {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(tx: FranklinTx, signature: PackedEthSignature) -> Self {
        Self {
            tx,
            ethereum_signature: EthereumSignature::new(signature),
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
