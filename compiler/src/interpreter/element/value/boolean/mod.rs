//!
//! The interpreter integer value.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use r1cs::Bn256;
use r1cs::ConstraintSystem;

use crate::lexical::BooleanLiteral;

#[derive(Clone)]
pub struct Boolean(pub r1cs::Boolean);

impl Boolean {
    pub fn new_from_bool<CS: ConstraintSystem<Bn256>>(
        mut system: CS,
        value: bool,
    ) -> Result<Self, Error> {
        let value = r1cs::allocate_boolean(system.namespace(|| "boolean_new_from_bool"), value)
            .map_err(|error| Error::InnerAllocation(error.to_string()))?;

        Ok(Self(value))
    }

    pub fn new_from_literal<CS: ConstraintSystem<Bn256>>(
        mut system: CS,
        literal: BooleanLiteral,
    ) -> Result<Self, Error> {
        let value: bool = literal.into();
        let value = r1cs::allocate_boolean(system.namespace(|| "boolean_new_from_literal"), value)
            .map_err(|error| Error::InnerAllocation(error.to_string()))?;

        Ok(Self(value))
    }

    pub fn is_false(&self) -> bool {
        !self.0.get_value().expect("Always returns a value")
    }

    pub fn is_true(&self) -> bool {
        self.0.get_value().expect("Always returns a value")
    }

    pub fn or<CS: ConstraintSystem<Bn256>>(
        self,
        mut system: CS,
        other: Self,
    ) -> Result<Self, Error> {
        r1cs::or(system.namespace(|| "boolean_or"), &self.0, &other.0)
            .map_err(|error| Error::InnerOperation("or", error.to_string()))
            .map(Self)
    }

    pub fn xor<CS: ConstraintSystem<Bn256>>(
        self,
        mut system: CS,
        other: Self,
    ) -> Result<Self, Error> {
        r1cs::xor(system.namespace(|| "boolean_xor"), &self.0, &other.0)
            .map_err(|error| Error::InnerOperation("xor", error.to_string()))
            .map(Self)
    }

    pub fn and<CS: ConstraintSystem<Bn256>>(
        self,
        mut system: CS,
        other: Self,
    ) -> Result<Self, Error> {
        r1cs::and(system.namespace(|| "boolean_and"), &self.0, &other.0)
            .map_err(|error| Error::InnerOperation("and", error.to_string()))
            .map(Self)
    }

    pub fn equals<CS: ConstraintSystem<Bn256>>(
        &self,
        mut system: CS,
        other: &Self,
    ) -> Result<Boolean, Error> {
        r1cs::equals_boolean(system.namespace(|| "boolean_equals"), &self.0, &other.0)
            .map(Boolean)
            .map_err(|error| Error::InnerOperation("equals", error.to_string()))
    }

    pub fn not_equals<CS: ConstraintSystem<Bn256>>(
        &self,
        mut system: CS,
        other: &Self,
    ) -> Result<Boolean, Error> {
        r1cs::not_equals_boolean(system.namespace(|| "boolean_not_equals"), &self.0, &other.0)
            .map(Boolean)
            .map_err(|error| Error::InnerOperation("not_equals", error.to_string()))
    }

    pub fn not<CS: ConstraintSystem<Bn256>>(self, mut system: CS) -> Result<Self, Error> {
        r1cs::not(system.namespace(|| "boolean_not"), &self.0)
            .map_err(|error| Error::InnerOperation("not", error.to_string()))
            .map(Self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.get_value().expect("Always returns a value"))
    }
}

impl PartialEq<Self> for Boolean {
    fn eq(&self, other: &Self) -> bool {
        self.0.get_value() == other.0.get_value()
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
