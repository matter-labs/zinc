//!
//! The project resource GET method `source` module.
//!

use actix_web::http::StatusCode;
use actix_web::web;

use crate::database::model::project::select_source::Input as ProjectSelectSourceInput;
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
    query: web::Query<zinc_zksync::SourceRequestQuery>,
) -> crate::Result<zinc_zksync::SourceResponseBody, Error> {
    let query = query.into_inner();

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();

    let response = postgresql
        .select_project_source(
            ProjectSelectSourceInput::new(query.name, query.version),
            None,
        )
        .await
        .map(|response| {
            zinc_zksync::SourceResponseBody::new(
                response.zinc_version,
                serde_json::from_value::<zinc_source::Project>(response.project)
                    .expect(zinc_const::panic::DATA_CONVERSION),
            )
        })?;

    Ok(Response::new_with_data(StatusCode::OK, response))
}
