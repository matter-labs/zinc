//!
//! The Zinc server binary.
//!

mod arguments;
mod error;

use actix_web::middleware;
use actix_web::App;
use actix_web::HttpServer;

use zinc_server::AppData;

use self::arguments::Arguments;
use self::error::Error;

///
/// The application entry point.
///
#[actix_rt::main]
async fn main() -> Result<(), Error> {
    let args = Arguments::new();

    zinc_utils::logger::initialize(zinc_const::app_name::ZINC_SERVER, args.verbosity);

    let data = AppData::new().wrap();

    let address = format!("{}:{}", zinc_const::http::HOST, args.port);

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Logger::default())
            .configure(zinc_server::configure)
    })
    .bind(address)
    .map_err(Error::ServerBinding)?
    .run()
    .await
    .map_err(Error::ServerRuntime)
}
