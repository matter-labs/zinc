//!
//! The Zinc server library.
//!

pub(crate) mod app_data;
pub(crate) mod program;
pub(crate) mod status;

pub use self::app_data::AppData;

use actix_web::web;

///
/// The Zinc server routing initializer.
///
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/program")
            .service(
                web::resource("")
                    .route(web::post().to(self::program::post::handle))
                    .route(web::patch().to(self::program::patch::handle))
                    .route(web::delete().to(self::program::delete::handle)),
            )
            .service(
                web::resource("/source").route(web::get().to(self::program::source::get::handle)),
            )
            .service(
                web::resource("/input_template")
                    .route(web::get().to(self::program::input_template::get::handle)),
            )
            .service(
                web::resource("/output_template")
                    .route(web::get().to(self::program::output_template::get::handle)),
            ),
    );
}
