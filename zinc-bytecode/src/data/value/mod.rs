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
pub struct StructField {
    pub field: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Unit,
    Scalar(ScalarValue),
    Struct(Vec<StructField>),
    Array(Vec<Value>),
}

impl Value {
    pub fn default_from_type(data_type: &DataType) -> Self {
        match data_type {
            DataType::Unit => Value::Unit,
            DataType::Scalar(scalar_type) => match scalar_type {
                ScalarType::Boolean => Value::Scalar(ScalarValue::Bool(false)),
                ScalarType::Integer(int_type) => {
                    Value::Scalar(ScalarValue::Integer(0.into(), int_type.to_owned()))
                }
                ScalarType::Field => Value::Scalar(ScalarValue::Field(0.into())),
            },
            DataType::Enum => Value::Scalar(ScalarValue::Field(0.into())),

            DataType::Array(data_type, len) => {
                Value::Array(vec![Value::default_from_type(data_type); *len])
            }
            DataType::Struct(fields) => Value::Struct(
                fields
                    .iter()
                    .map(|(name, data_type)| StructField {
                        field: name.clone(),
                        value: Value::default_from_type(data_type),
                    })
                    .collect(),
            ),
            DataType::Tuple(fields) => Value::Array(
                fields
                    .iter()
                    .map(|r#type| Value::default_from_type(r#type))
                    .collect(),
            ),
        }
    }

    pub fn to_flat_values(&self) -> Vec<BigInt> {
        let mut flat_array = Vec::new();
        self.to_flat_values_recursive(&mut flat_array);
        flat_array
    }

    fn to_flat_values_recursive(&self, flat_array: &mut Vec<BigInt>) {
        match self {
            Value::Unit => {}
            Value::Scalar(value) => flat_array.push(value.to_bigint()),
            Value::Struct(fields) => {
                for StructField { value, .. } in fields.iter() {
                    value.to_flat_values_recursive(flat_array);
                }
            }
            Value::Array(values) => {
                for value in values.iter() {
                    value.to_flat_values_recursive(flat_array);
                }
            }
        }
    }

    /// Creates value from flat array and data type.
    pub fn from_flat_values(data_type: &DataType, flat_values: &[BigInt]) -> Option<Self> {
        let mut value = Self::default_from_type(data_type);
        let consumed = value.fill_from_flat_values(flat_values)?;
        if consumed == flat_values.len() {
            Some(value)
        } else {
            None
        }
    }

    /// Fills values from slice, returns number of used values or None if there is not enough.
    fn fill_from_flat_values(&mut self, flat_values: &[BigInt]) -> Option<usize> {
        match self {
            Value::Unit => Some(0),
            Value::Scalar(scalar) => {
                match scalar {
                    ScalarValue::Field(value) | ScalarValue::Integer(value, _) => {
                        *value = flat_values.first()?.clone();
                    }
                    ScalarValue::Bool(value) => {
                        *value = flat_values.first()? != &BigInt::from(0);
                    }
                }
                Some(1)
            }
            Value::Struct(fields) => {
                let mut offset = 0;
                for StructField { value, .. } in fields.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            }
            Value::Array(values) => {
                let mut offset = 0;
                for value in values.iter_mut() {
                    let slice = &flat_values[offset..];
                    offset += value.fill_from_flat_values(slice)?;
                }
                Some(offset)
            }
        }
    }

    pub fn to_json(&self) -> JsonValue {
        match self {
            Value::Unit => JsonValue::String("unit".into()),
            Value::Scalar(scalar) => match scalar {
                ScalarValue::Field(value) => {
                    if value <= &BigInt::from(std::u64::MAX) {
                        JsonValue::String(value.to_str_radix(10))
                    } else {
                        JsonValue::String(String::from("0x") + value.to_str_radix(16).as_str())
                    }
                }
                ScalarValue::Integer(value, int_type) => {
                    if value <= &BigInt::from(std::u64::MAX) || int_type.is_signed {
                        JsonValue::String(value.to_str_radix(10))
                    } else {
                        JsonValue::String(String::from("0x") + value.to_str_radix(16).as_str())
                    }
                }
                ScalarValue::Bool(value) => JsonValue::Bool(*value),
            },
            Value::Array(values) => JsonValue::Array(values.iter().map(Self::to_json).collect()),
            Value::Struct(fields) => {
                let mut object = JsonMap::<String, JsonValue>::new();
                for field in fields.iter() {
                    object.insert(field.field.clone(), field.value.to_json());
                }
                JsonValue::Object(object)
            }
        }
    }

    pub fn from_typed_json(value: &JsonValue, data_type: &DataType) -> Result<Self, Error> {
        match data_type {
            DataType::Unit => Self::unit_from_json(value),
            DataType::Scalar(inner) => Self::scalar_from_json(value, inner),
            DataType::Enum => Self::field_from_json(value),

            DataType::Array(inner, size) => Self::array_from_json(value, inner, *size),
            DataType::Tuple(inner) => Self::tuple_from_json(value, inner),
            DataType::Struct(fields) => Self::struct_from_json(value, fields),
        }
    }

    fn unit_from_json(value: &JsonValue) -> Result<Self, Error> {
        if let Some(s) = value.as_str() {
            if s == "unit" {
                return Ok(Value::Unit);
            }
        }
        Err(ErrorType::TypeError {
            expected: "\"unit\"".into(),
            actual: value.to_string(),
        }
        .into())
    }

    fn scalar_from_json(value: &JsonValue, scalar_type: &ScalarType) -> Result<Self, Error> {
        match scalar_type {
            ScalarType::Boolean => Self::boolean_from_json(value),
            ScalarType::Integer(inner) => Self::integer_from_json(value, inner),
            ScalarType::Field => Self::field_from_json(value),
        }
    }

    fn field_from_json(value: &JsonValue) -> Result<Self, Error> {
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

        Ok(Value::Scalar(ScalarValue::Field(bigint)))
    }

    fn boolean_from_json(value: &JsonValue) -> Result<Self, Error> {
        let value_bool = value.as_bool().ok_or_else(|| ErrorType::TypeError {
            expected: "boolean (true or false)".into(),
            actual: value.to_string(),
        })?;

        Ok(Value::Scalar(ScalarValue::Bool(value_bool)))
    }

    fn integer_from_json(value: &JsonValue, _type: &IntegerType) -> Result<Self, Error> {
        // TODO: overflow check

        Self::field_from_json(value)
    }

    fn struct_from_json(
        value: &JsonValue,
        field_types: &[(String, DataType)],
    ) -> Result<Self, Error> {
        let object = value
            .as_object()
            .ok_or_else(|| ErrorType::type_error("structure", value))?;

        let mut used_fields = HashSet::<&str>::new();
        let mut field_values = Vec::with_capacity(field_types.len());
        for (name, dtype) in field_types {
            used_fields.insert(name.as_str());

            let json_value = object
                .get(name)
                .ok_or_else(|| ErrorType::MissingField(name.clone()))?;

            let typed_value = Self::from_typed_json(json_value, dtype).in_struct(name.as_str())?;

            field_values.push(StructField {
                field: name.clone(),
                value: typed_value,
            })
        }

        for field in object.keys() {
            if !used_fields.contains(field.as_str()) {
                return Err(ErrorType::UnexpectedField(field.clone()).into());
            }
        }

        Ok(Value::Struct(field_values))
    }

    fn tuple_from_json(value: &JsonValue, types: &[DataType]) -> Result<Self, Error> {
        let array = value
            .as_array()
            .ok_or_else(|| ErrorType::type_error("tuple (json array)", value))?;

        if array.len() != types.len() {
            return Err(ErrorType::UnexpectedSize {
                expected: types.len(),
                actual: array.len(),
            }
            .into());
        }

        let mut values = Vec::with_capacity(types.len());
        for (index, (value, dtype)) in array.iter().zip(types).enumerate() {
            let typed_value = Self::from_typed_json(value, dtype).in_array(index)?;
            values.push(typed_value);
        }

        Ok(Value::Array(values))
    }

    fn array_from_json(value: &JsonValue, dtype: &DataType, size: usize) -> Result<Self, Error> {
        let array = value
            .as_array()
            .ok_or_else(|| ErrorType::type_error("array", value))?;

        if array.len() != size {
            return Err(ErrorType::UnexpectedSize {
                expected: size,
                actual: array.len(),
            }
            .into());
        }

        let mut values = Vec::with_capacity(size);
        for (index, value) in array.iter().enumerate() {
            let typed_value = Self::from_typed_json(value, dtype).in_array(index)?;

            values.push(typed_value);
        }

        Ok(Value::Array(values))
    }
}
