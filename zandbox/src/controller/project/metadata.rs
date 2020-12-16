//!
//! The project resource GET method `metadata` module.
//!

use std::str::FromStr;

use actix_web::http::StatusCode;

use crate::error::Error;
use crate::response::Response;

///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Gets the projects metadata from the database.
/// 2. Returns the metadata to the client.
///
pub async fn handle(
    app_data: crate::WebData,
) -> crate::Result<zinc_types::MetadataResponseBody, Error> {
    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();

    let response = postgresql
        .select_projects_metadata(None)
        .await
        .map(|response| {
            let projects = response
                .into_iter()
                .map(|record| {
                    zinc_project::ManifestProject::new(
                        record.name,
                        zinc_project::ProjectType::Contract,
                        semver::Version::from_str(record.version.as_str())
                            .expect(zinc_const::panic::VALIDATED_DURING_DATABASE_POPULATION),
                    )
                })
                .collect();
            zinc_types::MetadataResponseBody::new(projects)
        })?;

    Ok(Response::new_with_data(StatusCode::OK, response))
}
