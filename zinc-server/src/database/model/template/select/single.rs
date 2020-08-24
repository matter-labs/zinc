//!
//! The database template SELECT single models.
//!

///
/// The database template SELECT single input model.
///
pub struct Input {
    /// The template account unique ID.
    pub account_id: i64,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: i64) -> Self {
        Self { account_id }
    }
}

///
/// The database template SELECT source output model.
///
#[derive(sqlx::FromRow)]
pub struct Output {
    /// The template bytecode.
    pub bytecode: Vec<u8>,
    /// The template verifying key as a byte array.
    pub verifying_key: Vec<u8>,
}
