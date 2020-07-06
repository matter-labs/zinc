//!
//! The program resource DELETE method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Query;

///
/// The program resource DELETE method endpoint handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<Query>,
) -> impl Responder {
    let query = query.into_inner();

    let program = match app_data
        .write()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .remove_program(query.name.as_str())
    {
        Some(program) => program,
        None => return Response::new_error(Error::NotFound),
    };

    if program.contract_storage.is_some() {
        let mongodb_client = app_data
            .read()
            .expect(zinc_const::panic::MUTEX_SYNC)
            .mongodb_client
            .to_owned();

        if let Err(error) = mongodb_client
            .database(zinc_const::mongodb::DATABASE)
            .collection(query.name.as_str())
            .drop(None)
            .await
        {
            return Response::new_error(Error::MongoDb(error));
        }
    }

    Response::<(), _>::new_success(StatusCode::NO_CONTENT)
}
