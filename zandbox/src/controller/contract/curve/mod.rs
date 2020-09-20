//!
//! The contract resource GET method `Curve` module.
//!

pub mod error;
pub mod response;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;

use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::response::Body as ResponseBody;
use self::response::Instance as ResponseInstance;

///
/// The HTTP request handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
) -> crate::Result<ResponseBody, Error> {
    let response: ResponseBody = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql_client
        .select_contracts_curve()
        .await
        .map_err(Error::Database)?
        .into_iter()
        .map(|instance| {
            ResponseInstance::new(
                instance.account_id,
                instance.name,
                instance.version,
                instance.instance,
            )
        })
        .collect();

    Ok(Response::new_with_data(StatusCode::OK, response))
}
