//!
//! The program resource GET response.
//!

use serde_derive::Serialize;

use zinc_postgres::ProgramSelectOutput;

///
/// The program resource GET response body.
///
pub type Body = Vec<Program>;

///
/// The program resource GET response program.
///
#[derive(Debug, Serialize)]
pub struct Program {
    /// The program unique ID.
    pub id: i32,
    /// The name of the uploaded program.
    pub name: String,
    /// The version of the uploaded program.
    pub version: String,
}

impl From<ProgramSelectOutput> for Program {
    fn from(value: ProgramSelectOutput) -> Self {
        Self {
            id: value.id,
            name: value.name,
            version: value.version,
        }
    }
}
