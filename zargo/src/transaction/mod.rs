//!
//! The transaction tools.
//!

pub mod error;

use num::BigUint;

use zksync::provider::Provider;
use zksync::web3::types::Address;
use zksync_types::tx::ZkSyncTx;
use zksync_types::TokenLike;
use zksync_types::TxFeeTypes;

use zinc_types::TransactionMsg;

use self::error::Error;

///
/// Initializes a new initial zero transfer to assign an account ID to a newly created contract.
///
pub async fn new_initial(
    wallet: &zksync::Wallet<zksync_eth_signer::PrivateKeySigner, zksync::RpcProvider>,
    recipient: Address,
    token_symbol: String,
    amount: BigUint,
) -> anyhow::Result<zinc_types::Transaction> {
    let token_like = TokenLike::Symbol(token_symbol);
    let token = wallet
        .tokens
        .resolve(token_like.clone())
        .ok_or(Error::TokenNotFound)?;

    let amount =
        zksync::utils::closest_packable_token_amount(&zinc_types::num_compat_backward(amount));
    let fee = wallet
        .provider
        .get_tx_fee(TxFeeTypes::Transfer, recipient, token_like)
        .await
        .map_err(Error::FeeGetting)?
        .total_fee;
    let nonce = wallet
        .provider
        .account_info(wallet.signer.address)
        .await
        .map_err(Error::AccountInfoRetrieving)?
        .committed
        .nonce;

    let (transfer, signature) = wallet
        .signer
        .sign_transfer(token, amount, fee, recipient, nonce)
        .await
        .map_err(Error::TransactionSigning)?;

    Ok(zinc_types::Transaction::new(
        ZkSyncTx::Transfer(Box::new(transfer)),
        signature,
    ))
}

///
/// Converts an array of input transfers into an array of signed zkSync transactions.
///
pub async fn try_into_zksync(
    transaction: TransactionMsg,
    wallet: &zksync::Wallet<zksync_eth_signer::PrivateKeySigner, zksync::RpcProvider>,
    contract_fee: Option<BigUint>,
) -> anyhow::Result<zinc_types::Transaction> {
    let token = wallet
        .tokens
        .resolve(transaction.token_address.into())
        .ok_or(Error::TokenNotFound)?;
    let amount = zksync::utils::closest_packable_token_amount(&transaction.amount);
    let fee = wallet
        .provider
        .get_tx_fee(
            TxFeeTypes::Transfer,
            wallet.signer.address,
            transaction.token_address,
        )
        .await
        .map_err(Error::FeeGetting)?
        .total_fee
        + contract_fee
            .map(zinc_types::num_compat_backward)
            .unwrap_or_default();
    let fee = zksync::utils::closest_packable_fee_amount(&fee);
    let nonce = wallet
        .provider
        .account_info(wallet.signer.address)
        .await
        .map_err(Error::AccountInfoRetrieving)?
        .committed
        .nonce;

    let (transfer, signature) = wallet
        .signer
        .sign_transfer(token, amount, fee, transaction.recipient, nonce)
        .await
        .map_err(Error::TransactionSigning)?;

    Ok(zinc_types::Transaction::new(
        ZkSyncTx::Transfer(Box::new(transfer)),
        signature,
    ))
}
