//!
//! The cached contract data.
//!

use serde_json::Value as JsonValue;

use zksync::web3::types::H160;
use zksync::web3::types::H256;
use zksync_types::AccountId;

use zinc_build::Contract as BuildContract;
use zinc_build::Value as BuildValue;

///
/// The cached contract data.
///
#[derive(Debug, Clone)]
pub struct Contract {
    /// The contract ETH address.
    pub eth_address: H160,

    /// The contract name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract instance.
    pub instance: String,

    /// The contract source code.
    pub source_code: JsonValue,
    /// The contract bytecode.
    pub bytecode: Vec<u8>,
    /// The contract verifying key.
    pub verifying_key: Vec<u8>,

    /// The contract ETH private key.
    pub eth_private_key: H256,
    /// The contract zkSync account ID. Is set when the change-pubkey transaction is executed.
    pub account_id: Option<AccountId>,

    /// The pre-built contract ready to be called.
    pub build: BuildContract,
    /// The contract storage fields.
    pub fields: Vec<(String, BuildValue)>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        eth_address: H160,

        name: String,
        version: String,
        instance: String,

        source_code: JsonValue,
        bytecode: Vec<u8>,
        verifying_key: Vec<u8>,

        account_id: Option<AccountId>,
        eth_private_key: H256,

        build: BuildContract,
        fields: Vec<(String, BuildValue)>,
    ) -> Self {
        Self {
            eth_address,

            name,
            version,
            instance,

            source_code,
            bytecode,
            verifying_key,

            account_id,
            eth_private_key,

            build,
            fields,
        }
    }

    ///
    /// Sets the zkSync account ID.
    ///
    pub fn set_account_id(&mut self, account_id: AccountId) {
        self.account_id = Some(account_id);
    }
}
