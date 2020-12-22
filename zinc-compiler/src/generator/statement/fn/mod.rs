//!
//! The generator `fn` statement.
//!

pub mod role;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_types::Instruction;

use crate::generator::expression::operand::block::Expression;
use crate::generator::r#type::Type;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::analyzer::attribute::Attribute;
use crate::semantic::binding::Binding;
use crate::semantic::element::r#type::Type as SemanticType;

use self::role::Role;

///
/// The generator `fn` statement.
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
    pub input_arguments: Vec<(String, bool, Type)>,
    /// The function body.
    pub body: Expression,
    /// The function result type, which defaults to `()` if not specified.
    pub output_type: Type,
    /// The function unique ID, which is assigned during the semantic analysis.
    pub type_id: usize,
    /// The special function role, e.g. circuit entry or contract constructor.
    pub role: Role,
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
        bindings: Vec<Binding>,
        body: Expression,
        output_type: SemanticType,
        type_id: usize,
        role: Role,
        attributes: Vec<Attribute>,
    ) -> Self {
        let input_arguments = bindings
            .into_iter()
            .filter_map(|binding| match Type::try_from_semantic(&binding.r#type) {
                Some(r#type) => Some((binding.identifier.name, binding.is_mutable, r#type)),
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
            role,
            attributes,
        }
    }
}

impl IBytecodeWritable for Statement {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        let output_size = self.output_type.size();

        match self.role {
            Role::CircuitEntry | Role::ContractMethodEntry | Role::ContractConstuctor { .. } => {
                state.borrow_mut().start_entry_function(
                    self.location,
                    self.type_id,
                    self.identifier,
                    self.is_mutable,
                    self.input_arguments.clone(),
                    self.output_type.clone(),
                );
            }
            Role::UnitTest => {
                state.borrow_mut().start_unit_test_function(
                    self.location,
                    self.type_id,
                    self.identifier,
                    self.attributes,
                );
            }
            _ => {
                state
                    .borrow_mut()
                    .start_function(self.location, self.type_id, self.identifier);
            }
        }

        for (name, _is_mutable, r#type) in self.input_arguments.into_iter() {
            let size = match r#type {
                Type::Contract { .. } => Type::eth_address().size(),
                argument_type => argument_type.size(),
            };

            state.borrow_mut().define_variable(Some(name), size);
        }

        self.body.write_to_zinc_vm(state.clone());

        match self.role {
            Role::ContractConstuctor { project } => {
                let field_types: Vec<zinc_types::ContractFieldType> = match self.output_type {
                    Type::Contract { fields } => {
                        fields.into_iter().map(|field| field.into()).collect()
                    }
                    _ => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
                };

                state.borrow_mut().push_instruction(
                    Instruction::StorageInit(zinc_types::StorageInit::new(project, field_types)),
                    Some(self.location),
                );
                state.borrow_mut().push_instruction(
                    Instruction::Return(zinc_types::Return::new(Type::eth_address().size())),
                    Some(self.location),
                );
            }
            _ => {
                state.borrow_mut().push_instruction(
                    Instruction::Return(zinc_types::Return::new(output_size)),
                    Some(self.location),
                );
            }
        }
    }
}
