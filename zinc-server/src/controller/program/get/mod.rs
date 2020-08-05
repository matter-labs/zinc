//!
//! The program resource DELETE method module.
//!

pub mod error;
pub mod response;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;

use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::response::Body;
use self::response::Program;

///
/// The HTTP request handler.
///
pub async fn handle(app_data: web::Data<Arc<RwLock<SharedData>>>) -> impl Responder {
    let programs = match app_data
        .read()
        .expect(zinc_const::panic::MUTEX_SYNC)
        .postgresql_client
        .select_programs_all()
        .await
    {
        Ok(programs) => programs,
        Err(error) => return Response::error(Error::Postgresql(error)),
    };

    let programs: Body = programs.into_iter().map(Program::from).collect();

    Response::success_with_data(StatusCode::OK, programs)
}
