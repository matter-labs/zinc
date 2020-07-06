//!
//! The Zinc server binary.
//!

mod arguments;
mod error;

use actix_web::middleware;
use actix_web::App;
use actix_web::HttpServer;
use mongodb::options::ClientOptions as MongoClientOptions;
use mongodb::Client as MongoClient;

use zinc_server::SharedData;

use self::arguments::Arguments;
use self::error::Error;

///
/// The application entry point.
///
#[actix_rt::main]
async fn main() -> Result<(), Error> {
    let args = Arguments::new();

    zinc_utils::logger::initialize(zinc_const::app_name::ZINC_SERVER, args.verbosity);

    let data = SharedData::new(
        MongoClient::with_options(
            MongoClientOptions::parse(
                format!(
                    "mongodb://{}:{}",
                    args.mongodb_host,
                    args.mongodb_port.unwrap_or(zinc_const::mongodb::PORT)
                )
                .as_str(),
            )
            .await
            .map_err(Error::MongoDbOptions)?,
        )
        .map_err(Error::MongoDbClient)?,
    )
    .wrap();

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::DefaultHeaders::new().content_type())
            .wrap(middleware::Logger::default())
            .configure(zinc_server::configure)
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
