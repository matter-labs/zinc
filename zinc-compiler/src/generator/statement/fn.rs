//!
//! The generator `fn` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_build::Instruction;

use crate::generator::expression::operand::block::Expression;
use crate::generator::r#type::Type;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::attribute::Attribute;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The Zinc VM function statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    /// The statement location in the source code.
    pub location: Location,
    /// The function name.
    pub identifier: String,
    /// Whether the function can mutate its arguments.
    pub is_mutable: bool,
    /// The function arguments, where the compile time only ones like `()` are already filtered out.
    pub input_arguments: Vec<(String, Type)>,
    /// The function body.
    pub body: Expression,
    /// The function result type, which defaults to `()` if not specified.
    pub output_type: Type,
    /// The function unique ID, which is assigned during the semantic analysis.
    pub type_id: usize,
    /// Whether the function is a contract entry.
    pub is_contract_entry: bool,
    /// Whether the function is a circuit entry.
    pub is_main: bool,
    /// The function attibutes, e.g. the unit test ones.
    pub attributes: Vec<Attribute>,
}

impl Statement {
    ///
    /// A shortcut constructor.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        location: Location,
        identifier: String,
        is_mutable: bool,
        input_arguments: Vec<(String, SemanticType)>,
        body: Expression,
        output_type: SemanticType,
        type_id: usize,
        is_contract_entry: bool,
        is_main: bool,
        attributes: Vec<Attribute>,
    ) -> Self {
        let input_arguments = input_arguments
            .into_iter()
            .filter_map(|(name, r#type)| match Type::try_from_semantic(&r#type) {
                Some(r#type) => Some((name, r#type)),
                None => None,
            })
            .collect();

        let output_type = Type::try_from_semantic(&output_type).unwrap_or_else(Type::unit);

        Self {
            location,
            identifier,
            is_mutable,
            input_arguments,
            body,
            output_type,
            type_id,
            is_contract_entry,
            is_main,
            attributes,
        }
    }
}

impl IBytecodeWritable for Statement {
    fn write_all(self, state: Rc<RefCell<State>>) {
        let output_size = self.output_type.size();

        if self.is_main || self.is_contract_entry {
            state.borrow_mut().start_entry_function(
                self.location,
                self.type_id,
                self.identifier,
                self.is_mutable,
                self.input_arguments.clone(),
                self.output_type,
            );
        } else if self.attributes.contains(&Attribute::Test) {
            state.borrow_mut().start_unit_test_function(
                self.location,
                self.type_id,
                self.identifier,
                self.attributes.contains(&Attribute::ShouldPanic),
                self.attributes.contains(&Attribute::Ignore),
            );
        } else {
            state
                .borrow_mut()
                .start_function(self.location, self.type_id, self.identifier);
        }

        for (argument_name, argument_type) in self.input_arguments.into_iter() {
            match argument_type {
                Type::Contract { .. } => {}
                argument_type => {
                    state
                        .borrow_mut()
                        .define_variable(Some(argument_name), argument_type.size());
                }
            }
        }

        self.body.write_all(state.clone());

        if self.is_main || self.is_contract_entry {
            state.borrow_mut().push_instruction(
                Instruction::Exit(zinc_build::Exit::new(output_size)),
                Some(self.location),
            );
        } else {
            state.borrow_mut().push_instruction(
                Instruction::Return(zinc_build::Return::new(output_size)),
                Some(self.location),
            );
        }
    }
}
