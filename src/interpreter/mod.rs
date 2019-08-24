//!
//! The interpreter tools.
//!

mod error;
mod evaluator;
mod executor;
mod field;

pub use self::error::Error;
pub use self::evaluator::Evaluator;
pub use self::executor::Executor;
pub use self::field::Error as OperatorError;
pub use self::field::Field;

use crate::syntax::CircuitProgram;

pub fn interpret(program: CircuitProgram) {
    let mut executor = Executor::default();
    for statement in program.statements.into_iter() {
        log::debug!("{}", statement);
        let _ = executor
            .execute(statement)
            .map_err(|error| log::error!("{}", error));
    }
}
