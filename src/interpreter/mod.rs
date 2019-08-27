//!
//! The interpreter tools.
//!

mod error;
mod evaluator;
mod executor;
mod field;
mod warning;

pub use self::error::Error;
pub use self::evaluator::Evaluator;
pub use self::executor::Executor;
pub use self::field::Error as OperatorError;
pub use self::field::Field;
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
