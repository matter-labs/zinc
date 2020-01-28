//!
//! The semantic analyzer standard library `from_bits` function type element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
pub struct FromBitsStandardLibraryFunction {
    pub identifier: &'static str,
}

impl FromBitsStandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "from_bits",
        }
    }

    pub fn builtin_identifier() -> BuiltinIdentifier {
        BuiltinIdentifier::UnsignedFromBits
    }

    pub fn arguments_count(&self) -> usize {
        1
    }

    pub fn simulate(&self, input: &Type) -> Option<Type> {
        match input {
            Type::Array { r#type, size } => match (r#type.deref(), *size) {
                (Type::Boolean, crate::BITLENGTH_BOOLEAN) => Some(Type::new_boolean()),
                (Type::Boolean, crate::BITLENGTH_FIELD_PADDED) => Some(Type::new_field()),
                (Type::Boolean, size)
                    if size < crate::BITLENGTH_FIELD_PADDED
                        && size % crate::BITLENGTH_BYTE == 0 =>
                {
                    Some(Type::new_integer_unsigned(size))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for FromBitsStandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn std::{}(bits: [bool: N]) -> field", self.identifier,)
    }
}
