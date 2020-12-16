//!
//! The project resource GET method `source` module.
//!

use actix_web::http::StatusCode;
use actix_web::web;

use crate::database::model;
use crate::error::Error;
use crate::response::Response;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract from the in-memory cache.
/// 2. Return the contract source code to the client.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_types::SourceRequestQuery>,
) -> crate::Result<zinc_types::SourceResponseBody, Error> {
    let query = query.into_inner();

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();

    let response = postgresql
        .select_project_source(
            model::project::select_source::Input::new(query.name, query.version),
            None,
        )
        .await
        .map(|response| {
            zinc_types::SourceResponseBody::new(
                response.zinc_version,
                serde_json::from_value::<zinc_project::Project>(response.project)
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
        })?;

    Ok(Response::new_with_data(StatusCode::OK, response))
}
