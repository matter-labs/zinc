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

use zksync::web3::types::H160;
use zksync::web3::types::H256;
use zksync::web3::types::U256;
use zksync::zksync_models::node::tx::PackedEthSignature;

use crate::database::model::contract::insert::input::Input as ContractInsertInput;
use crate::database::model::field::insert::input::Input as FieldInsertInput;
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
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<RequestQuery>,
    body: web::Json<RequestBody>,
) -> crate::Result<ResponseBody, Error> {
    let query = query.into_inner();
    let body = body.into_inner();

    log::debug!(
        "Publishing an instance of the contract `{} {}`",
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

    let network = match query.network {
        zinc_data::Network::Localhost => zksync::Network::Localhost,
        zinc_data::Network::Rinkeby => zksync::Network::Rinkeby,
        zinc_data::Network::Ropsten => zksync::Network::Ropsten,
    };
    let provider = zksync::Provider::new(network);

    log::debug!("Generating ETH private key");
    let mut contract_private_key = H256::default();
    contract_private_key.randomize();
    let contract_eth_address: H160 =
        PackedEthSignature::address_from_private_key(&contract_private_key).unwrap();
    log::debug!(
        "The contract ETH address is {}",
        contract_eth_address.to_string()
    );

    let source_address: H160 = body.transfer.source_address[2..]
        .parse()
        .map_err(Error::InvalidSourceAddress)?;
    let source_private_key: H256 = body.transfer.source_private_key[2..]
        .parse()
        .map_err(Error::InvalidSourcePrivateKey)?;

    let owner_wallet_credentials =
        zksync::WalletCredentials::from_eth_pk(source_address, source_private_key)
            .map_err(Error::ZkSync)?;
    let owner_wallet = zksync::Wallet::new(provider.clone(), owner_wallet_credentials)
        .await
        .map_err(Error::ZkSync)?;

    log::debug!("Making the initial deposit");
    let ethereum = owner_wallet
        .ethereum("http://localhost:8545")
        .await
        .map_err(Error::ZkSync)?;
    let amount = U256::from(body.transfer.amount.to_bytes_be().as_slice())
        .pow(zinc_const::zandbox::ETH_BALANCE_EXPONENT.into());
    let eth_deposit_tx_hash = ethereum
        .deposit("ETH", amount, contract_eth_address)
        .await
        .map_err(Error::ZkSync)?;
    crate::wait::eth_tx(&ethereum, eth_deposit_tx_hash).await;

    log::debug!("Performing the change-pubkey transaction");
    let contract_wallet_credentials =
        zksync::WalletCredentials::from_eth_pk(contract_eth_address, contract_private_key)
            .map_err(Error::ZkSync)?;
    let mut contract_wallet = zksync::Wallet::new(provider, contract_wallet_credentials)
        .await
        .map_err(Error::ZkSync)?;
    let contract_account_id = crate::wait::account_id(&mut contract_wallet)
        .await
        .map_err(Error::ZkSync)?;
    contract_wallet
        .start_change_pubkey()
        .send()
        .await
        .map_err(Error::ZkSync)?;

    let mut fields = Vec::with_capacity(storage.len());
    match output.result {
        BuildValue::Structure(mut storage_fields) => match storage_fields.remove(0).1 {
            BuildValue::Contract(storage_fields) => {
                for (index, (name, value)) in storage_fields.into_iter().enumerate() {
                    let value = value.into_json();
                    fields.push(FieldInsertInput::new(
                        contract_account_id as i64,
                        index as i16,
                        name,
                        value,
                    ));
                }
            }
            _ => return Err(Error::InvalidStorage),
        },
        _ => return Err(Error::InvalidStorage),
    }

    log::debug!("Writing the contract to the server cache");
    app_data
        .write()
        .expect(zinc_const::panic::MULTI_THREADING)
        .contracts
        .insert(
            contract_account_id,
            SharedDataContract::new(build, contract_eth_address, contract_private_key),
        );

    log::debug!("Writing the contract to the persistent PostgreSQL database");
    app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .insert_contract(ContractInsertInput::new(
            contract_account_id as i64,
            query.name,
            query.version,
            env!("CARGO_PKG_VERSION").to_owned(),
            serde_json::to_value(body.source).expect(zinc_const::panic::DATA_VALID),
            body.bytecode,
            body.verifying_key,
            contract_eth_address.into(),
            contract_private_key.into(),
        ))
        .await
        .map_err(Error::Database)?;

    log::debug!("Writing the contract storage to the persistent PostgreSQL database");
    app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .insert_fields(fields)
        .await
        .map_err(Error::Database)?;

    let response = ResponseBody::new(contract_account_id, contract_eth_address);

    log::debug!("The sequence has been successfully executed");
    Ok(Response::new_with_data(StatusCode::CREATED, response))
}
