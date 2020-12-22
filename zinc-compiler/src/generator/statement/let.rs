//!
//! The generator `let` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_types::Instruction;

use crate::generator::expression::Expression;
use crate::generator::r#type::Type;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::binding::Binding;

///
/// The generator `let` statement.
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
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        self.expression.write_to_zinc_vm(state.clone());

        for binding in self.bindings.into_iter().rev() {
            let r#type = match Type::try_from_semantic(&binding.r#type) {
                Some(r#type) => r#type,
                None => continue,
            };

            match r#type {
                Type::Contract { .. } => {
                    let size = Type::eth_address().size();
                    let address = state
                        .borrow_mut()
                        .define_variable(Some(binding.identifier.name), size);
                    state.borrow_mut().push_instruction(
                        Instruction::Store(zinc_types::Store::new(address, size)),
                        Some(self.location),
                    );
                }
                r#type => {
                    let size = r#type.size();
                    let address = state.borrow_mut().define_variable(
                        if !binding.is_wildcard {
                            Some(binding.identifier.name)
                        } else {
                            None
                        },
                        size,
                    );

                    if let Some(scalar_type) = r#type.into() {
                        state.borrow_mut().push_instruction(
                            Instruction::Cast(zinc_types::Cast::new(scalar_type)),
                            Some(self.location),
                        );
                    }
                    state.borrow_mut().push_instruction(
                        Instruction::Store(zinc_types::Store::new(address, size)),
                        Some(self.location),
                    );
                }
            }
        }
    }
}
