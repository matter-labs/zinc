//!
//! The Zandbox server daemon binary.
//!

mod arguments;
mod error;

use std::collections::HashMap;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use colored::Colorize;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use zksync_types::AccountId;
use zksync_types::Address;

use zinc_build::Program as BuildProgram;
use zinc_build::Value as BuildValue;

use zandbox::ContractSelectAllOutput;
use zandbox::DatabaseClient;
use zandbox::FieldSelectInput;
use zandbox::FieldSelectOutput;
use zandbox::SharedData;
use zandbox::SharedDataContract;

use self::arguments::Arguments;
use self::error::Error;

///
/// The application entry point.
///
#[actix_rt::main]
async fn main() -> Result<(), Error> {
    let args = Arguments::new();

    zinc_utils::initialize_logger(zinc_const::app_name::ZANDBOX, args.verbosity);

    log::info!("Zandbox server started");

    log::info!("Initializing the PostgreSQL client");
    let database_client = DatabaseClient::new(
        args.postgresql_host,
        args.postgresql_port.unwrap_or(zinc_const::postgresql::PORT),
        args.postgresql_user,
        args.postgresql_password,
        args.postgresql_database,
    )
    .await?;

    log::info!("Loading the compiled contracts from the database");
    let mut contracts: HashMap<Address, SharedDataContract> = database_client
        .select_contracts()
        .await?
        .into_par_iter()
        .map(
            |ContractSelectAllOutput {
                 address,

                 name,
                 version,
                 instance,

                 source_code,
                 bytecode,
                 verifying_key,

                 account_id,
                 eth_private_key,
             }| {
                let address = zinc_utils::eth_address_from_vec(address);
                let eth_private_key = zinc_utils::eth_private_key_from_vec(eth_private_key);

                log::info!(
                    "{} instance `{}` of the contract `{} v{}` with address {}",
                    "Loaded".bright_green(),
                    instance,
                    name,
                    version,
                    serde_json::to_string(&address).expect(zinc_const::panic::DATA_CONVERSION),
                );

                let program = BuildProgram::try_from_slice(bytecode.as_slice())
                    .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);

                let build = match program {
                    BuildProgram::Circuit(_circuit) => {
                        panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
                    }
                    BuildProgram::Contract(contract) => contract,
                };

                let contract = SharedDataContract::new(
                    address,
                    name,
                    version,
                    instance,
                    source_code,
                    bytecode,
                    verifying_key,
                    Some(account_id as AccountId),
                    eth_private_key,
                    build,
                    vec![],
                );

                (address, contract)
            },
        )
        .collect();

    log::info!("Loading the contract storages from the database");
    for (address, contract) in contracts.iter_mut() {
        let storage_value = database_client
            .select_fields(FieldSelectInput::new(*address))
            .await?;
        for (index, FieldSelectOutput { name, value }) in storage_value.into_iter().enumerate() {
            let r#type = contract.build.storage[index].r#type.clone();
            let value = BuildValue::try_from_typed_json(value, r#type)
                .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
            contract.fields.push((name, value));
        }
    }

    let data = SharedData::new(database_client, contracts).wrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::DefaultHeaders::new().content_type())
            .wrap(actix_cors::Cors::default())
            .app_data(web::JsonConfig::default().limit(zinc_const::limit::JSON_PAYLOAD))
            .data(data.clone())
            .configure(zandbox::configure)
    })
    .bind(format!(
        "{}:{}",
        zinc_const::zandbox::HOST,
        args.http_port.unwrap_or(zinc_const::zandbox::PORT)
    ))
    .map_err(Error::ServerBinding)?
    .run()
    .await
    .map_err(Error::ServerRuntime)?;

    log::info!("Zandbox server finished");
    Ok(())
}
