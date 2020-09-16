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
use hex::FromHex;
use wallet_gen::coin::Coin;
use wallet_gen::wallet::Wallet;

use zinc_build::Program as BuildProgram;
use zinc_build::Type as BuildType;
use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use crate::database::model::contract::insert::input::Input as ContractInsertInput;
use crate::database::model::field::insert::input::Input as FieldInsertInput;
use crate::database::model::method::insert::input::Input as MethodInsertInput;
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

    let methods: Vec<MethodInsertInput> = build
        .methods
        .iter()
        .map(|(name, method)| {
            MethodInsertInput::new(
                query.contract_id,
                name.to_owned(),
                false,
                serde_json::to_value(&method.input).expect(zinc_const::panic::DATA_VALID),
                serde_json::to_value(&method.output).expect(zinc_const::panic::DATA_VALID),
            )
        })
        .collect();

    let input_value = BuildValue::try_from_typed_json(body.arguments, constructor.input)
        .map_err(Error::InvalidInput)?;

    let storage = build.storage.clone();
    let storage_value = BuildValue::new(BuildType::Contract(build.storage.clone()));
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

    let mut fields = Vec::with_capacity(storage.len());
    match output.result {
        BuildValue::Structure(mut storage_fields) => match storage_fields.remove(0).1 {
            BuildValue::Contract(storage_fields) => {
                for (index, (name, value)) in storage_fields.into_iter().enumerate() {
                    let value = value.into_json();
                    fields.push(FieldInsertInput::new(
                        query.contract_id,
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

    let wallet = Wallet::generate(Coin::Ethereum).expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
    let eth_address = <[u8; zinc_const::size::ETH_ADDRESS]>::from_hex(&wallet.address[2..])
        .expect(zinc_const::panic::DATA_VALID);
    let public_key = <[u8; zinc_const::size::ETH_PUBLIC_KEY]>::from_hex(wallet.public_key.as_str())
        .expect(zinc_const::panic::DATA_VALID);
    let private_key =
        <[u8; zinc_const::size::ETH_PRIVATE_KEY]>::from_hex(wallet.private_key.as_str())
            .expect(zinc_const::panic::DATA_VALID);

    app_data
        .write()
        .expect(zinc_const::panic::MULTI_THREADING)
        .contracts
        .insert(
            query.contract_id,
            SharedDataContract::new(build, eth_address, private_key),
        );

    app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .insert_contract(ContractInsertInput::new(
            query.contract_id,
            query.name,
            query.version,
            env!("CARGO_PKG_VERSION").to_owned(),
            serde_json::to_value(body.source).expect(zinc_const::panic::DATA_VALID),
            body.bytecode,
            body.verifying_key,
            eth_address,
            public_key,
            private_key,
        ))
        .await
        .map_err(Error::Database)?;

    app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .insert_methods(methods)
        .await
        .map_err(Error::Database)?;

    app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .insert_fields(fields)
        .await
        .map_err(Error::Database)?;

    let response = ResponseBody::new(wallet.address);

    Response::new_with_data(StatusCode::CREATED, response).into()
}
