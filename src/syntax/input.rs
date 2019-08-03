//!
//! The syntax input.
//!

use crate::syntax::Identificator;
use crate::syntax::Type;

#[derive(Debug)]
pub struct Input {
    id: Identificator,
    r#type: Type,
}
