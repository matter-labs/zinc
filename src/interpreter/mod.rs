//!
//! The interpreter tools.
//!

mod error;
mod executor;
mod place;
mod scope;
mod stack;
mod value;
mod warning;

pub use self::error::Error;
pub use self::error::OperatorError;
pub use self::executor::Executor;
pub use self::place::Place;
pub use self::scope::Scope;
pub use self::stack::Element as StackElement;
pub use self::value::Value;
pub use self::warning::Warning;

use crate::syntax::CircuitProgram;

pub fn interpret(program: CircuitProgram) -> Result<(), Error> {
    let mut executor = Executor::default();
    for statement in program.statements.into_iter() {
        log::debug!("{}", statement);
        executor.execute(statement)?;
    }
    Ok(())
}
