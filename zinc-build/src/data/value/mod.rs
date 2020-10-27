//!
//! The Zinc VM template value.
//!

pub mod contract_field;
pub mod error;
pub mod scalar;

use std::collections::HashSet;

use num::BigInt;
use num::Signed;
use num::Zero;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use zinc_math::InferenceError;

use crate::data::r#type::contract_field::ContractField as ContractFieldType;
use crate::data::r#type::scalar::integer::Type as IntegerType;
use crate::data::r#type::scalar::Type as ScalarType;
use crate::data::r#type::Type;

use self::contract_field::ContractField;
use self::error::context::IContext as IErrorContext;
use self::error::r#type::Type as ErrorType;
use self::error::Error;
use self::scalar::Value as ScalarValue;

///
/// The Zinc VM template value.
///
/// The representation of the input and output data stored in JSON template files.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// Represented with `()` string.
    Unit,
    /// See the inner element description.
    Scalar(ScalarValue),
    /// An enumeration scalar value, represented with its variant name string.
    Enumeration {
        /// The enumeration variant name.
        name: String,
        /// The enumeration variant value.
        value: ScalarValue,
    },

    /// Represented with JSON array.
    Array(Vec<Value>),
    /// Represented with JSON object.
    Structure(Vec<(String, Value)>),
    /// Represented with JSON object.
    Contract(Vec<ContractField>),

    /// The `std::collections::MTreeMap` value.
    Map(Vec<(Value, Value)>),
}

