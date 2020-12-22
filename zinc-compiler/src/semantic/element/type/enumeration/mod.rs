//!
//! The semantic analyzer enumeration type element.
//!

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

use num::BigInt;

use zinc_lexical::Location;
use zinc_syntax::Variant;

use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// Describes an enumeration type.
///
/// Consists of the local enumeration `identifier` within its scope, global `type_id`,
/// and the implementation `scope`, which contains the enumeration variants and
/// reference to its parent scope.
///
#[derive(Debug, Clone)]
pub struct Enumeration {
    /// The enumeration type location in the code.
    pub location: Option<Location>,
    /// The enumeration type identifier.
    pub identifier: String,
    /// The unique enumeration type ID.
    pub type_id: usize,
    /// The enumeration elements bitlength, enough to fit the largest variant.
    pub bitlength: usize,
    /// The ordered list of the variant names.
    pub names: Vec<String>,
    /// The ordered list of the variant values.
    pub values: Vec<BigInt>,
    /// The ordered list of the enumeration generic type arguments.
    pub generics: Vec<String>,
    /// The enumeration scope, where its methods and associated items are declared.
    pub scope: Rc<RefCell<Scope>>,
}

impl Enumeration {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        identifier: String,
        type_id: usize,
        variants: Vec<Variant>,
        generics: Vec<String>,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Self, Error> {
        let mut variants_bigint = Vec::with_capacity(variants.len());
        for variant in variants.iter() {
            let value = IntegerConstant::try_from(&variant.literal)?;
            variants_bigint.push((variant.identifier.to_owned(), value.value.to_owned()));
        }
        let names: Vec<String> = variants_bigint
            .iter()
            .map(|(identifier, _value)| identifier.name.to_owned())
            .collect();
        let bigints: Vec<BigInt> = variants_bigint
            .iter()
            .map(|(_identifier, value)| value.to_owned())
            .collect();
        for (index, bigint) in bigints.iter().enumerate() {
            if bigints.iter().filter(|value| value == &bigint).count() > 1 {
                let variant = variants
                    .get(index)
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

                return Err(Error::TypeDuplicateVariantValue {
                    location: variant.identifier.location,
                    r#type: identifier,
                    variant_name: variant.identifier.name.to_owned(),
                    variant_value: bigint.to_owned(),
                });
            }
        }

        let minimal_bitlength = IntegerConstant::minimal_bitlength_bigints(
            bigints.iter().collect::<Vec<&BigInt>>().as_slice(),
            false,
            location,
        )?;

        let enumeration = Self {
            location: Some(location),
            identifier,
            type_id,
            bitlength: minimal_bitlength,
            names,
            values: bigints,
            generics,
            scope: scope.clone(),
        };

        for (identifier, value) in variants_bigint.into_iter() {
            let identifier_location = identifier.location;

            let mut constant =
                IntegerConstant::new(identifier_location, value, false, minimal_bitlength, false);

            constant.set_enumeration(enumeration.clone());

            Scope::define_variant(scope.clone(), identifier, Constant::Integer(constant))?;
        }

        Ok(enumeration)
    }
}

impl PartialEq<Self> for Enumeration {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl fmt::Display for Enumeration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
