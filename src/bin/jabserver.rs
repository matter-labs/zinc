//!
//! The Jabberwocky server binary.
//!

use std::str;

use actix_web::middleware;
use actix_web::web;
use actix_web::web::Payload;
use actix_web::App;
use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use bytes::BytesMut;
use failure::Fail;
use futures::prelude::*;
use serde_derive::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jabserver", about = "The Jabberwocky language server")]
struct Arguments {
    #[structopt(short = "p", long = "port", default_value = "80")]
    port: u16,
}

#[derive(Debug, Fail)]
enum ServerError {
    #[fail(display = "Binding: {}", _0)]
    Binding(std::io::Error),
    #[fail(display = "Running: {}", _0)]
    Running(std::io::Error),
}

fn main() -> Result<(), ServerError> {
    init_logger();

    let args: Arguments = Arguments::from_args();

    let address = format!("0.0.0.0:{}", args.port);
    log::info!("Starting the HTTP server at {}", address);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/compile").to_async(handler))
    })
    .bind(address)
    .map_err(ServerError::Binding)?
    .run()
    .map_err(ServerError::Running)
}

#[derive(Serialize)]
struct CircuitError {
    error: String,
}

fn handler(payload: Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    payload
        .from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(|body| {
            let input = body.to_vec();
            log::info!("Received: {}", unsafe { str::from_utf8_unchecked(&input) });
            let response = match compiler::parse(input) {
                Ok(circuit) => serde_json::to_vec(&circuit),
                Err(error) => serde_json::to_vec(&CircuitError {
                    error: error.to_string(),
                }),
            }
            .expect("Serialization bug");

            HttpResponse::Ok().body(response)
        })
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "compiler=info,jabserver=info");
    }
    env_logger::Builder::from_default_env()
        .default_format_timestamp_nanos(true)
        .init();
}
