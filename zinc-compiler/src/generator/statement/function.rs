//!
//! The generator function statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::bytecode::Bytecode;
use crate::generator::expression::operand::block::Expression;
use crate::generator::r#type::Type;
use crate::semantic::Type as SemanticType;

#[derive(Debug, Clone)]
pub struct Statement {
    pub identifier: String,
    pub input_arguments: Vec<(String, Type)>,
    pub body: Expression,
    pub output_type: Option<Type>,
    pub unique_id: usize,
    pub is_main: bool,
}

impl Statement {
    pub fn new(
        identifier: String,
        input_arguments: Vec<(String, SemanticType)>,
        body: Expression,
        output_type: SemanticType,
        unique_id: usize,
        is_main: bool,
    ) -> Self {
        let input_arguments = input_arguments
            .into_iter()
            .filter_map(|(name, r#type)| match Type::try_from_semantic(&r#type) {
                Some(r#type) => Some((name, r#type)),
                None => None,
            })
            .collect();

        let output_type = Type::try_from_semantic(&output_type);

        Self {
            identifier,
            input_arguments,
            body,
            output_type,
            unique_id,
            is_main,
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        let output_size = self
            .output_type
            .as_ref()
            .map(|r#type| r#type.size())
            .unwrap_or(0);

        if self.is_main {
            bytecode.borrow_mut().start_main_function(
                self.unique_id,
                self.input_arguments.clone(),
                self.output_type,
            );
        } else {
            bytecode
                .borrow_mut()
                .start_function(self.unique_id, self.identifier);
        }

        for (argument_name, argument_type) in self.input_arguments.into_iter() {
            bytecode
                .borrow_mut()
                .declare_variable(Some(argument_name), argument_type);
        }

        self.body.write_all_to_bytecode(bytecode.clone());

        if self.is_main {
            bytecode.borrow_mut().push_instruction(
                Instruction::Exit(zinc_bytecode::Exit::new(output_size)),
                crate::lexical::Location::default(),
            );
        } else {
            bytecode.borrow_mut().push_instruction(
                Instruction::Return(zinc_bytecode::Return::new(output_size)),
                crate::lexical::Location::default(),
            );
        }
    }
}
