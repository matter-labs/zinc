//!
//! The Zandbox controller.
//!

pub mod contract;
pub mod head;
pub mod project;

use actix_web::web;

///
/// The Zandbox router.
///
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(
                    web::scope("/contract")
                        .service(
                            web::resource("")
                                .route(web::head().to(head::handle))
                                .route(web::post().to(contract::publish::handle)),
                        )
                        .service(
                            web::resource("/curve")
                                .route(web::head().to(head::handle))
                                .route(web::get().to(contract::curve::handle)),
                        )
                        .service(
                            web::resource("/initialize")
                                .route(web::head().to(head::handle))
                                .route(web::post().to(contract::initialize::handle)),
                        )
                        .service(
                            web::resource("/query")
                                .route(web::head().to(head::handle))
                                .route(web::put().to(contract::query::handle)),
                        )
                        .service(
                            web::resource("/call")
                                .route(web::head().to(head::handle))
                                .route(web::post().to(contract::call::handle)),
                        )
                        .service(
                            web::resource("/fee")
                                .route(web::head().to(head::handle))
                                .route(web::put().to(contract::fee::handle)),
                        ),
                )
                .service(
                    web::scope("/project")
                        .service(
                            web::resource("")
                                .route(web::head().to(head::handle))
                                .route(web::get().to(project::metadata::handle))
                                .route(web::post().to(project::upload::handle)),
                        )
                        .service(
                            web::resource("/source")
                                .route(web::head().to(head::handle))
                                .route(web::get().to(project::source::handle)),
                        ),
                ),
        ),
    );
}
