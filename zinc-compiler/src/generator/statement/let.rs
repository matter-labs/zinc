//!
//! The generator `let` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num::BigInt;

use zinc_build::Instruction;
use zinc_lexical::Location;

use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::expression::Expression;
use crate::generator::r#type::Type;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::semantic::binding::Binding;

///
/// The Zinc VM storage memory allocating statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    /// The statement location in the source code.
    pub location: Location,
    /// The declaration bindings.
    pub bindings: Vec<Binding>,
    /// The expression assigned to the bindings.
    pub expression: Expression,
}

impl Statement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, bindings: Vec<Binding>, expression: Expression) -> Self {
        Self {
            location,
            bindings,
            expression,
        }
    }
}

impl IBytecodeWritable for Statement {
    fn write_all(self, state: Rc<RefCell<State>>) {
        self.expression.write_all(state.clone());

        for binding in self.bindings.into_iter().rev() {
            let r#type = match Type::try_from_semantic(&binding.r#type) {
                Some(r#type) => r#type,
                None => continue,
            };

            let size = r#type.size();
            let address = state
                .borrow_mut()
                .define_variable(Some(binding.identifier.name), size);

            match r#type {
                Type::Contract { fields } => {
                    for (index, field) in fields.into_iter().enumerate().rev() {
                        IntegerConstant::new(
                            BigInt::from(index),
                            false,
                            zinc_const::bitlength::FIELD,
                        )
                        .write_all(state.clone());
                        state.borrow_mut().push_instruction(
                            Instruction::StorageStore(zinc_build::StorageStore::new(
                                field.r#type.size(),
                            )),
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
}
