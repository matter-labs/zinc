//!
//! The semantic analyzer structure value element.
//!

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_syntax::Identifier;

use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::contract::Contract as ContractValue;
use crate::semantic::element::value::Value;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// Structures are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    /// The location of the structure expression, which start from the `{` left curly bracket.
    pub location: Option<Location>,
    /// The ordered structure fields array. Location is `None` if fields are not pushed separately.
    pub fields: Vec<(String, Option<Location>, Type)>,
    /// The structure type, which is set for values validation.
    pub r#type: Option<StructureType>,
}

impl Structure {
    ///
    /// A shortcut constructor.
    ///
    /// The type is not set here, so the value must be `validate`d later.
    ///
    pub fn new(location: Option<Location>) -> Self {
        Self {
            location,
            fields: vec![],
            r#type: None,
        }
    }

    ///
    /// A shortcut constructor, which is called when the contract type is already known.
    ///
    pub fn new_with_type(location: Option<Location>, r#type: StructureType) -> Self {
        Self {
            location,
            fields: r#type
                .fields
                .clone()
                .into_iter()
                .map(|(name, r#type)| (name, None, r#type))
                .collect(),
            r#type: Some(r#type),
        }
    }

    ///
    /// Converts the structure value into a contract one, transferring all the fields one-by-one.
    ///
    pub fn into_contract(self, scope: Rc<RefCell<Scope>>) -> ContractValue {
        ContractValue::from_structure(self, scope)
    }

    ///
    /// Pushes a typed element into the structure fields array.
    ///
    pub fn push(&mut self, name: String, location: Option<Location>, r#type: Type) {
        self.fields.push((name, location, r#type));
    }

    ///
    /// Sets the structure type and checks if the pushed field types match it.
    ///
    pub fn validate(&mut self, expected: StructureType) -> Result<(), Error> {
        if self.fields.len() < expected.fields.len() {
            return Err(Error::StructureFieldCount {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                r#type: expected.identifier.to_owned(),
                expected: expected.fields.len(),
                found: self.fields.len(),
            });
        }

        for (index, (name, location, r#type)) in self.fields.iter().enumerate() {
            match expected.fields.get(index) {
                Some((expected_name, expected_type)) => {
                    if name != expected_name {
                        return Err(Error::StructureFieldExpected {
                            location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            r#type: expected.identifier.to_owned(),
                            position: index + 1,
                            expected: expected_name.to_owned(),
                            found: name.to_owned(),
                        });
                    }

                    if r#type != expected_type {
                        return Err(Error::StructureFieldInvalidType {
                            location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            r#type: expected.identifier.to_owned(),
                            field_name: expected_name.to_owned(),
                            expected: expected_type.to_string(),
                            found: r#type.to_string(),
                        });
                    }
                }
                None => {
                    return Err(Error::StructureFieldCount {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        r#type: expected.identifier.to_owned(),
                        expected: expected.fields.len(),
                        found: index + 1,
                    });
                }
            }
        }

        self.r#type = Some(expected);

        Ok(())
    }

    ///
    /// Slices the structure, returning the specified field.
    ///
    pub fn slice(self, expected: Identifier) -> Result<(Value, StackFieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        for (index, (name, _location, r#type)) in self.fields.iter().enumerate() {
            if name == expected.name.as_str() {
                let access =
                    StackFieldAccess::new(expected.name, index, offset, r#type.size(), total_size);

                let result = Value::try_from_type(r#type, false, self.location)
                    .expect(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS);

                return Ok((result, access));
            }
            offset += r#type.size();
        }

        Err(Error::StructureFieldDoesNotExist {
            location: expected.location,
            r#type: self
                .r#type
                .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                .identifier,
            field_name: expected.name,
        })
    }
}

impl ITyped for Structure {
    fn r#type(&self) -> Type {
        self.r#type
            .clone()
            .map(Type::Structure)
            .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<runtime> '{}' with fields {{ {} }}",
            self.r#type
                .as_ref()
                .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                .identifier,
            self.fields
                .iter()
                .map(|(name, _location, r#type)| format!("'{}' of type '{}'", name, r#type))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
