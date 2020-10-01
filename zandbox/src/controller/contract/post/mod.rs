//!
//! The contract resource POST method module.
//!

pub mod error;
pub mod request;
pub mod response;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;

use zinc_build::Program as BuildProgram;
use zinc_build::Type as BuildType;
use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use zksync::web3::types::Address;
use zksync::web3::types::H256;
use zksync::zksync_models::tx::PackedEthSignature;

use crate::response::Response;
use crate::shared_data::contract::Contract as SharedDataContract;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body as RequestBody;
use self::request::Query as RequestQuery;
use self::response::Body as ResponseBody;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Parse the contract bytecode from the request.
/// 2. Extract the contract constructor from its metadata.
/// 3. Parse the construtor arguments.
/// 4. Run the construtor on the Zinc VM which must return the contract storage.
/// 5. Generate a private key for the contract.
/// 6. Fill the external contract storage fields.
/// 7. Write the contract and its storage to the in-memory cache.
/// 8. Return the created contract address to the client.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
    body: web::Json<RequestBody>,
) -> crate::Result<ResponseBody, Error> {
    let query = query.into_inner();
    let body = body.into_inner();

    log::debug!(
        "Publishing the instance `{}` of the contract `{} {}`",
        query.instance,
        query.name,
        query.version
    );

    let program =
        BuildProgram::try_from_slice(body.bytecode.as_slice()).map_err(Error::InvalidBytecode)?;

    let build = match program.clone() {
        BuildProgram::Circuit(_circuit) => return Err(Error::NotAContract),
        BuildProgram::Contract(contract) => contract,
    };

    let constructor = build
        .methods
        .get(zinc_const::zandbox::CONTRACT_CONSTRUCTOR_NAME)
        .cloned()
        .ok_or(Error::ConstructorNotFound)?;

    let input_value = BuildValue::try_from_typed_json(body.arguments, constructor.input)
        .map_err(Error::InvalidInput)?;

    log::debug!("Initializing the contract storage");
    let storage = build.storage.clone();
    let storage_value = BuildValue::new(BuildType::Contract(build.storage.clone()));

    log::debug!("Running the contract constructor on the virtual machine");
    let build_to_run = build.clone();
    let output = async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(build_to_run).run::<Bn256>(
            input_value,
            storage_value,
            zinc_const::zandbox::CONTRACT_CONSTRUCTOR_NAME.to_owned(),
        )
    })
    .await
    .map_err(Error::RuntimeError)?;

    log::debug!("Generating an ETH private key");
    let mut contract_private_key = H256::default();
    contract_private_key.randomize();
    let contract_address: Address =
        PackedEthSignature::address_from_private_key(&contract_private_key)
            .expect(zinc_const::panic::DATA_CONVERSION);
    log::debug!(
        "The contract ETH address is {}",
        serde_json::to_string(&contract_address).expect(zinc_const::panic::DATA_CONVERSION),
    );

    let mut fields = Vec::with_capacity(storage.len());
    match output.result {
        BuildValue::Contract(storage_fields) => {
            for mut field in storage_fields.into_iter() {
                match field.name.as_str() {
                    "address" if field.is_external => {
                        field.value = BuildValue::scalar_from_eth_address(contract_address)
                    }
                    _ => {}
                }

                fields.push((field.name, field.value));
            }
        }
        _ => panic!(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION),
    }

    log::debug!("Writing the contract to the temporary server cache");
    app_data
        .write()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .contracts
        .insert(
            contract_address,
            SharedDataContract::new(
                contract_address,

                query.name,
                query.version,
                query.instance,

                serde_json::to_value(body.source).expect(zinc_const::panic::DATA_CONVERSION),
                body.bytecode,
                body.verifying_key,

                None,
                contract_private_key,

                build,
                fields,
            ),
        );

    let response = ResponseBody::new(contract_address);

    log::debug!("The contract is waiting for the initialization");
    Ok(Response::new_with_data(StatusCode::CREATED, response))
}
