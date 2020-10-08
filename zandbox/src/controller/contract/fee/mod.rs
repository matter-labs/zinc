//!
//! The contract resource PUT method `fee` module.
//!

pub mod error;
pub mod request;
pub mod response;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use num_old::BigUint;
use num_old::Zero;

use zksync_types::tx::ZkSyncTx;
use zksync_types::TxFeeTypes;

use zinc_build::Value as BuildValue;
use zinc_data::Transfer;
use zinc_vm::Bn256;

use crate::database::model::field::select::Input as FieldSelectInput;
use crate::response::Response;
use crate::shared_data::SharedData;
use crate::storage::Storage;

use self::error::Error;
use self::request::Body as RequestBody;
use self::request::Query as RequestQuery;
use self::response::Body as ResponseBody;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract from the in-memory cache.
/// 2. Extract the called method from its metadata and check if it is mutable.
/// 3. Check if the transactions in the contract method arguments match the signed ones.
/// 4. Parse the method input arguments.
/// 5. Get the contract storage from data sources and convert it to the Zinc VM representation.
/// 6. Run the method on the Zinc VM.
/// 7. Extract the transfer data from the Zinc VM.
/// 8. Calculate the fee required for the transfers.
/// 9. Send the calculated fee back to the client.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
    body: web::Json<RequestBody>,
) -> crate::Result<ResponseBody, Error> {
    let query = query.into_inner();
    let body = body.into_inner();

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();

    log::debug!(
        "Calculating the fee for method `{}` of contract {}",
        query.method,
        serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
    );

    let contract = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .contracts
        .get(&query.address)
        .cloned()
        .ok_or_else(|| {
            Error::ContractNotFound(
                serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
            )
        })?;
    let account_id = contract.account_id.ok_or_else(|| {
        Error::ContractLocked(
            serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION),
        )
    })?;

    let method = match contract.build.methods.get(query.method.as_str()).cloned() {
        Some(method) => method,
        None => return Err(Error::MethodNotFound(query.method)),
    };
    if !method.is_mutable {
        return Err(Error::MethodIsImmutable(query.method));
    }

    log::debug!("Initializing the contract wallet");
    let provider = zksync::Provider::new(query.network);
    let wallet_credentials = zksync::WalletCredentials::from_eth_pk(
        query.address,
        contract.eth_private_key,
        query.network,
    )?;
    let wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

    let argument_transfer = Transfer::try_from_json(&body.arguments)?;
    argument_transfer.validate(&wallet, &body.transaction)?;

    let input_value = BuildValue::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;

    log::debug!("Loading the pre-transaction contract storage");
    let database_fields = postgresql
        .select_fields(FieldSelectInput::new(account_id))
        .await?;
    let storage = Storage::new_with_data(
        database_fields,
        contract.build.storage.as_slice(),
        contract.eth_address,
        &wallet,
    )
    .await?;

    log::debug!("Running the contract method on the virtual machine");
    let method = query.method;
    let contract_build = contract.build;
    let vm_time = std::time::Instant::now();
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(contract_build).run::<Bn256>(
            input_value,
            storage.into_build(),
            method,
        )
    })
    .await
    .map_err(Error::RuntimeError)?;
    log::debug!("VM executed in {} ms", vm_time.elapsed().as_millis());

    log::debug!("Calculating the fee for the method transfers");
    let mut fee = BigUint::zero();
    let token = match body.transaction.tx {
        ZkSyncTx::Transfer(ref transfer) => wallet
            .tokens
            .resolve(transfer.token.into())
            .ok_or(Error::TokenNotFound(transfer.token))?,
        _ => panic!(zinc_const::panic::VALUE_ALWAYS_EXISTS),
    };
    for transfer in output.transfers.into_iter() {
        fee += wallet
            .provider
            .get_tx_fee(TxFeeTypes::Transfer, transfer.recipient.into(), token.id)
            .await?
            .total_fee;
    }
    log::debug!(
        "The contract transfers total fee is {} {}",
        zksync_utils::format_units(&fee, token.decimals),
        token.symbol,
    );

    let response = ResponseBody::new(fee);

    log::debug!("The fee has been successfully calculated");
    Ok(Response::new_with_data(StatusCode::OK, response))
}
