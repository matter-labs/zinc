//!
//! The interpreter tools.
//!

mod element;
mod error;
mod executor;
mod scope;
mod warning;

pub use self::element::Boolean;
pub use self::element::Element;
pub use self::element::Error as ElementError;
pub use self::element::Integer;
pub use self::element::Place;
pub use self::element::Value;
pub use self::element::ValueError;
pub use self::error::Error;
pub use self::executor::Executor;
pub use self::scope::Error as ScopeError;
pub use self::scope::Scope;
pub use self::scope::Warning as ScopeWarning;
pub use self::warning::Warning;

use crate::syntax::CircuitProgram;

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(program: CircuitProgram) -> Result<(), Error> {
        let mut executor = Executor::default();
        for statement in program.statements.into_iter() {
            executor.execute(statement)?;
        }
        Ok(())
    }
}
