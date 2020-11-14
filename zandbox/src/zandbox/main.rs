//!
//! The Zandbox server daemon binary.
//!

pub(crate) mod arguments;

use std::str::FromStr;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;

use zandbox::DatabaseClient;
use zandbox::SharedData;

use self::arguments::Arguments;

///
/// The application entry point.
///
#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::new();

    zinc_logger::initialize(zinc_const::app_name::ZANDBOX, args.verbosity);

    log::info!("Zandbox server started");

    let network = zksync::Network::from_str(args.network.as_str())
        .map_err(|network| anyhow::anyhow!(format!("Invalid network `{}`", network)))?;

    log::info!("Initializing the PostgreSQL client");
    let postgresql = DatabaseClient::new(args.postgresql_uri.as_str()).await?;

    let data = SharedData::new(postgresql, network).wrap();

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
    ))?
    .run()
    .await?;

    log::info!("Zandbox server finished");
    Ok(())
}
