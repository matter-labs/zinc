//!
//! The HEAD method module.
//!

use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::Responder;

///
/// The HEAD method endpoint handler.
///
pub async fn handle() -> impl Responder {
    HttpResponse::new(StatusCode::OK)
}
