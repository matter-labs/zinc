//!
//! The Zinc server library.
//!

pub(crate) mod head;
pub(crate) mod program;
pub(crate) mod response;
pub(crate) mod shared_data;

pub use self::shared_data::SharedData;

use actix_web::web;

///
/// The Zinc server routing initializer.
///
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api").service(
            web::scope("/v1").service(
                web::scope("/program")
                    .service(
                        web::resource("")
                            .route(web::head().to(head::handle))
                            .route(web::put().to(self::program::put::handle))
                            .route(web::delete().to(self::program::delete::handle)),
                    )
                    .service(
                        web::resource("/run")
                            .route(web::head().to(head::handle))
                            .route(web::post().to(self::program::run::handle)),
                    )
                    .service(
                        web::resource("/source")
                            .route(web::head().to(head::handle))
                            .route(web::get().to(self::program::source::get::handle)),
                    )
                    .service(
                        web::resource("/input_template")
                            .route(web::head().to(head::handle))
                            .route(web::get().to(self::program::input_template::get::handle)),
                    )
                    .service(
                        web::resource("/output_template")
                            .route(web::head().to(head::handle))
                            .route(web::get().to(self::program::output_template::get::handle)),
                    ),
            ),
        ),
    );
}
