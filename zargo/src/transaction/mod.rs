//!
//! The transaction tools.
//!

pub mod error;

use num_old::Zero;

use zksync::web3::types::Address;
use zksync::zksync_models::FranklinTx;
use zksync::zksync_models::TokenLike;
use zksync::zksync_models::TxFeeTypes;

use zinc_data::Transfer;
use zinc_data::Transaction;

use self::error::Error;

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
    transfers: Vec<Transfer>,
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
