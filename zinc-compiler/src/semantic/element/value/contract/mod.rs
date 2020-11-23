//!
//! The semantic analyzer contract value element.
//!

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_syntax::Identifier;

use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::r#type::contract::Contract as ContractType;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::structure::Structure as StructureValue;
use crate::semantic::element::value::Value;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// Contracts are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    /// The location of the contract expression, which start from the `{` left curly bracket.
    pub location: Option<Location>,
    /// The ordered contract fields array. Location is `None` if fields are not pushed separately.
    pub fields: Vec<(String, Option<Location>, Type)>,
    /// The contract type, which is set for values validation.
    pub r#type: Option<ContractType>,
}

impl Contract {
    ///
    /// A shortcut constructor, which is called when the contract type is already known.
    ///
    pub fn new_with_type(location: Option<Location>, r#type: ContractType) -> Self {
        Self {
            location,
            fields: r#type
                .fields
                .clone()
                .into_iter()
                .map(|field| (field.identifier.name, None, field.r#type))
                .collect(),
            r#type: Some(r#type),
        }
    }

    ///
    /// Converts the contract value into a structure one, transferring all the fields one-by-one.
    ///
    pub fn from_structure(structure: StructureValue, scope: Rc<RefCell<Scope>>) -> Self {
        let mut fields = Vec::with_capacity(
            zinc_const::contract::IMPLICIT_FIELDS_COUNT + structure.fields.len(),
        );
        fields.push((
            zinc_const::contract::FIELD_NAME_ADDRESS.to_owned(),
            None,
            Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS),
        ));
        fields.push((
            zinc_const::contract::FIELD_NAME_BALANCES.to_owned(),
            None,
            Scope::resolve_mtreemap(
                structure
                    .location
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                scope,
            ),
        ));
        fields.extend(structure.fields);

        Self {
            location: structure.location,
            fields,
            r#type: None,
        }
    }

    ///
    /// Sets the contract type and checks if the pushed field types match it.
    ///
    pub fn validate(&mut self, expected: ContractType) -> Result<(), Error> {
        if self.fields.len() < expected.fields.len() {
            return Err(Error::StructureFieldCount {
                location: self.location.unwrap_or(expected.location),
                r#type: expected.identifier.to_owned(),
                expected: expected.fields.len() - zinc_const::contract::IMPLICIT_FIELDS_COUNT,
                found: self.fields.len() - zinc_const::contract::IMPLICIT_FIELDS_COUNT,
            });
        }

        for (index, (name, location, r#type)) in self.fields.iter().enumerate() {
            match expected.fields.get(index) {
                Some(expected_field) => {
                    if name != &expected_field.identifier.name {
                        return Err(Error::StructureFieldExpected {
                            location: location
                                .unwrap_or_else(|| expected_field.identifier.location),
                            r#type: expected.identifier.to_owned(),
                            position: index + 1 - zinc_const::contract::IMPLICIT_FIELDS_COUNT,
                            expected: expected_field.identifier.name.to_owned(),
                            found: name.to_owned(),
                        });
                    }

                    if r#type != &expected_field.r#type {
                        return Err(Error::StructureFieldInvalidType {
                            location: location
                                .unwrap_or_else(|| expected_field.identifier.location),
                            r#type: expected.identifier.to_owned(),
                            field_name: expected_field.identifier.name.to_owned(),
                            expected: expected_field.r#type.to_string(),
                            found: r#type.to_string(),
                        });
                    }
                }
                None => {
                    return Err(Error::StructureFieldCount {
                        location: location.unwrap_or_else(|| expected.location),
                        r#type: expected.identifier.to_owned(),
                        expected: expected.fields.len()
                            - zinc_const::contract::IMPLICIT_FIELDS_COUNT,
                        found: index + 1 - zinc_const::contract::IMPLICIT_FIELDS_COUNT,
                    });
                }
            }
        }

        self.r#type = Some(expected);

        Ok(())
    }

    ///
    /// Slices the contract storage, returning the specified field.
    ///
    pub fn slice(self, expected: Identifier) -> Result<(Value, ContractFieldAccess), Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        for (index, (name, _location, r#type)) in self.fields.iter().enumerate() {
            if name == expected.name.as_str() {
                let access = ContractFieldAccess::new(
                    expected.name,
                    index,
                    offset,
                    r#type.size(),
                    total_size,
                    false,
                    r#type.is_mtreemap(),
                );

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

impl ITyped for Contract {
    fn r#type(&self) -> Type {
        self.r#type
            .clone()
            .map(Type::Contract)
            .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type == other.r#type
    }
}

impl fmt::Display for Contract {
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
