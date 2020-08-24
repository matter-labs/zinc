//!
//! The Zinc server controller.
//!

pub mod contract;
pub mod head;
pub mod method;
pub mod template;

use actix_web::web;

///
/// The Zinc server routing initializer.
///
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(
                    web::scope("/template").service(
                        web::resource("")
                            .route(web::head().to(head::handle))
                            .route(web::get().to(template::get::handle))
                            .route(web::post().to(template::post::handle)),
                    ),
                )
                .service(
                    web::scope("/method").service(
                        web::resource("/types")
                            .route(web::head().to(head::handle))
                            .route(web::get().to(method::types::get::handle)),
                    ),
                )
                .service(
                    web::scope("/contract")
                        .service(
                            web::resource("")
                                .route(web::head().to(head::handle))
                                .route(web::post().to(contract::post::handle)),
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
                        ),
                ),
        ),
    );
}
