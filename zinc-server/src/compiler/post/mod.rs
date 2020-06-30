//!
//! The compiler resource POST.
//!

pub mod payload;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::web;
use actix_web::Responder;

use zinc_compiler::Source;

use crate::app_data::AppData;

use self::payload::Payload;

///
/// The compiler POST method endpoint handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<AppData>>>,
    payload: web::Json<Payload>,
) -> impl Responder {
    let mut app_data = app_data.write().expect(crate::panic::MUTEX_SYNC);
    let payload = payload.into_inner();

    let source = Source::try_from_string(payload.source, true).unwrap();
    dbg!(source);

    app_data.count += 1;

    format!("Hello, World!")
}
