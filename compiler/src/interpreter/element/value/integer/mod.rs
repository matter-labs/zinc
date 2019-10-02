//!
//! The interpreter integer.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use num_bigint::BigInt;
use num_traits::Num;

use r1cs::AllocatedNum;
use r1cs::Bn256;
use r1cs::ConstraintSystem;

use crate::interpreter::Boolean;
use crate::lexical::IntegerLiteral;
use crate::syntax::TypeVariant;

#[derive(Clone)]
pub struct Integer {
    pub number: AllocatedNum<Bn256>,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Integer {
    pub fn new_from_usize<CS: ConstraintSystem<Bn256>>(
        mut system: CS,
        value: usize,
    ) -> Result<Self, Error> {
        let number = r1cs::allocate_number(
            system.namespace(|| "integer_new_from_usize"),
            value.to_string().as_str(),
        )
        .map_err(|error| Error::InnerAllocation(error.to_string()))?;

        Ok(Self {
            number,
            is_signed: false,
            bitlength: 64,
        })
    }

    pub fn new_from_bigint<CS: ConstraintSystem<Bn256>>(
        mut system: CS,
        value: BigInt,
        is_signed: bool,
        bitlength: usize,
    ) -> Result<Self, Error> {
        let number = r1cs::allocate_number(
            system.namespace(|| "integer_new_from_bigint"),
            value.to_string().as_str(),
        )
        .map_err(|error| Error::InnerAllocation(error.to_string()))?;

        Ok(Self {
            number,
            is_signed,
            bitlength,
        })
    }

    pub fn new_from_literal<CS: ConstraintSystem<Bn256>>(
        mut system: CS,
        literal: IntegerLiteral,
    ) -> Result<Self, Error> {
        let (string, base) = match literal {
            IntegerLiteral::Decimal { value } => (value, crate::BASE_DECIMAL as u32),
            IntegerLiteral::Hexadecimal { value } => (value, crate::BASE_HEXADECIMAL as u32),
        };

        let number = BigInt::from_str_radix(&string, base).expect("Always valid");
        let mut bitlength = crate::SIZE_BYTE;
        let mut exponent = BigInt::from(crate::MAX_BYTE);
        while number >= exponent {
            if bitlength == crate::SIZE_MAX_INT {
                exponent *= 64;
                bitlength += crate::SIZE_FIELD - crate::SIZE_MAX_INT;
            } else if bitlength == crate::SIZE_FIELD {
                return Err(Error::LiteralTooLarge(bitlength));
            } else {
                exponent *= crate::MAX_BYTE;
                bitlength += crate::SIZE_BYTE;
            }
        }

        let number = r1cs::allocate_number(
            system.namespace(|| "integer_new_from_literal"),
            number.to_string().as_str(),
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
            (false, crate::SIZE_FIELD) => TypeVariant::Field,
            (true, bitlength) if bitlength < crate::SIZE_FIELD => TypeVariant::Int { bitlength },
            (false, bitlength) if bitlength < crate::SIZE_FIELD => TypeVariant::Uint { bitlength },
            (_, bitlength) => panic!(
                "Always less than {}, but now it is {}",
                crate::SIZE_FIELD,
                bitlength,
            ),
        }
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.is_signed == other.is_signed && self.bitlength == other.bitlength
    }

    pub fn equals<CS: ConstraintSystem<Bn256>>(
        &self,
        mut system: CS,
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

    pub fn not_equals<CS: ConstraintSystem<Bn256>>(
        &self,
        mut system: CS,
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

    pub fn greater_equals<CS: ConstraintSystem<Bn256>>(
        &self,
        mut system: CS,
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

    pub fn lesser_equals<CS: ConstraintSystem<Bn256>>(
        &self,
        mut system: CS,
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

    pub fn greater<CS: ConstraintSystem<Bn256>>(
        &self,
        mut system: CS,
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

    pub fn lesser<CS: ConstraintSystem<Bn256>>(
        &self,
        mut system: CS,
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

    pub fn add<CS: ConstraintSystem<Bn256>>(
        mut self,
        mut system: CS,
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

    pub fn subtract<CS: ConstraintSystem<Bn256>>(
        mut self,
        mut system: CS,
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

    pub fn multiply<CS: ConstraintSystem<Bn256>>(
        mut self,
        mut system: CS,
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

    pub fn divide<CS: ConstraintSystem<Bn256>>(
        self,
        _system: CS,
        other: Self,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        unimplemented!();
    }

    pub fn modulo<CS: ConstraintSystem<Bn256>>(
        self,
        _system: CS,
        other: Self,
    ) -> Result<Self, Error> {
        if !self.has_the_same_type_as(&other) {
            return Err(Error::OperandTypesMismatch(
                self.type_variant(),
                other.type_variant(),
            ));
        }

        unimplemented!();
    }

    pub fn negate<CS: ConstraintSystem<Bn256>>(mut self, mut system: CS) -> Result<Self, Error> {
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

    pub fn cast<CS: ConstraintSystem<Bn256>>(
        mut self,
        mut system: CS,
        type_variant: TypeVariant,
    ) -> Result<Self, Error> {
        let type_variant = match (self.is_signed, self.bitlength, type_variant) {
            (false, b1, TypeVariant::Uint { bitlength: b2 })
                if b1 >= crate::SIZE_FIELD_PADDED - crate::SIZE_BYTE || b1 >= b2 =>
            {
                return Err(Error::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (false, b1, TypeVariant::Int { bitlength: b2 })
                if b1 >= crate::SIZE_FIELD_PADDED - crate::SIZE_BYTE * 2
                    || b1 + crate::SIZE_BYTE >= b2 =>
            {
                return Err(Error::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (true, b1, TypeVariant::Int { bitlength: b2 })
                if b1 >= crate::SIZE_FIELD_PADDED - crate::SIZE_BYTE || b1 >= b2 =>
            {
                return Err(Error::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (true, b1, TypeVariant::Uint { bitlength: b2 })
                if b1 >= crate::SIZE_FIELD_PADDED - crate::SIZE_BYTE || b1 >= b2 =>
            {
                return Err(Error::CastingToLesserOrEqualBitlength(b1, b2));
            }
            (_, _, type_variant) => type_variant,
        };

        let (is_signed, bitlength) = match type_variant {
            TypeVariant::Uint { bitlength } => (false, bitlength),
            TypeVariant::Int { bitlength } => (true, bitlength),
            TypeVariant::Field => (false, crate::SIZE_FIELD),
            type_variant => return Err(Error::CastingToInvalidType(self, type_variant)),
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
