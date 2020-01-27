//!
//! The semantic analyzer standard library `to_bits` function type element.
//!

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
pub struct ToBitsStandardLibraryFunction {
    pub identifier: &'static str,
}

impl ToBitsStandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "to_bits",
        }
    }

    pub fn builtin_identifier() -> BuiltinIdentifier {
        BuiltinIdentifier::ToBits
    }

    pub fn arguments_count(&self) -> usize {
        1
    }

    pub fn simulate(&self, input: &Type) -> Option<Type> {
        match input {
            Type::Boolean => Some(Type::new_array(
                Type::new_boolean(),
                crate::BITLENGTH_BOOLEAN,
            )),
            Type::IntegerUnsigned { bitlength } => {
                Some(Type::new_array(Type::new_boolean(), *bitlength))
            }
            Type::Field => Some(Type::new_array(
                Type::new_boolean(),
                crate::BITLENGTH_FIELD_PADDED,
            )),
            _ => None,
        }
    }
}

impl fmt::Display for ToBitsStandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn std::{}(value: field) -> [bool: N]", self.identifier,)
    }
}
