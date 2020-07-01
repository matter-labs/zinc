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
        web::resource("/program")
            .route(web::get().to(self::program::get::handle))
            .route(web::post().to(self::program::post::handle))
            .route(web::delete().to(self::program::delete::handle)),
    );
}
