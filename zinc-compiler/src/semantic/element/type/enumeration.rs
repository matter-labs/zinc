//!
//! The semantic analyzer enumeration type element.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

use num_bigint::BigInt;

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::variant::Variant;

///
/// Describes an enumeration type.
///
/// Consists of the local enumeration `identifier` within its scope, global `unique_id`,
/// and the implementation `scope`, which contains the enumeration variants and
/// reference to its parent scope.
///
#[derive(Debug, Clone)]
pub struct Enumeration {
    pub identifier: String,
    pub unique_id: usize,
    pub bitlength: usize,
    pub scope: Rc<RefCell<Scope>>,
}

impl Enumeration {
    pub fn new(
        identifier: Identifier,
        unique_id: usize,
        variants: Vec<Variant>,
        scope_parent: Option<Rc<RefCell<Scope>>>,
    ) -> Result<Self, Error> {
        let scope = Rc::new(RefCell::new(Scope::new(scope_parent)));

        let mut variants_bigint = Vec::with_capacity(variants.len());
        for variant in variants.into_iter() {
            let value = IntegerConstant::try_from(&variant.literal).map_err(|error| {
                Error::Element(
                    variant.identifier.location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?;
            variants_bigint.push((variant.identifier, value.value));
        }
        let bigints: Vec<&BigInt> = variants_bigint.iter().map(|variant| &variant.1).collect();
        let minimal_bitlength =
            IntegerConstant::minimal_bitlength_bigints(bigints.as_slice(), false).map_err(
                |error| {
                    Error::Element(
                        identifier.location,
                        ElementError::Constant(ConstantError::Integer(error)),
                    )
                },
            )?;

        let enumeration = Self {
            identifier: identifier.name,
            unique_id,
            bitlength: minimal_bitlength,
            scope: scope.clone(),
        };

        for (identifier, value) in variants_bigint.into_iter() {
            let location = identifier.location;
            let mut constant = IntegerConstant::new(value, false, minimal_bitlength);
            constant.set_enumeration(enumeration.clone());
            Scope::declare_constant(scope.clone(), identifier, Constant::Integer(constant))
                .map_err(|error| Error::Scope(location, error))?;
        }

        scope
            .borrow_mut()
            .declare_self(Type::Enumeration(enumeration.clone()));

        Ok(enumeration)
    }
}

impl PartialEq<Self> for Enumeration {
    fn eq(&self, other: &Self) -> bool {
        self.unique_id == other.unique_id
    }
}

impl fmt::Display for Enumeration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "enum {}", self.identifier)
    }
}
