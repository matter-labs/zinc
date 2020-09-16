//!
//! The contract resource POST call method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use serde_json::Value as JsonValue;

use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
use crate::database::model::field::update::input::Input as FieldUpdateInput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body as RequestBody;
use self::request::Query as RequestQuery;

///
/// The HTTP request handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
    body: web::Json<RequestBody>,
) -> crate::Result<JsonValue, Error> {
    let query = query.into_inner();
    let body = body.into_inner();

    let contract = app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .contracts
        .get(&query.contract_id)
        .cloned()
        .ok_or(Error::ContractNotFound)?;

    let method = contract
        .build
        .methods
        .get(query.method.as_str())
        .cloned()
        .ok_or(Error::MethodNotFound)?;
    if !method.is_mutable {
        return Err(Error::MethodIsImmutable);
    }

    let input_value = BuildValue::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;

    let storage_value = app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .select_fields(FieldSelectInput::new(query.contract_id))
        .await
        .map_err(Error::Database)?;
    let storage_fields_count = storage_value.len();
    if storage_value.len() != contract.build.storage.len() {
        return Err(Error::InvalidStorageSize {
            expected: contract.build.storage.len(),
            found: storage_value.len(),
        });
    }
    let mut fields = Vec::with_capacity(storage_value.len());
    for (index, FieldSelectOutput { name, value }) in storage_value.into_iter().enumerate() {
        let r#type = contract.build.storage[index].1.clone();
        let value = match BuildValue::try_from_typed_json(value, r#type) {
            Ok(value) => value,
            Err(error) => return Err(Error::InvalidStorage(error)),
        };
        fields.push((name, value))
    }
    let storage_value = BuildValue::Contract(fields);

    let build = contract.build;
    let method = query.method;
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(build).run::<Bn256>(input_value, storage_value, method)
    })
    .await
    .map_err(Error::RuntimeError)?;

    let mut storage_fields = Vec::with_capacity(storage_fields_count);
    match output.storage {
        BuildValue::Contract(fields) => {
            for (index, (_name, value)) in fields.into_iter().enumerate() {
                let value = value.into_json();
                storage_fields.push(FieldUpdateInput::new(
                    index as i16,
                    query.contract_id,
                    value,
                ));
            }
        }
        _ => panic!(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION),
    }

    app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .update_fields(storage_fields)
        .await
        .map_err(Error::Database)?;

    let wallet_credentials = zksync::WalletCredentials::from_eth_pk(
        contract.eth_address.into(),
        contract.private_key.into(),
    )
    .map_err(Error::ZkSync)?;

    let network = match query.network {
        zinc_data::Network::Localhost => zksync::Network::Localhost,
        zinc_data::Network::Rinkeby => zksync::Network::Rinkeby,
        zinc_data::Network::Ropsten => zksync::Network::Ropsten,
    };
    let provider = zksync::Provider::new(network);

    let wallet = zksync::Wallet::new(provider, wallet_credentials)
        .await
        .map_err(Error::ZkSync)?;

    for transfer in output.transfers.into_iter() {
        wallet
            .start_transfer()
            .to(transfer.to.into())
            .token("ETH")
            .map_err(Error::ZkSync)?
            .amount(num_old::BigUint::from_bytes_be(
                transfer.amount.to_bytes_be().as_slice(), // TODO: remove when the SDK is updated
            ))
            .send()
            .await
            .map_err(Error::ZkSync)?
            .wait_for_commit()
            .await
            .map_err(Error::ZkSync)?;
    }

    let response = output.result.into_json();

    Response::new_with_data(StatusCode::OK, response).into()
}
