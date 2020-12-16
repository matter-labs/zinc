//!
//! The project resource POST method `upload` module.
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
/// 1. Write the uploaded project to the database.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_types::UploadRequestQuery>,
    body: web::Json<zinc_types::UploadRequestBody>,
) -> crate::Result<(), Error> {
    let query = query.into_inner();
    let body = body.into_inner();
    let log_id = format!("{}-{}", query.name, query.version);

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();

    postgresql
        .insert_project(
            model::project::insert_one::Input::new(
                query.name.clone(),
                query.version.clone(),
                semver::Version::parse(env!("CARGO_PKG_VERSION"))
                    .expect(zinc_const::panic::DATA_CONVERSION),
                body.project,
                body.bytecode,
                body.verifying_key,
            ),
            None,
        )
        .await?;

    log::info!("[{}] Project uploaded", log_id);

    Ok(Response::new(StatusCode::CREATED))
}
