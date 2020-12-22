//!
//! The template value.
//!

pub mod contract_field;
pub mod scalar;

use std::collections::HashSet;

use anyhow::Context;
use num::BigInt;
use num::Signed;
use num::Zero;
use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::contract_field::ContractField as ContractFieldType;
use crate::data::r#type::scalar::integer::Type as IntegerType;
use crate::data::r#type::scalar::Type as ScalarType;
use crate::data::r#type::Type;
use crate::error::Error;

use self::contract_field::ContractField;
use self::scalar::Value as ScalarValue;

///
/// The template value.
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
    pub fn try_from_typed_json(value: serde_json::Value, r#type: Type) -> anyhow::Result<Self> {
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
    pub fn into_json(self) -> serde_json::Value {
        match self {
            Self::Unit => serde_json::Value::Null,
            Self::Scalar(scalar) => match scalar {
                ScalarValue::Field(value) => serde_json::Value::String(format!(
                    "0x{}",
                    value.to_str_radix(zinc_const::base::HEXADECIMAL)
                )),
                ScalarValue::Integer(value, r#type) => serde_json::Value::String(
                    if r#type.bitlength == zinc_const::bitlength::ETH_ADDRESS {
                        format!("0x{}", value.to_str_radix(zinc_const::base::HEXADECIMAL))
                    } else {
                        value.to_string()
                    },
                ),
                ScalarValue::Boolean(value) => serde_json::Value::Bool(value),
            },
            Self::Enumeration { name, value: _ } => serde_json::Value::String(name),

            Self::Array(values) => {
                serde_json::Value::Array(values.into_iter().map(Self::into_json).collect())
            }
            Self::Structure(fields) => {
                let mut object =
                    serde_json::Map::<String, serde_json::Value>::with_capacity(fields.len());
                for (name, value) in fields.into_iter() {
                    if name == zinc_lexical::Keyword::SelfLowercase.to_string() {
                        continue;
                    }

                    object.insert(name, Self::into_json(value));
                }
                serde_json::Value::Object(object)
            }
            Self::Contract(fields) => {
                let mut object =
                    serde_json::Map::<String, serde_json::Value>::with_capacity(fields.len());
                for field in fields.into_iter() {
                    object.insert(field.name, Self::into_json(field.value));
                }
                serde_json::Value::Object(object)
            }

            Self::Map(entries) => {
                let mut array = Vec::with_capacity(entries.len());
                for (key, value) in entries.into_iter() {
                    array.push(serde_json::json!({
                        "key": key.into_json(),
                        "value": value.into_json(),
                    }));
                }
                serde_json::Value::Array(array)
            }
        }
    }

    ///
    /// Inserts a contract address `self` argument into the function arguments structure.
    ///
    pub fn insert_contract_instance(&mut self, value: BigInt) {
        if let Self::Structure(arguments) = self {
            arguments.insert(
                0,
                (
                    zinc_lexical::Keyword::SelfLowercase.to_string(),
                    Self::Scalar(ScalarValue::Integer(value, IntegerType::ETH_ADDRESS)),
                ),
            );
        }
    }

    ///
    /// Creates a unit value from the JSON `value`.
    ///
    fn unit_from_json(value: serde_json::Value) -> anyhow::Result<Self> {
        if !value.is_null() {
            anyhow::bail!(Error::TypeError {
                expected: "null".to_owned(),
                found: value.to_string(),
            });
        }

        Ok(Self::Unit)
    }

    ///
    /// Creates a boolean value from the JSON `value`.
    ///
    fn boolean_from_json(value: serde_json::Value) -> anyhow::Result<Self> {
        let value_bool = value.as_bool().ok_or_else(|| Error::TypeError {
            expected: "true | false".to_owned(),
            found: value.to_string(),
        })?;

        Ok(Self::Scalar(ScalarValue::Boolean(value_bool)))
    }

    ///
    /// Creates an integer value from the JSON `value`.
    ///
    fn integer_from_json(value: serde_json::Value, r#type: IntegerType) -> anyhow::Result<Self> {
        let value_string = value.as_str().ok_or_else(|| Error::TypeError {
            expected: "numeric string: 0b[0-1]+ | 0o[0-7]+ | [0-9]+ | 0x[0-9A-Fa-f]+".into(),
            found: value.to_string(),
        })?;

        let bigint = zinc_math::bigint_from_str(value_string).map_err(Error::from)?;
        if bigint.is_negative() && !r#type.is_signed {
            anyhow::bail!(Error::from(zinc_math::Error::Overflow {
                value: bigint,
                is_signed: r#type.is_signed,
                bitlength: r#type.bitlength,
            }));
        }

        let bitlength =
            zinc_math::infer_minimal_bitlength(&bigint, r#type.is_signed).map_err(Error::from)?;
        if bitlength > r#type.bitlength {
            anyhow::bail!(Error::from(zinc_math::Error::Overflow {
                value: bigint,
                is_signed: r#type.is_signed,
                bitlength: r#type.bitlength,
            }));
        }

        Ok(Self::Scalar(ScalarValue::Integer(bigint, r#type)))
    }

    ///
    /// Creates an enumeration value from the JSON `value`.
    ///
    fn enumeration_from_json(
        value: serde_json::Value,
        bitlength: usize,
        variants: Vec<(String, BigInt)>,
    ) -> anyhow::Result<Self> {
        let expected = variants
            .iter()
            .map(|(name, _value)| name.to_owned())
            .collect::<Vec<String>>()
            .join(" | ");
        let value_string = value.as_str().ok_or_else(|| Error::TypeError {
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
                anyhow::bail!(Error::UnexpectedVariant(value_string.to_owned(),));
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
    fn field_from_json(value: serde_json::Value) -> anyhow::Result<Self> {
        let value_string = value.as_str().ok_or_else(|| Error::TypeError {
            expected: "numeric string: 0b[0-1]+ | 0o[0-7]+ | [0-9]+ | 0x[0-9A-Fa-f]+".into(),
            found: value.to_string(),
        })?;

        let bigint_result = zinc_math::bigint_from_str(value_string);
        let bigint =
            bigint_result.map_err(|_| Error::InvalidNumberFormat(value_string.to_owned()))?;

        let bitlength = zinc_math::infer_minimal_bitlength(&bigint, false).map_err(Error::from)?;
        if bitlength > zinc_const::bitlength::FIELD {
            anyhow::bail!(Error::from(zinc_math::Error::Overflow {
                value: bigint,
                is_signed: false,
                bitlength: zinc_const::bitlength::FIELD,
            }));
        }

        Ok(Self::Scalar(ScalarValue::Field(bigint)))
    }

    ///
    /// Creates a scalar value from the JSON `value`.
    ///
    fn scalar_from_json(value: serde_json::Value, scalar_type: ScalarType) -> anyhow::Result<Self> {
        match scalar_type {
            ScalarType::Boolean => Self::boolean_from_json(value),
            ScalarType::Integer(inner) => Self::integer_from_json(value, inner),
            ScalarType::Field => Self::field_from_json(value),
        }
    }

    ///
    /// Creates an array value from the JSON `value`.
    ///
    fn array_from_json(
        value: serde_json::Value,
        r#type: Type,
        size: usize,
    ) -> anyhow::Result<Self> {
        let array = value
            .as_array()
            .cloned()
            .ok_or_else(|| Error::type_error("JSON array".to_owned(), value))?;

        if array.len() != size {
            anyhow::bail!(Error::UnexpectedSize {
                expected: size,
                found: array.len(),
            });
        }

        let mut values = Vec::with_capacity(size);
        for (index, value) in array.into_iter().enumerate() {
            let typed_value = Self::try_from_typed_json(value, r#type.clone())
                .with_context(|| format!("[{}]", index))?;

            values.push(typed_value);
        }

        Ok(Self::Array(values))
    }

    ///
    /// Creates a tuple value from the JSON `value`.
    ///
    fn tuple_from_json(value: serde_json::Value, types: Vec<Type>) -> anyhow::Result<Self> {
        let array = value
            .as_array()
            .cloned()
            .ok_or_else(|| Error::type_error("JSON array".to_owned(), value))?;

        if array.len() != types.len() {
            anyhow::bail!(Error::UnexpectedSize {
                expected: types.len(),
                found: array.len(),
            });
        }

        let mut values = Vec::with_capacity(types.len());
        for (index, (value, r#type)) in array.into_iter().zip(types).enumerate() {
            let typed_value =
                Self::try_from_typed_json(value, r#type).with_context(|| format!("[{}]", index))?;
            values.push(typed_value);
        }

        Ok(Self::Array(values))
    }

    ///
    /// Creates a structure value from the JSON `value`.
    ///
    fn structure_from_json(
        value: serde_json::Value,
        field_types: Vec<(String, Type)>,
    ) -> anyhow::Result<Self> {
        let mut object = value
            .as_object()
            .cloned()
            .ok_or_else(|| Error::type_error("JSON object".to_owned(), value))?;

        let mut used_fields = HashSet::with_capacity(field_types.len());
        let mut field_values = Vec::with_capacity(field_types.len());
        for (name, r#type) in field_types.into_iter() {
            if name == zinc_lexical::Keyword::SelfLowercase.to_string() {
                continue;
            }

            used_fields.insert(name.clone());

            let json_value = object
                .remove(name.as_str())
                .ok_or_else(|| Error::MissingField(name.clone()))?;

            let value = Self::try_from_typed_json(json_value, r#type)
                .with_context(|| format!(".{}", name))?;

            field_values.push((name, value));
        }

        for field in object.keys() {
            if !used_fields.contains(field.as_str()) {
                anyhow::bail!(Error::UnexpectedField(field.clone()));
            }
        }

        Ok(Self::Structure(field_values))
    }

    ///
    /// Creates a contract value from the JSON `value`.
    ///
    fn contract_from_json(
        value: serde_json::Value,
        field_types: Vec<ContractFieldType>,
    ) -> anyhow::Result<Self> {
        let mut object = value
            .as_object()
            .cloned()
            .ok_or_else(|| Error::type_error("JSON object".to_owned(), value))?;

        let mut used_fields = HashSet::with_capacity(field_types.len());
        let mut field_values = Vec::with_capacity(field_types.len());
        for field_type in field_types.into_iter() {
            used_fields.insert(field_type.name.clone());

            let json_value = object
                .remove(field_type.name.as_str())
                .ok_or_else(|| Error::MissingField(field_type.name.clone()))?;

            let field_name = field_type.name.clone();
            let value = Self::try_from_typed_json(json_value, field_type.r#type)
                .with_context(|| format!(".{}", field_name))?;

            field_values.push(ContractField::new(
                field_type.name,
                value,
                field_type.is_public,
                field_type.is_implicit,
            ));
        }

        for field in object.keys() {
            if !used_fields.contains(field.as_str()) {
                anyhow::bail!(Error::UnexpectedField(field.clone()));
            }
        }

        Ok(Self::Contract(field_values))
    }

    ///
    /// Creates an `std::collections::MTreeMap` value from the JSON `value`.
    ///
    fn map_from_json(
        value: serde_json::Value,
        key_type: Type,
        value_type: Type,
    ) -> anyhow::Result<Self> {
        let entries = match value {
            serde_json::Value::Array(array) => array,
            value => anyhow::bail!(Error::InvalidNumberFormat(value.to_string())),
        };

        let mut result = Vec::with_capacity(entries.len());
        for entry in entries.into_iter() {
            let entry = entry
                .as_object()
                .ok_or_else(|| Error::InvalidMapFormat(entry.to_string()))?;

            let key = entry
                .get("key")
                .cloned()
                .ok_or_else(|| Error::MissingField("key".to_owned()))?;
            let key = Self::try_from_typed_json(key, key_type.clone())?;

            let value = entry
                .get("value")
                .cloned()
                .ok_or_else(|| Error::MissingField("value".to_owned()))?;
            let value = Self::try_from_typed_json(value, value_type.clone())?;

            result.push((key, value));
        }
        Ok(Self::Map(result))
    }
}
