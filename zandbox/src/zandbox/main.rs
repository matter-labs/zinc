//!
//! The Zandbox server daemon binary.
//!

mod arguments;
mod error;

use std::collections::HashMap;
use std::str::FromStr;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use colored::Colorize;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use zksync_eth_signer::PrivateKeySigner;
use zksync_types::AccountId;

use zinc_build::Application as BuildApplication;

use zandbox::ContractSelectAllOutput;
use zandbox::ContractStorage;
use zandbox::DatabaseClient;
use zandbox::FieldSelectInput;
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

    zinc_logger::initialize(zinc_const::app_name::ZANDBOX, args.verbosity);

    log::info!("Zandbox server started");

    let network =
        zksync::Network::from_str(args.network.as_str()).map_err(Error::InvalidNetwork)?;

    log::info!("Initializing the PostgreSQL client");
    let postgresql = DatabaseClient::new(args.postgresql_uri.as_str()).await?;

    log::info!("Loading the compiled contracts from the database");
    let database_data: Vec<ContractSelectAllOutput> = postgresql
        .select_contracts()
        .await?
        .into_par_iter()
        .collect();

    let mut contracts = HashMap::with_capacity(database_data.len());
    for contract in database_data.into_iter() {
        let eth_address = zinc_zksync::eth_address_from_vec(contract.eth_address);
        let eth_private_key = zinc_zksync::eth_private_key_from_vec(contract.eth_private_key);

        log::info!(
            "{} instance `{}` of the contract `{} v{}` with address {}",
            "Loaded".bright_green(),
            contract.instance,
            contract.name,
            contract.version,
            serde_json::to_string(&eth_address).expect(zinc_const::panic::DATA_CONVERSION),
        );

        let application = BuildApplication::try_from_slice(contract.bytecode.as_slice())
            .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);

        let build = match application {
            BuildApplication::Circuit(_circuit) => {
                panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
            }
            BuildApplication::Contract(contract) => contract,
        };

        let provider = zksync::Provider::new(network);
        let wallet_credentials = zksync::WalletCredentials::from_eth_signer(
            eth_address,
            PrivateKeySigner::new(eth_private_key),
            network,
        )
        .await?;
        let wallet = zksync::Wallet::new(provider, wallet_credentials).await?;

        let database_fields = postgresql
            .select_fields(FieldSelectInput::new(contract.account_id as AccountId))
            .await?;

        let storage = ContractStorage::new_with_data(
            database_fields,
            build.storage.as_slice(),
            eth_address,
            &wallet,
        )
        .await?;

        contracts.insert(
            eth_address,
            SharedDataContract::new(
                eth_address,
                contract.name,
                contract.version,
                contract.instance,
                contract.source_code,
                contract.bytecode,
                contract.verifying_key,
                Some(contract.account_id as AccountId),
                eth_private_key,
                build,
                storage,
            ),
        );
    }

    let data = SharedData::new(postgresql, contracts).wrap();

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
