//!
//! The semantic analyzer standard library `from_bits_signed` function type element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::StandardLibraryFunctionError;
use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
pub struct FromBitsSignedStandardLibraryFunction {
    identifier: &'static str,
}

impl FromBitsSignedStandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "from_bits_signed",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::SignedFromBits
    }

    pub fn arguments_count(&self) -> usize {
        1
    }

    pub fn validate(&self, inputs: &[Type]) -> Result<Type, StandardLibraryFunctionError> {
        match inputs.get(0) {
            Some(Type::Array { r#type, size }) => match (r#type.deref(), *size) {
                (Type::Boolean, size)
                    if crate::BITLENGTH_BYTE <= size
                        && size <= crate::BITLENGTH_MAX_INT
                        && size % crate::BITLENGTH_BYTE == 0 =>
                {
                    Ok(Type::new_integer_signed(size))
                }
                (r#type, size) => Err(StandardLibraryFunctionError::ArgumentType(
                    self.identifier,
                    "[bool; {{N}}]".to_owned(),
                    format!("[{}; {}]", r#type, size),
                )),
            },
            Some(r#type) => Err(StandardLibraryFunctionError::ArgumentType(
                self.identifier,
                "[bool; {{N}}]".to_owned(),
                r#type.to_string(),
            )),
            None => Err(StandardLibraryFunctionError::ArgumentCount(
                self.identifier,
                self.arguments_count(),
                inputs.len(),
            )),
        }
    }
}

impl fmt::Display for FromBitsSignedStandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::{}(bits: [bool; {{N}}]) -> i{{N}}",
            self.identifier
        )
    }
}
