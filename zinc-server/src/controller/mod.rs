//!
//! The Zinc server controller.
//!

pub mod entry;
pub mod head;
pub mod instance;
pub mod program;

use actix_web::web;

///
/// The Zinc server routing initializer.
///
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(
                    web::scope("/program")
                        .service(
                            web::resource("")
                                .route(web::head().to(head::handle))
                                .route(web::get().to(program::get::handle))
                                .route(web::post().to(program::post::handle)),
                        )
                        .service(
                            web::resource("/source")
                                .route(web::head().to(head::handle))
                                .route(web::get().to(program::source::get::handle)),
                        ),
                )
                .service(
                    web::scope("/entry").service(
                        web::resource("/types")
                            .route(web::head().to(head::handle))
                            .route(web::get().to(entry::types::get::handle)),
                    ),
                )
                .service(
                    web::scope("/instance")
                        .service(
                            web::resource("/")
                                .route(web::head().to(head::handle))
                                .route(web::post().to(instance::post::handle)),
                        )
//                        .service(
//                            web::resource("/run")
//                                .route(web::head().to(head::handle))
//                                .route(web::post().to(instance::run::handle)),
//                        ),
                ),
        ),
    );
}
