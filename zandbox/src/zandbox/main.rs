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

use zinc_build::Program as BuildProgram;

use zandbox::ContractSelectOutput;
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

    let database_client = DatabaseClient::new(
        args.postgresql_host,
        args.postgresql_port.unwrap_or(zinc_const::postgresql::PORT),
        args.postgresql_user,
        args.postgresql_password,
        args.postgresql_database,
    )
    .await?;

    ThreadPoolBuilder::new()
        .stack_size(zinc_const::limit::COMPILER_STACK_SIZE)
        .build_global()
        .expect(zinc_const::panic::RAYON_POOL_INITIALIZATION);

    let contracts = database_client
        .select_contracts()
        .await?
        .into_par_iter()
        .map(
            |ContractSelectOutput {
                 contract_id,
                 name,
                 version,
                 bytecode,
                 eth_address,
                 private_key,
             }| {
                log::info!(
                    "{} {} v{} with ID {}",
                    "Loading".bright_green(),
                    name,
                    version,
                    contract_id
                );

                let program = BuildProgram::try_from_slice(bytecode.as_slice())
                    .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);

                let build = match program {
                    BuildProgram::Circuit(_circuit) => {
                        panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
                    }
                    BuildProgram::Contract(contract) => contract,
                };

                let mut eth_address_array = [0; zinc_const::size::ETH_ADDRESS];
                for (index, byte) in eth_address.into_iter().enumerate() {
                    eth_address_array[index] = byte;
                }
                let mut private_key_array = [0; zinc_const::size::ETH_PRIVATE_KEY];
                for (index, byte) in private_key.into_iter().enumerate() {
                    private_key_array[index] = byte;
                }

                let contract = SharedDataContract::new(build, eth_address_array, private_key_array);

                (contract_id, contract)
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
        zinc_const::http::HOST,
        args.http_port.unwrap_or(zinc_const::http::PORT)
    ))
    .map_err(Error::ServerBinding)?
    .run()
    .await
    .map_err(Error::ServerRuntime)
}