impl Value {
    ///
    /// Creates a value of `r#type`.
    ///
    pub fn new(r#type: Type) -> Self {
        match r#type {
            Type::Unit => Self::Unit,
            Type::Scalar(scalar_type) => match scalar_type {
                ScalarType::Boolean => Self::Scalar(ScalarValue::Boolean(false)),
                ScalarType::Integer(inner) => {
                    Self::Scalar(ScalarValue::Integer(BigInt::zero(), inner))
                }
                ScalarType::Field => Self::Scalar(ScalarValue::Field(BigInt::zero())),
            },
            Type::Enumeration {
                bitlength,
                mut variants,
            } => {
                let (name, value) = variants.remove(0);

                match bitlength {
                    zinc_const::bitlength::FIELD => Self::Enumeration {
                        name,
                        value: ScalarValue::Field(value),
                    },
                    bitlength => Self::Enumeration {
                        name,
                        value: ScalarValue::Integer(value, IntegerType::new(false, bitlength)),
                    },
                }
            }

            Type::Array(r#type, size) => Self::Array(vec![Self::new(*r#type); size]),
            Type::Tuple(fields) => Self::Array(fields.into_iter().map(Self::new).collect()),
            Type::Structure(fields) => Self::Structure(
                fields
                    .into_iter()
                    .map(|(name, r#type)| (name, Self::new(r#type)))
                    .collect(),
            ),
            Type::Contract(fields) => Self::Contract(
                fields
                    .into_iter()
                    .map(ContractField::new_from_type)
                    .collect(),
            ),

            Type::Map { .. } => Self::Map(vec![]),
        }
    }

    ///
    /// Creates a value of `r#type` from the JSON `value`.
    ///
    pub fn try_from_typed_json(value: JsonValue, r#type: Type) -> Result<Self, Error> {
        match r#type {
            Type::Unit => Self::unit_from_json(value),
            Type::Scalar(inner) => Self::scalar_from_json(value, inner),
            Type::Enumeration {
                bitlength,
                variants,
            } => Self::enumeration_from_json(value, bitlength, variants),

            Type::Array(inner, size) => Self::array_from_json(value, *inner, size),
            Type::Tuple(inner) => Self::tuple_from_json(value, inner),
            Type::Structure(fields) => Self::structure_from_json(value, fields),
            Type::Contract(fields) => Self::contract_from_json(value, fields),

            Type::Map {
                key_type,
                value_type,
            } => Self::map_from_json(value, *key_type, *value_type),
        }
    }

    ///
    /// Creates a value from a flat array `flat_values` and data `r#type`.
    ///
    pub fn from_flat_values(r#type: Type, flat_values: &[BigInt]) -> Self {
        match r#type {
            Type::Unit => Self::Unit,
            Type::Scalar(r#type) => match r#type {
                ScalarType::Boolean => flat_values
                    .first()
                    .cloned()
                    .map(|value| value != BigInt::zero())
                    .map(ScalarValue::Boolean)
                    .map(Self::Scalar),
                ScalarType::Integer(r#type) => flat_values
                    .first()
                    .cloned()
                    .map(|value| ScalarValue::Integer(value, r#type))
                    .map(Self::Scalar),
                ScalarType::Field => flat_values
                    .first()
                    .cloned()
                    .map(ScalarValue::Field)
                    .map(Self::Scalar),
            }
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            Type::Enumeration { bitlength, .. } => flat_values
                .first()
                .cloned()
                .map(|value| match bitlength {
                    zinc_const::bitlength::FIELD => ScalarValue::Field(value),
                    bitlength => ScalarValue::Integer(value, IntegerType::new(false, bitlength)),
                })
                .map(Self::Scalar)
                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            Type::Array(r#type, size) => {
                let mut offset = 0;
                let mut result = Vec::with_capacity(size);
                for _ in 0..size {
                    let slice = &flat_values[offset..];
                    offset += r#type.size();
                    result.push(Self::from_flat_values(*r#type.clone(), slice));
                }
                Self::Array(result)
            }

            Type::Tuple(types) => {
                let mut offset = 0;
                let mut result = Vec::with_capacity(types.len());
                for r#type in types.into_iter() {
                    let slice = &flat_values[offset..];
                    offset += r#type.size();
                    result.push(Self::from_flat_values(r#type, slice));
                }
                Self::Array(result)
            }
            Type::Structure(fields) => {
                let mut offset = 0;
                let mut result = Vec::with_capacity(fields.len());
                for (name, r#type) in fields.into_iter() {
                    let slice = &flat_values[offset..];
                    offset += r#type.size();
                    result.push((name, Self::from_flat_values(r#type, slice)));
                }
                Self::Structure(result)
            }
            Type::Contract(fields) => {
                let mut offset = 0;
                let mut result = Vec::with_capacity(fields.len());
                for field in fields.into_iter() {
                    let slice = &flat_values[offset..];
                    offset += field.r#type.size();
                    result.push(ContractField::new(
                        field.name,
                        Self::from_flat_values(field.r#type, slice),
                        field.is_public,
                        field.is_implicit,
                    ));
                }
                Self::Contract(result)
            }

            Type::Map { .. } => Self::Map(vec![]),
        }
    }

    ///
    /// Flattens the value into an array of `BigInt`s.
    ///
    /// Is used to write the input to the VM data stack.
    ///
    pub fn into_flat_values(self) -> Vec<BigInt> {
        match self {
            Self::Unit => vec![],
            Self::Scalar(value) => vec![value.to_bigint()],
            Self::Enumeration { name: _, value } => vec![value.to_bigint()],

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
            Self::Contract(fields) => fields
                .into_iter()
                .map(|field| Self::into_flat_values(field.value))
                .flatten()
                .collect(),

            Self::Map(_entries) => vec![],
        }
    }

    ///
    /// Converts the value to a JSON value.
    ///
    /// Is used to write the values to the input and output JSON template files.
    ///
    pub fn into_json(self) -> JsonValue {
        match self {
            Self::Unit => JsonValue::Null,
            Self::Scalar(scalar) => match scalar {
                ScalarValue::Field(value) => JsonValue::String(format!(
                    "0x{}",
                    value.to_str_radix(zinc_const::base::HEXADECIMAL)
                )),
                ScalarValue::Integer(value, r#type) => {
                    JsonValue::String(if r#type.bitlength == zinc_const::bitlength::ETH_ADDRESS {
                        format!("0x{}", value.to_str_radix(zinc_const::base::HEXADECIMAL))
                    } else {
                        value.to_string()
                    })
                }
                ScalarValue::Boolean(value) => JsonValue::Bool(value),
            },
            Self::Enumeration { name, value: _ } => JsonValue::String(name),

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
            Self::Contract(fields) => {
                let mut object = JsonMap::<String, JsonValue>::with_capacity(fields.len());
                for field in fields.into_iter() {
                    object.insert(field.name, Self::into_json(field.value));
                }
                JsonValue::Object(object)
            }

            Self::Map(entries) => {
                let mut array = Vec::with_capacity(entries.len());
                for (key, value) in entries.into_iter() {
                    array.push(json!({
                        "key": key.into_json(),
                        "value": value.into_json(),
                    }));
                }
                JsonValue::Array(array)
            }
        }
    }

    ///
    /// Creates a unit value from the JSON `value`.
    ///
    fn unit_from_json(value: JsonValue) -> Result<Self, Error> {
        if value.is_null() {
            return Ok(Self::Unit);
        }

        Err(ErrorType::TypeError {
            expected: "null".to_owned(),
            found: value.to_string(),
        }
        .into())
    }

    ///
    /// Creates a boolean value from the JSON `value`.
    ///
    fn boolean_from_json(value: JsonValue) -> Result<Self, Error> {
        let value_bool = value.as_bool().ok_or_else(|| ErrorType::TypeError {
            expected: "true | false".to_owned(),
            found: value.to_string(),
        })?;

        Ok(Self::Scalar(ScalarValue::Boolean(value_bool)))
    }

    ///
    /// Creates an integer value from the JSON `value`.
    ///
    fn integer_from_json(value: JsonValue, r#type: IntegerType) -> Result<Self, Error> {
        let value_string = value.as_str().ok_or_else(|| ErrorType::TypeError {
            expected: "numeric string: 0b[0-1]+ | 0o[0-7]+ | [0-9]+ | 0x[0-9A-Fa-f]+".into(),
            found: value.to_string(),
        })?;

        let bigint_result = zinc_math::bigint_from_str(value_string);
        let bigint =
            bigint_result.map_err(|_| ErrorType::InvalidNumberFormat(value_string.into()))?;
        if bigint.is_negative() && !r#type.is_signed {
            return Err(Error::from(ErrorType::ValueOverflow {
                inner: InferenceError::Overflow {
                    value: bigint,
                    is_signed: r#type.is_signed,
                    bitlength: r#type.bitlength,
                },
            }));
        }

        let bitlength = zinc_math::infer_minimal_bitlength(&bigint, r#type.is_signed)
            .map_err(|error| Error::from(ErrorType::ValueOverflow { inner: error }))?;
        if bitlength > r#type.bitlength {
            return Err(Error::from(ErrorType::ValueOverflow {
                inner: InferenceError::Overflow {
                    value: bigint,
                    is_signed: r#type.is_signed,
                    bitlength: r#type.bitlength,
                },
            }));
        }

        Ok(Self::Scalar(ScalarValue::Integer(bigint, r#type)))
    }

    ///
    /// Creates an enumeration value from the JSON `value`.
    ///
    fn enumeration_from_json(
        value: JsonValue,
        bitlength: usize,
        variants: Vec<(String, BigInt)>,
    ) -> Result<Self, Error> {
        let expected = variants
            .iter()
            .map(|(name, _value)| name.to_owned())
            .collect::<Vec<String>>()
            .join(" | ");
        let value_string = value.as_str().ok_or_else(|| ErrorType::TypeError {
            expected,
            found: value.to_string(),
        })?;

        let bigint = match variants.into_iter().find(|(name, value)| {
            name == value_string
                || zinc_math::bigint_from_str(value.to_string().as_str())
                    == zinc_math::bigint_from_str(value_string)
        }) {
            Some((_name, bigint)) => bigint,
            None => {
                return Err(Error::from(ErrorType::UnexpectedVariant(
                    value_string.to_owned(),
                )))
            }
        };

        match bitlength {
            zinc_const::bitlength::FIELD => Ok(Self::Scalar(ScalarValue::Field(bigint))),
            bitlength => Ok(Self::Scalar(ScalarValue::Integer(
                bigint,
                IntegerType::new(false, bitlength),
            ))),
        }
    }

    ///
    /// Creates a field value from the JSON `value`.
    ///
    fn field_from_json(value: JsonValue) -> Result<Self, Error> {
        let value_string = value.as_str().ok_or_else(|| ErrorType::TypeError {
            expected: "numeric string: 0b[0-1]+ | 0o[0-7]+ | [0-9]+ | 0x[0-9A-Fa-f]+".into(),
            found: value.to_string(),
        })?;

        let bigint_result = zinc_math::bigint_from_str(value_string);
        let bigint =
            bigint_result.map_err(|_| ErrorType::InvalidNumberFormat(value_string.into()))?;

        let bitlength = zinc_math::infer_minimal_bitlength(&bigint, false)
            .map_err(|error| Error::from(ErrorType::ValueOverflow { inner: error }))?;
        if bitlength > zinc_const::bitlength::FIELD {
            return Err(Error::from(ErrorType::ValueOverflow {
                inner: InferenceError::Overflow {
                    value: bigint,
                    is_signed: false,
                    bitlength: zinc_const::bitlength::FIELD,
                },
            }));
        }

        Ok(Self::Scalar(ScalarValue::Field(bigint)))
    }

    ///
    /// Creates a scalar value from the JSON `value`.
    ///
    fn scalar_from_json(value: JsonValue, scalar_type: ScalarType) -> Result<Self, Error> {
        match scalar_type {
            ScalarType::Boolean => Self::boolean_from_json(value),
            ScalarType::Integer(inner) => Self::integer_from_json(value, inner),
            ScalarType::Field => Self::field_from_json(value),
        }
    }

    ///
    /// Creates an array value from the JSON `value`.
    ///
    fn array_from_json(value: JsonValue, r#type: Type, size: usize) -> Result<Self, Error> {
        let array = value
            .as_array()
            .cloned()
            .ok_or_else(|| ErrorType::type_error("JSON array".to_owned(), value))?;

        if array.len() != size {
            return Err(ErrorType::UnexpectedSize {
                expected: size,
                found: array.len(),
            }
            .into());
        }

        let mut values = Vec::with_capacity(size);
        for (index, value) in array.into_iter().enumerate() {
            let typed_value = Self::try_from_typed_json(value, r#type.clone()).push_array(index)?;

            values.push(typed_value);
        }

        Ok(Self::Array(values))
    }

    ///
    /// Creates a tuple value from the JSON `value`.
    ///
    fn tuple_from_json(value: JsonValue, types: Vec<Type>) -> Result<Self, Error> {
        let array = value
            .as_array()
            .cloned()
            .ok_or_else(|| ErrorType::type_error("JSON array".to_owned(), value))?;

        if array.len() != types.len() {
            return Err(ErrorType::UnexpectedSize {
                expected: types.len(),
                found: array.len(),
            }
            .into());
        }

        let mut values = Vec::with_capacity(types.len());
        for (index, (value, r#type)) in array.into_iter().zip(types).enumerate() {
            let typed_value = Self::try_from_typed_json(value, r#type).push_array(index)?;
            values.push(typed_value);
        }

        Ok(Self::Array(values))
    }

    ///
    /// Creates a structure value from the JSON `value`.
    ///
    fn structure_from_json(
        value: JsonValue,
        field_types: Vec<(String, Type)>,
    ) -> Result<Self, Error> {
        let mut object = value
            .as_object()
            .cloned()
            .ok_or_else(|| ErrorType::type_error("JSON object".to_owned(), value))?;

        let mut used_fields = HashSet::with_capacity(field_types.len());
        let mut field_values = Vec::with_capacity(field_types.len());
        for (name, r#type) in field_types.into_iter() {
            used_fields.insert(name.clone());

            let json_value = object
                .remove(name.as_str())
                .ok_or_else(|| ErrorType::MissingField(name.clone()))?;

            let value =
                Self::try_from_typed_json(json_value, r#type).push_structure(name.as_str())?;

            field_values.push((name, value));
        }

        for field in object.keys() {
            if !used_fields.contains(field.as_str()) {
                return Err(ErrorType::UnexpectedField(field.clone()).into());
            }
        }

        Ok(Self::Structure(field_values))
    }

    ///
    /// Creates a contract value from the JSON `value`.
    ///
    fn contract_from_json(
        value: JsonValue,
        field_types: Vec<ContractFieldType>,
    ) -> Result<Self, Error> {
        let mut object = value
            .as_object()
            .cloned()
            .ok_or_else(|| ErrorType::type_error("JSON object".to_owned(), value))?;

        let mut used_fields = HashSet::with_capacity(field_types.len());
        let mut field_values = Vec::with_capacity(field_types.len());
        for field_type in field_types.into_iter() {
            used_fields.insert(field_type.name.clone());

            let json_value = object
                .remove(field_type.name.as_str())
                .ok_or_else(|| ErrorType::MissingField(field_type.name.clone()))?;

            let value = Self::try_from_typed_json(json_value, field_type.r#type)
                .push_structure(field_type.name.as_str())?;

            field_values.push(ContractField::new(
                field_type.name,
                value,
                field_type.is_public,
                field_type.is_implicit,
            ));
        }

        for field in object.keys() {
            if !used_fields.contains(field.as_str()) {
                return Err(ErrorType::UnexpectedField(field.clone()).into());
            }
        }

        Ok(Self::Contract(field_values))
    }

    ///
    /// Creates an `std::collections::MTreeMap` value from the JSON `value`.
    ///
    fn map_from_json(value: JsonValue, key_type: Type, value_type: Type) -> Result<Self, Error> {
        let entries = match value {
            JsonValue::Array(array) => array,
            value => return Err(ErrorType::InvalidNumberFormat(value.to_string()).into()),
        };

        let mut result = Vec::with_capacity(entries.len());
        for entry in entries.into_iter() {
            let entry = entry
                .as_object()
                .ok_or_else(|| ErrorType::InvalidMapFormat(entry.to_string()))?;

            let key = entry
                .get("key")
                .cloned()
                .ok_or_else(|| ErrorType::MissingField("key".to_owned()))?;
            let key = Self::try_from_typed_json(key, key_type.clone())?;

            let value = entry
                .get("value")
                .cloned()
                .ok_or_else(|| ErrorType::MissingField("value".to_owned()))?;
            let value = Self::try_from_typed_json(value, value_type.clone())?;

            result.push((key, value));
        }
        Ok(Self::Map(result))
    }
}
