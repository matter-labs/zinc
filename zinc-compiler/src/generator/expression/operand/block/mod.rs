//!
//! The generator expression block operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_build::Instruction;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::state::State;
use crate::generator::statement::Statement;
use crate::generator::IBytecodeWritable;

///
/// The block expression which is translated to Zinc VM bytecode.
///
#[derive(Debug, Clone)]
pub struct Expression {
    /// Whether the block is marked as `unconstrained`.
    is_unconstrained: bool,
    /// The block statements.
    statements: Vec<Statement>,
    /// The optional block expressions, whose type is defaulted to `()` if unset.
    expression: Option<GeneratorExpression>,
}

impl Expression {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        is_unconstrained: bool,
        statements: Vec<Statement>,
        expression: Option<GeneratorExpression>,
    ) -> Self {
        Self {
            is_unconstrained,
            statements,
            expression,
        }
    }
}

impl IBytecodeWritable for Expression {
    fn write_all(self, bytecode: Rc<RefCell<State>>) {
        if self.is_unconstrained {
            bytecode.borrow_mut().push_instruction(
                Instruction::SetUnconstrained(zinc_build::SetUnconstrained),
                None,
            );
        }

        for statement in self.statements.into_iter() {
            statement.write_all(bytecode.clone());
        }
        if let Some(expression) = self.expression {
            expression.write_all(bytecode.clone());
        }

        if self.is_unconstrained {
            bytecode.borrow_mut().push_instruction(
                Instruction::UnsetUnconstrained(zinc_build::UnsetUnconstrained),
                None,
            );
        }
    }
}
