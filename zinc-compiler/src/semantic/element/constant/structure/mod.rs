//!
//! The semantic analyzer constant structure element.
//!

#[cfg(test)]
mod tests;

use std::fmt;

use zinc_lexical::Location;
use zinc_syntax::Identifier;

use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;

///
/// Structures are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    /// The location of the structure expression, which start from the `{` left curly bracket.
    pub location: Location,
    /// The structure type, which is set for values validation.
    pub r#type: Option<StructureType>,
    /// The ordered structure values array.
    pub values: Vec<(Identifier, Constant)>,
}

impl Structure {
    ///
    /// A shortcut constructor.
    ///
    /// The type is not set here, so the value must be `validate`d later.
    ///
    pub fn new(location: Location) -> Self {
        Self {
            location,
            r#type: None,
            values: vec![],
        }
    }

    ///
    /// Pushes a typed element into the structure fields array.
    ///
    pub fn push(&mut self, identifier: Identifier, value: Constant) {
        self.values.push((identifier, value));
    }

    ///
    /// Sets the structure type and checks if the pushed field types match it.
    ///
    pub fn validate(&mut self, expected: StructureType) -> Result<(), Error> {
        if self.values.len() < expected.fields.len() {
            return Err(Error::StructureFieldCount {
                location: self.location,
                r#type: expected.identifier.to_owned(),
                expected: expected.fields.len(),
                found: self.values.len(),
            });
        }

        for (index, (identifier, constant)) in self.values.iter().enumerate() {
            match expected.fields.get(index) {
                Some((expected_name, expected_type)) => {
                    if &identifier.name != expected_name {
                        return Err(Error::StructureFieldExpected {
                            location: identifier.location,
                            r#type: expected.identifier.to_owned(),
                            position: index + 1,
                            expected: expected_name.to_owned(),
                            found: identifier.name.to_owned(),
                        });
                    }

                    let r#type = constant.r#type();
                    if &r#type != expected_type {
                        return Err(Error::StructureFieldInvalidType {
                            location: constant.location(),
                            r#type: expected.identifier.to_owned(),
                            field_name: expected_name.to_owned(),
                            expected: expected_type.to_string(),
                            found: r#type.to_string(),
                        });
                    }
                }
                None => {
                    return Err(Error::StructureFieldCount {
                        location: identifier.location,
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
    pub fn slice(self, identifier: Identifier) -> Result<(Constant, StackFieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        for (index, (name, value)) in self.values.into_iter().enumerate() {
            let element_size = value.r#type().size();

            if name.name == identifier.name {
                let access =
                    StackFieldAccess::new(name.name, index, offset, element_size, total_size);

                return Ok((value, access));
            }

            offset += element_size;
        }

        Err(Error::StructureFieldDoesNotExist {
            location: identifier.location,
            r#type: self
                .r#type
                .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                .identifier,
            field_name: identifier.name,
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
            "'{}' with fields {{ {} }}",
            self.r#type
                .as_ref()
                .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                .identifier,
            self.values
                .iter()
                .map(|(identifier, value)| format!("{}: {}", identifier.name, value))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
