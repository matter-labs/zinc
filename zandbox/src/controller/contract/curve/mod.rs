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
/// Sequence:
/// 1. Get all the contract instances with the name 'curve' from the database.
/// 2. Return the instances to the client.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
) -> crate::Result<ResponseBody, Error> {
    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql_client
        .clone();

    let response: ResponseBody = postgresql
        .select_contracts_curve()
        .await?
        .into_iter()
        .map(|instance| {
            ResponseInstance::new(
                zinc_utils::eth_address_from_vec(instance.address),
                instance.name,
                instance.version,
                instance.instance,
            )
        })
        .collect();

    Ok(Response::new_with_data(StatusCode::OK, response))
}
