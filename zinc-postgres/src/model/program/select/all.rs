//!
//! The PostgreSQL program SELECT all models.
//!

///
/// The PostgreSQL program SELECT all models.
///
#[derive(sqlx::FromRow)]
pub struct Output {
    /// The program unique ID.
    pub id: i32,
    /// The program name.
    pub name: String,
    /// The program version.
    pub version: String,
}
