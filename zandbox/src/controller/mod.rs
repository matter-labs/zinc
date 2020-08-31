//!
//! The Zandbox server daemon controller.
//!

pub mod contract;
pub mod head;
pub mod method;

use actix_web::web;

///
/// The Zandbox server daemon routing initializer.
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
                )
                .service(
                    web::scope("/method").service(
                        web::resource("/types")
                            .route(web::head().to(head::handle))
                            .route(web::get().to(method::types::get::handle)),
                    ),
                ),
        ),
    );
}
