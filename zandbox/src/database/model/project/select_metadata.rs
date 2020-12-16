//!
//! The database project SELECT metadata model.
//!

///
/// The database project SELECT metadata output model.
///
#[derive(Debug, sqlx::FromRow)]
pub struct Output {
    /// The project name.
    pub name: String,
    /// The project version.
    pub version: String,
}
