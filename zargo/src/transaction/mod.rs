//!
//! The transaction tools.
//!

pub mod error;

use num_old::Zero;

use zksync::web3::types::Address;
use zksync_types::FranklinTx;
use zksync_types::TokenLike;
use zksync_types::TxFeeTypes;

use zinc_data::Transaction;
use zinc_data::Transfer;

use self::error::Error;

///
/// Initializes a new initial zero transfer to assign an account ID to a newly created contract.
///
pub fn new_initial(wallet: &zksync::Wallet, recipient: Address) -> Result<Transaction, Error> {
    let mut runtime = tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

    let token_symbol = "ETH";
    let token_like = TokenLike::Symbol(token_symbol.to_owned());
    let token = wallet
        .tokens
        .resolve(token_like)
        .ok_or(Error::TokenNotFound)?;

    let amount = num_old::BigUint::zero();
    let fee = runtime
        .block_on(
            wallet
                .provider
                .get_tx_fee(TxFeeTypes::Transfer, recipient, token_symbol),
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
pub fn try_into_zksync(
    transfer: Transfer,
    wallet: &zksync::Wallet,
    fee_multiplier: u64,
) -> Result<Transaction, Error> {
    let mut runtime = tokio::runtime::Runtime::new().expect(zinc_const::panic::ASYNC_RUNTIME);

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
    let fee = zksync::utils::closest_packable_fee_amount(&fee);
    let nonce = runtime
        .block_on(wallet.provider.account_info(wallet.signer.address))
        .map_err(Error::AccountInfoRetrieving)?
        .committed
        .nonce;

    let (transfer, signature) = wallet
        .signer
        .sign_transfer(token, amount, fee, transfer.recipient, nonce)
        .map_err(Error::TransferSigning)?;
    let signature = signature.expect(zinc_const::panic::DATA_CONVERSION);

    Ok(Transaction::new(
        FranklinTx::Transfer(Box::new(transfer)),
        signature,
    ))
}
