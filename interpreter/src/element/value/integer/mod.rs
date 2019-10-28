//!
//! The interpreter element integer value.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use parser::IntegerLiteral;
use parser::TypeVariant;
use r1cs::AllocatedNum;
use r1cs::Bn256;
use r1cs::ConstraintSystem;

use crate::element::Boolean;

#[derive(Clone)]
pub struct Integer {
    pub number: AllocatedNum<Bn256>,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Integer {
    pub fn new_from_usize<S: ConstraintSystem<Bn256>>(
        mut system: S,
        value: usize,
        bitlength: usize,
    ) -> Result<Self, Error> {
        let number = r1cs::allocate_number(
            system.namespace(|| "integer_new_from_usize"),
            value.to_string().as_str(),
        )
        .map_err(|error| Error::InnerAllocation(error.to_string()))?;

        Ok(Self {
            number,
            is_signed: false,
            bitlength,
        })
    }

    pub fn new_from_literal<S: ConstraintSystem<Bn256>>(
        mut system: S,
        literal: IntegerLiteral,
        bitlength: Option<usize>,
    ) -> Result<Self, Error> {
        let (number, inferred_bitlength) =
            semantic::infer_integer_literal(&literal).map_err(Error::Inference)?;
        let bitlength = bitlength.unwrap_or(inferred_bitlength);

        let number = r1cs::allocate_number(
            system.namespace(|| "integer_new_from_literal"),
            number.as_str(),
        )
        .map_err(|error| Error::InnerAllocation(error.to_string()))?;

        Ok(Self {
            number,
            is_signed: false,
            bitlength,
        })
    }

    pub fn type_variant(&self) -> TypeVariant {
        match (self.is_signed, self.bitlength) {
            (false, semantic::BITLENGTH_FIELD) => TypeVariant::new_field(),
            (is_signed, bitlength) if bitlength < semantic::BITLENGTH_FIELD => {
                TypeVariant::new_integer(is_signed, bitlength)
            }
            (..) => panic!("Always checked by the branches above"),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed && self.bitlength == other.bitlength
    }

    pub fn equals<S: ConstraintSystem<Bn256>>(
        &self,
        mut system: S,
        other: &Self,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        r1cs::equals_number(
            system.namespace(|| "integer_equals"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map(Boolean)
        .map_err(|error| Error::InnerOperation("equals", error.to_string()))
    }

    pub fn not_equals<S: ConstraintSystem<Bn256>>(
        &self,
        mut system: S,
        other: &Self,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        r1cs::not_equals_number(
            system.namespace(|| "integer_not_equals"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map(Boolean)
        .map_err(|error| Error::InnerOperation("not_equals", error.to_string()))
    }

    pub fn greater_equals<S: ConstraintSystem<Bn256>>(
        &self,
        mut system: S,
        other: &Self,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        r1cs::greater_equals(
            system.namespace(|| "integer_greater_equals"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map(Boolean)
        .map_err(|error| Error::InnerOperation("greater_equals", error.to_string()))
    }

    pub fn lesser_equals<S: ConstraintSystem<Bn256>>(
        &self,
        mut system: S,
        other: &Self,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        r1cs::lesser_equals(
            system.namespace(|| "integer_lesser_equals"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map(Boolean)
        .map_err(|error| Error::InnerOperation("lesser_equals", error.to_string()))
    }

    pub fn greater<S: ConstraintSystem<Bn256>>(
        &self,
        mut system: S,
        other: &Self,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        r1cs::greater(
            system.namespace(|| "integer_greater"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map(Boolean)
        .map_err(|error| Error::InnerOperation("greater", error.to_string()))
    }

    pub fn lesser<S: ConstraintSystem<Bn256>>(
        &self,
        mut system: S,
        other: &Self,
    ) -> Result<Boolean, Error> {
        if !self.has_the_same_type_as(other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        r1cs::lesser(
            system.namespace(|| "integer_lesser"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map(Boolean)
        .map_err(|error| Error::InnerOperation("lesser", error.to_string()))
    }

    pub fn add<S: ConstraintSystem<Bn256>>(
        mut self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        self.number = r1cs::add(
            system.namespace(|| "integer_add"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::InnerOperation("add", error.to_string()))?
        .0;

        Ok(self)
    }

    pub fn subtract<S: ConstraintSystem<Bn256>>(
        mut self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        self.number = r1cs::subtract(
            system.namespace(|| "integer_subtract"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::InnerOperation("subtract", error.to_string()))?
        .0;

        Ok(self)
    }

    pub fn multiply<S: ConstraintSystem<Bn256>>(
        mut self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        self.number = r1cs::multiply(
            system.namespace(|| "integer_multiply"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::InnerOperation("multiply", error.to_string()))?
        .0;

        Ok(self)
    }

    pub fn divide<S: ConstraintSystem<Bn256>>(
        mut self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        self.number = r1cs::divide(
            system.namespace(|| "integer_divide"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::InnerOperation("divide", error.to_string()))?
        .0;

        Ok(self)
    }

    pub fn modulo<S: ConstraintSystem<Bn256>>(
        mut self,
        mut system: S,
        other: Self,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        self.number = r1cs::modulo(
            system.namespace(|| "integer_modulo"),
            &self.number,
            &other.number,
            self.bitlength,
        )
        .map_err(|error| Error::InnerOperation("modulo", error.to_string()))?
        .0;

        Ok(self)
    }

    pub fn negate<S: ConstraintSystem<Bn256>>(mut self, mut system: S) -> Result<Self, Error> {
        self.number = r1cs::negate(
            system.namespace(|| "integer_negate"),
            &self.number,
            self.bitlength,
        )
        .map_err(|error| Error::InnerOperation("negate", error.to_string()))?
        .0;
        self.is_signed = true;

        Ok(self)
    }

    pub fn cast<S: ConstraintSystem<Bn256>>(
        mut self,
        mut system: S,
        to: TypeVariant,
    ) -> Result<Self, Error> {
        let from = self.type_variant();
        semantic::validate_casting(&from, &to).map_err(Error::Casting)?;
        let (is_signed, bitlength) = match to {
            TypeVariant::IntegerUnsigned { bitlength } => (false, bitlength),
            TypeVariant::IntegerSigned { bitlength } => (true, bitlength),
            TypeVariant::Field => (false, semantic::BITLENGTH_FIELD),
            _ => panic!("Always checked by some branches above"),
        };

        self.number = r1cs::cast(
            system.namespace(|| "integer_cast"),
            &self.number,
            self.bitlength,
        )
        .map_err(|error| Error::InnerOperation("cast", error.to_string()))?;
        self.is_signed = is_signed;
        self.bitlength = bitlength;

        Ok(self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self
            .number
            .get_value()
            .expect("Always returns a value")
            .to_string();
        let value = value[5..value.len() - 1].trim_start_matches('0');
        write!(f, "{}", if value.is_empty() { "0" } else { value })
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
        self.fmt(f)
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
