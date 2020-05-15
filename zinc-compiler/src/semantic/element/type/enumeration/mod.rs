//!
//! The semantic analyzer enumeration type element.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

use num_bigint::BigInt;

use crate::lexical::token::location::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::variant::Variant;

///
/// Describes an enumeration type.
///
/// Consists of the local enumeration `identifier` within its scope, global `type_id`,
/// and the implementation `scope`, which contains the enumeration variants and
/// reference to its parent scope.
///
#[derive(Debug, Clone)]
pub struct Enumeration {
    pub location: Option<Location>,
    pub identifier: String,
    pub type_id: usize,
    pub bitlength: usize,
    pub values: Vec<BigInt>,
    pub scope: Rc<RefCell<Scope>>,
}

impl Enumeration {
    pub fn new(
        location: Location,
        identifier: String,
        type_id: usize,
        variants: Vec<Variant>,
        scope: Option<Rc<RefCell<Scope>>>,
    ) -> Result<Self, Error> {
        let scope = scope.unwrap_or_else(|| Scope::new(None).wrap());

        let mut variants_bigint = Vec::with_capacity(variants.len());
        for variant in variants.into_iter() {
            let value = IntegerConstant::try_from(&variant.literal).map_err(|error| {
                Error::Element(ElementError::Constant(ConstantError::Integer(error)))
            })?;
            variants_bigint.push((variant.identifier, value.value.clone()));
        }
        let mut bigints: Vec<BigInt> = variants_bigint
            .iter()
            .map(|(_identifier, value)| value.to_owned())
            .collect();

        let minimal_bitlength = IntegerConstant::minimal_bitlength_bigints(
            bigints.iter().collect::<Vec<&BigInt>>().as_slice(),
            false,
            location,
        )
        .map_err(|error| Error::Element(ElementError::Constant(ConstantError::Integer(error))))?;

        bigints.sort();
        let mut enumeration = Self {
            location: Some(location),
            identifier,
            type_id,
            bitlength: minimal_bitlength,
            values: bigints,
            scope: scope.clone(),
        };

        for (identifier, value) in variants_bigint.into_iter() {
            let identifier_location = identifier.location;

            let mut constant =
                IntegerConstant::new(identifier_location, value, false, minimal_bitlength);

            constant.set_enumeration(enumeration.clone());

            Scope::define_constant(scope.clone(), identifier, Constant::Integer(constant))?;
        }

        scope
            .borrow_mut()
            .define_self(Type::Enumeration(enumeration.clone()));

        enumeration.values.sort();

        Ok(enumeration)
    }
}

impl PartialEq<Self> for Enumeration {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl fmt::Display for Enumeration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
