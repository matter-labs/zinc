//!
//! The interpreter integer.
//!

use std::fmt;

use failure::Fail;

use bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;
use pairing::bn256::Bn256;

use crate::syntax::TypeVariant;

#[derive(Clone)]
pub struct Integer {
    pub number: AllocatedNum<Bn256>,
    pub is_signed: bool,
    pub bitlength: usize,
}

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "synthesis: {}", _0)]
    Synthesis(String),
    #[fail(
        display = "operand types mismatch: [{}] and [{}] are of different types",
        _0, _1
    )]
    OperandTypesMismatch(Integer, Integer),
    #[fail(display = "casting to invalid type: from [{}] to '{}'", _0, _1)]
    CastingToInvalidType(Integer, TypeVariant),
    #[fail(display = "casting to lesser bitlength: from {} to {}", _0, _1)]
    CastingToLesserOrEqualBitlength(usize, usize),
}

impl Integer {
    pub fn new(number: AllocatedNum<Bn256>, is_signed: bool, bitlength: usize) -> Self {
        Self {
            number,
            is_signed,
            bitlength,
        }
    }

    pub fn type_variant(&self) -> TypeVariant {
        match (self.is_signed, self.bitlength) {
            (false, 254) => TypeVariant::Field,
            (true, bitlength) => TypeVariant::Int { bitlength },
            (false, bitlength) => TypeVariant::Uint { bitlength },
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed && self.bitlength == other.bitlength
    }

    pub fn greater_equals<CS: ConstraintSystem<Bn256>>(
        &self,
        other: &Self,
        mut system: CS,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        Ok(jab::greater_equals(
            system.namespace(|| "lesser"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::Synthesis(error.to_string()))?)
    }

    pub fn lesser_equals<CS: ConstraintSystem<Bn256>>(
        &self,
        other: &Self,
        mut system: CS,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        Ok(jab::lesser_equals(
            system.namespace(|| "lesser"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::Synthesis(error.to_string()))?)
    }

    pub fn greater<CS: ConstraintSystem<Bn256>>(
        &self,
        other: &Self,
        mut system: CS,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        Ok(jab::greater(
            system.namespace(|| "lesser"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::Synthesis(error.to_string()))?)
    }

    pub fn lesser<CS: ConstraintSystem<Bn256>>(
        &self,
        other: &Self,
        mut system: CS,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        Ok(jab::lesser(
            system.namespace(|| "lesser"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::Synthesis(error.to_string()))?)
    }

    pub fn add<CS: ConstraintSystem<Bn256>>(
        mut self,
        other: Self,
        mut system: CS,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        self.number = jab::add(
            system.namespace(|| "add"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::Synthesis(error.to_string()))?
        .0;
        Ok(self)
    }

    pub fn subtract<CS: ConstraintSystem<Bn256>>(
        mut self,
        other: Self,
        mut system: CS,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        self.number = jab::subtract(
            system.namespace(|| "subtract"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::Synthesis(error.to_string()))?
        .0;
        Ok(self)
    }

    pub fn multiply<CS: ConstraintSystem<Bn256>>(
        mut self,
        other: Self,
        mut system: CS,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        self.number = jab::multiply(
            system.namespace(|| "multiply"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::Synthesis(error.to_string()))?
        .0;
        Ok(self)
    }

    pub fn divide<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        _system: CS,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        unimplemented!();
    }

    pub fn modulo<CS: ConstraintSystem<Bn256>>(
        self,
        other: Self,
        _system: CS,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.to_owned(),
                other.to_owned(),
            ));
        }

        unimplemented!();
    }

    pub fn negate<CS: ConstraintSystem<Bn256>>(mut self, mut system: CS) -> Result<Self, Error> {
        self.number = jab::negate(system.namespace(|| "negate"), &self.number, self.bitlength)
            .map_err(|error| Error::Synthesis(error.to_string()))?
            .0;
        self.is_signed = true;
        Ok(self)
    }

    pub fn cast<CS: ConstraintSystem<Bn256>>(
        mut self,
        type_variant: TypeVariant,
        mut system: CS,
    ) -> Result<Self, Error> {
        match (self.is_signed, self.bitlength, type_variant) {
            (false, b1, TypeVariant::Uint { bitlength: b2 }) if b1 >= 248 || b1 >= b2 => {
                Err(Error::CastingToLesserOrEqualBitlength(b1, b2))
            }
            (false, b1, TypeVariant::Int { bitlength: b2 }) if b1 >= 240 || b1 + 8 >= b2 => {
                Err(Error::CastingToLesserOrEqualBitlength(b1, b2))
            }
            (true, b1, TypeVariant::Int { bitlength: b2 }) if b1 >= 248 || b1 >= b2 => {
                Err(Error::CastingToLesserOrEqualBitlength(b1, b2))
            }
            (true, b1, TypeVariant::Uint { bitlength: b2 }) if b1 >= 248 || b1 >= b2 => {
                Err(Error::CastingToLesserOrEqualBitlength(b1, b2))
            }
            (false, b1, TypeVariant::Uint { bitlength: b2 }) if b1 >= 254 => {
                Err(Error::CastingToLesserOrEqualBitlength(b1, b2))
            }
            (false, b1, TypeVariant::Int { bitlength: b2 }) if b1 >= 254 => {
                Err(Error::CastingToLesserOrEqualBitlength(b1, b2))
            }
            (_, _, type_variant) => {
                let (is_signed, bitlength) = match type_variant {
                    TypeVariant::Uint { bitlength } => (false, bitlength),
                    TypeVariant::Int { bitlength } => (true, bitlength),
                    TypeVariant::Field => (false, 254),
                    type_variant => return Err(Error::CastingToInvalidType(self, type_variant)),
                };
                self.number = jab::cast(system.namespace(|| "cast"), &self.number, self.bitlength)
                    .map_err(|error| Error::Synthesis(error.to_string()))?;
                self.is_signed = is_signed;
                self.bitlength = bitlength;
                Ok(self)
            }
        }
    }

    pub fn inc<CS: ConstraintSystem<Bn256>>(mut self, mut system: CS) -> Result<Self, Error> {
        let one = jab::allocate_number(system.namespace(|| "one"), "1")
            .map_err(|error| Error::Synthesis(error.to_string()))?;
        self.number = jab::add(
            system.namespace(|| "inc"),
            &self.number,
            &one,
            self.bitlength,
        )
        .map_err(|error| Error::Synthesis(error.to_string()))?
        .0;
        Ok(self)
    }
}

impl PartialEq<Self> for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.number.get_value() == other.number.get_value()
            && self.is_signed == other.is_signed
            && self.bitlength == other.bitlength
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r#type = match (self.is_signed, self.bitlength) {
            (false, 254) => "field".to_owned(),
            (true, bitlength) => format!("int{}", bitlength),
            (false, bitlength) => format!("uint{}", bitlength),
        };
        write!(
            f,
            "{:?}: {}",
            self.number.get_value().expect("Always returns a value"),
            r#type
        )
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r#type = match (self.is_signed, self.bitlength) {
            (false, 254) => "field".to_owned(),
            (true, bitlength) => format!("int{}", bitlength),
            (false, bitlength) => format!("uint{}", bitlength),
        };
        write!(
            f,
            "{:?}: {}",
            self.number.get_value().expect("Always returns a value"),
            r#type
        )
    }
}
