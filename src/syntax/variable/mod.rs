//!
//! The syntax variable.
//!

mod name;

pub use self::name::Error as NameError;
pub use self::name::Name;

use crate::syntax::Type;

pub struct Variable {
    name: Name,
    r#type: Type,
}
