//!
//! The generator `let` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;

use zinc_bytecode::Instruction;

use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::expression::Expression;
use crate::generator::r#type::Type;
use crate::generator::state::State;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The Zinc VM storage memory allocating statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    pub location: Location,
    pub name: String,
    pub r#type: Type,
    pub expression: Expression,
}

impl Statement {
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

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<State>>) {
        let size = self.r#type.size();
        let address = bytecode.borrow_mut().define_variable(Some(self.name), size);

        self.expression.write_all_to_bytecode(bytecode.clone());

        match self.r#type {
            Type::Contract { fields } => {
                for (index, (_name, r#type)) in fields.into_iter().enumerate().rev() {
                    IntegerConstant::new(BigInt::from(index), false, zinc_const::bitlength::FIELD)
                        .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::StorageStore(zinc_bytecode::StorageStore::new(r#type.size())),
                        Some(self.location),
                    );
                }
            }
            r#type => {
                if let Some(scalar_type) = r#type.into() {
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_bytecode::Cast::new(scalar_type)),
                        Some(self.location),
                    );
                }
                bytecode.borrow_mut().push_instruction(
                    Instruction::Store(zinc_bytecode::Store::new(address, size)),
                    Some(self.location),
                );
            }
        }
    }
}
