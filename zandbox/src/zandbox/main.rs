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
use zinc_compiler::State;

use zandbox::ContractSelectOutput;
use zandbox::DatabaseClient;
use zandbox::SharedData;

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
                 source_code,
             }| {
                log::info!(
                    "{} {} v{} (ID {})",
                    "Compiling".bright_green(),
                    name,
                    version,
                    contract_id
                );

                let source: zinc_source::Source = serde_json::from_value(source_code)
                    .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);
                let source = zinc_compiler::Source::try_from_string(source, true)
                    .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION);

                let state = State::unwrap_rc(
                    source
                        .compile(name)
                        .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
                );

                let contract = match state.into_program(true) {
                    BuildProgram::Circuit(_circuit) => {
                        panic!(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION)
                    }
                    BuildProgram::Contract(contract) => contract,
                };

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
