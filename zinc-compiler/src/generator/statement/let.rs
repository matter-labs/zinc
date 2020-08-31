//!
//! The generator `let` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;

use zinc_build::Instruction;

use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::expression::Expression;
use crate::generator::r#type::Type;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The Zinc VM storage memory allocating statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    /// The statement location in the source code.
    pub location: Location,
    /// The declared variable name.
    pub name: String,
    /// The declared variable type.
    pub r#type: Type,
    /// The expression assigned to the variable.
    pub expression: Expression,
}

impl Statement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        name: String,
        r#type: SemanticType,
        expression: Expression,
    ) -> Option<Self> {
        Type::try_from_semantic(&r#type).map(|r#type| Self {
            location,
            name,
            r#type,
            expression,
        })
    }
}

impl IBytecodeWritable for Statement {
    fn write_all(self, state: Rc<RefCell<State>>) {
        let size = self.r#type.size();
        let address = state.borrow_mut().define_variable(Some(self.name), size);

        self.expression.write_all(state.clone());

        match self.r#type {
            Type::Contract { fields } => {
                for (index, (_name, r#type)) in fields.into_iter().enumerate().rev() {
                    IntegerConstant::new(BigInt::from(index), false, zinc_const::bitlength::FIELD)
                        .write_all(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::StorageStore(zinc_build::StorageStore::new(r#type.size())),
                        Some(self.location),
                    );
                }
            }
            r#type => {
                if let Some(scalar_type) = r#type.into() {
                    state.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_build::Cast::new(scalar_type)),
                        Some(self.location),
                    );
                }
                state.borrow_mut().push_instruction(
                    Instruction::Store(zinc_build::Store::new(address, size)),
                    Some(self.location),
                );
            }
        }
    }
}
