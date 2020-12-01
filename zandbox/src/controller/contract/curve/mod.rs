//!
//! The contract resource GET method `Curve` module.
//!

pub mod response;

use actix_web::http::StatusCode;

use crate::error::Error;
use crate::response::Response;

use self::response::Body as ResponseBody;
use self::response::Instance as ResponseInstance;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get all the contract instances with the name 'curve' from the database.
/// 2. Return the instances to the client.
///
pub async fn handle(app_data: crate::WebData) -> crate::Result<ResponseBody, Error> {
    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();

    let response: ResponseBody = postgresql
        .select_contracts_curve(None)
        .await?
        .into_iter()
        .map(|instance| {
            ResponseInstance::new(
                zinc_types::address_from_slice(instance.eth_address.as_slice()),
                instance.name,
                instance.version,
                instance.instance,
            )
        })
        .collect();

    Ok(Response::new_with_data(StatusCode::OK, response))
}
