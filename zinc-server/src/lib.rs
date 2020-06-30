//!
//! The Zinc server library.
//!

pub(crate) mod app_data;
pub(crate) mod compiler;
pub(crate) mod panic;

pub use self::app_data::AppData;

use actix_web::web;

///
/// The Zinc server routing initializer.
///
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(web::resource("/compiler").route(web::post().to(self::compiler::post::handle)));
}
