//!
//! The Zinc VM template value.
//!

pub mod error;
pub mod scalar;

use std::collections::HashSet;

use num_bigint::BigInt;
use num_traits::Num;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use crate::data::r#type::scalar::integer::Type as IntegerType;
use crate::data::r#type::scalar::Type as ScalarType;
use crate::data::r#type::Type as DataType;

use self::error::Error;
use self::error::ErrorContext;
use self::error::ErrorType;
use self::scalar::Value as ScalarValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Unit,
    Scalar(ScalarValue),
    Array(Vec<Value>),
    Structure(Vec<(String, Value)>),
}

impl Value {
    pub fn new(r#type: DataType) -> Self {
        match r#type {
            DataType::Unit => Self::Unit,
            DataType::Scalar(scalar_type) => match scalar_type {
                ScalarType::Boolean => Self::Scalar(ScalarValue::Boolean(false)),
                ScalarType::Integer(inner) => Self::Scalar(ScalarValue::Integer(0.into(), inner)),
                ScalarType::Field => Self::Scalar(ScalarValue::Field(0.into())),
            },
            DataType::Enumeration => Self::Scalar(ScalarValue::Field(0.into())),

            DataType::Array(r#type, size) => Self::Array(vec![Self::new(*r#type); size]),
            DataType::Tuple(fields) => Self::Array(fields.into_iter().map(Self::new).collect()),
            DataType::Structure(fields) => Self::Structure(
                fields
                    .into_iter()
                    .map(|(name, r#type)| (name, Self::new(r#type)))
                    .collect(),
            ),
        }
    }

    ///
    /// Creates value from flat array and data type.
    ///
    pub fn new_from_flat_values(r#type: DataType, flat_values: &[BigInt]) -> Option<Self> {
        let mut value = Self::new(r#type);
        let consumed = value.fill_from_flat_values(flat_values)?;
        if consumed == flat_values.len() {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_flat_values(self) -> Vec<BigInt> {
        match self {
            Self::Unit => vec![],
            Self::Scalar(value) => vec![value.to_bigint()],
            Self::Array(values) => values
                .into_iter()
                .map(Self::into_flat_values)
                .flatten()
                .collect(),
            Self::Structure(fields) => fields
                .into_iter()
                .map(|(_name, value)| Self::into_flat_values(value))
                .flatten()
                .collect(),
        }
    }

    pub fn into_json(self) -> JsonValue {
        match self {
            Self::Unit => JsonValue::Null,
            Self::Scalar(scalar) => match scalar {
                ScalarValue::Field(value) => {
                    if value <= BigInt::from(std::u64::MAX) {
                        JsonValue::String(value.to_str_radix(zinc_const::BASE_DECIMAL as u32))
                    } else {
                        JsonValue::String(
                            String::from("0x")
                                + value
                                    .to_str_radix(zinc_const::BASE_HEXADECIMAL as u32)
                                    .as_str(),
                        )
                    }
                }
                ScalarValue::Integer(value, r#type) => {
                    if value <= BigInt::from(std::u64::MAX) || r#type.is_signed {
                        JsonValue::String(value.to_str_radix(zinc_const::BASE_DECIMAL as u32))
                    } else {
                        JsonValue::String(
                            String::from("0x")
                                + value
                                    .to_str_radix(zinc_const::BASE_HEXADECIMAL as u32)
                                    .as_str(),
                        )
                    }
                }
                ScalarValue::Boolean(value) => JsonValue::Bool(value),
            },
            Self::Array(values) => {
                JsonValue::Array(values.into_iter().map(Self::into_json).collect())
            }
            Self::Structure(fields) => {
                let mut object = JsonMap::<String, JsonValue>::with_capacity(fields.len());
                for (name, value) in fields.into_iter() {
                    object.insert(name, Self::into_json(value));
                }
                JsonValue::Object(object)
            }
        }
    }

    pub fn from_typed_json(value: JsonValue, r#type: DataType) -> Result<Self, Error> {
        match r#type {
            DataType::Unit => Self::unit_from_json(value),
            DataType::Scalar(inner) => Self::scalar_from_json(value, inner),
            DataType::Enumeration => Self::field_from_json(value),

            DataType::Array(inner, size) => Self::array_from_json(value, *inner, size),
            DataType::Tuple(inner) => Self::tuple_from_json(value, inner),
            DataType::Structure(fields) => Self::structure_from_json(value, fields),
        }
    }

    ///
    /// Fills values from slice, returns number of used values or None if there is not enough.
    ///
    fn fill_from_flat_values(&mut self, flat_values: &[BigInt]) -> Option<usize> {
        match self {
            Self::Unit => Some(0),
            Self::Scalar(scalar) => {
                match scalar {
                    ScalarValue::Field(value) | ScalarValue::Integer(value, _) => {
                        *value = flat_values.first()?.clone();
                    }
                    ScalarValue::Boolean(value) => {
                        *value = flat_values.first()? != &BigInt::from(0);
                    }
                }
                Some(1)
            }
            Self::Array(values) => {
                let mut offset = 0;
                for value in values.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            }
            Self::Structure(fields) => {
                let mut offset = 0;
                for (_name, value) in fields.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            }
        }
    }

    fn unit_from_json(value: JsonValue) -> Result<Self, Error> {
        if value.is_null() {
            return Ok(Self::Unit);
        }

        Err(ErrorType::TypeError {
            expected: "null".into(),
            actual: value.to_string(),
        }
        .into())
    }

    fn boolean_from_json(value: JsonValue) -> Result<Self, Error> {
        let value_bool = value.as_bool().ok_or_else(|| ErrorType::TypeError {
            expected: "boolean (true or false)".into(),
            actual: value.to_string(),
        })?;

        Ok(Self::Scalar(ScalarValue::Boolean(value_bool)))
    }

    fn integer_from_json(value: JsonValue, _type: IntegerType) -> Result<Self, Error> {
        // TODO: overflow check

        Self::field_from_json(value)
    }

    fn field_from_json(value: JsonValue) -> Result<Self, Error> {
        let value_string = value.as_str().ok_or_else(|| ErrorType::TypeError {
            expected: "field (number string)".into(),
            actual: value.to_string(),
        })?;

        let bigint_result = if value_string.starts_with("0x") {
            BigInt::from_str_radix(&value_string[2..], 16)
        } else {
            BigInt::from_str_radix(value_string, 10)
        };

        let bigint =
            bigint_result.map_err(|_| ErrorType::InvalidNumberFormat(value_string.into()))?;

        // TODO: overflow check

        Ok(Self::Scalar(ScalarValue::Field(bigint)))
    }

    fn scalar_from_json(value: JsonValue, scalar_type: ScalarType) -> Result<Self, Error> {
        match scalar_type {
            ScalarType::Boolean => Self::boolean_from_json(value),
            ScalarType::Integer(inner) => Self::integer_from_json(value, inner),
            ScalarType::Field => Self::field_from_json(value),
        }
    }

    fn array_from_json(value: JsonValue, r#type: DataType, size: usize) -> Result<Self, Error> {
        let array = value
            .as_array()
            .cloned()
            .ok_or_else(|| ErrorType::type_error("array", value))?;

        if array.len() != size {
            return Err(ErrorType::UnexpectedSize {
                expected: size,
                actual: array.len(),
            }
            .into());
        }

        let mut values = Vec::with_capacity(size);
        for (index, value) in array.into_iter().enumerate() {
            let typed_value = Self::from_typed_json(value, r#type.clone()).in_array(index)?;

            values.push(typed_value);
        }

        Ok(Self::Array(values))
    }

    fn tuple_from_json(value: JsonValue, types: Vec<DataType>) -> Result<Self, Error> {
        let array = value
            .as_array()
            .cloned()
            .ok_or_else(|| ErrorType::type_error("tuple (json array)", value))?;

        if array.len() != types.len() {
            return Err(ErrorType::UnexpectedSize {
                expected: types.len(),
                actual: array.len(),
            }
            .into());
        }

        let mut values = Vec::with_capacity(types.len());
        for (index, (value, r#type)) in array.into_iter().zip(types).enumerate() {
            let typed_value = Self::from_typed_json(value, r#type).in_array(index)?;
            values.push(typed_value);
        }

        Ok(Self::Array(values))
    }

    fn structure_from_json(
        value: JsonValue,
        field_types: Vec<(String, DataType)>,
    ) -> Result<Self, Error> {
        let mut object = value
            .as_object()
            .cloned()
            .ok_or_else(|| ErrorType::type_error("structure", value))?;

        let mut used_fields = HashSet::with_capacity(field_types.len());
        let mut field_values = Vec::with_capacity(field_types.len());
        for (name, r#type) in field_types.into_iter() {
            used_fields.insert(name.clone());

            let json_value = object
                .remove(name.as_str())
                .ok_or_else(|| ErrorType::MissingField(name.clone()))?;

            let value = Self::from_typed_json(json_value, r#type).in_struct(name.as_str())?;

            field_values.push((name, value));
        }

        for field in object.keys() {
            if !used_fields.contains(field.as_str()) {
                return Err(ErrorType::UnexpectedField(field.clone()).into());
            }
        }

        Ok(Self::Structure(field_values))
    }
}
