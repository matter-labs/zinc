//!
//! The Zandbox server daemon binary.
//!

mod arguments;
mod error;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use colored::Colorize;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::ThreadPoolBuilder;

use zksync::zksync_models::node::AccountId;

use zinc_build::Program as BuildProgram;

use zandbox::ContractSelectAllOutput;
use zandbox::DatabaseClient;
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

    log::info!("Initializing the contract deserializing thread pool");
    ThreadPoolBuilder::new()
        .stack_size(zinc_const::limit::COMPILER_STACK_SIZE)
        .build_global()
        .expect(zinc_const::panic::THREAD_POOL);

    log::info!("Loading the compiled contracts from the database");
    let contracts = database_client
        .select_contracts()
        .await?
        .into_par_iter()
        .map(
            |ContractSelectAllOutput {
                 account_id,
                 name,
                 version,
                 instance,
                 bytecode,
                 eth_address,
                 eth_private_key,
             }| {
                let program = BuildProgram::try_from_slice(bytecode.as_slice())
                    .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);

                let build = match program {
                    BuildProgram::Circuit(_circuit) => {
                        panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
                    }
                    BuildProgram::Contract(contract) => contract,
                };

                let contract = SharedDataContract::new(
                    build,
                    zinc_utils::eth_address_from_vec(eth_address),
                    zinc_utils::eth_private_key_from_vec(eth_private_key),
                );

                log::info!(
                    "{} [ID {:6}] instance `{}` of the contract `{} v{}`",
                    "Loaded".bright_green(),
                    account_id,
                    instance,
                    name,
                    version,
                );

                (account_id as AccountId, contract)
            },
        )
        .collect();

    let data = SharedData::new(database_client, contracts).wrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().content_type())
            .wrap(middleware::Logger::default())
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
