//!
//! The Zinc Mongo contract storage tools.
//!

use std::str::FromStr;

use bson::Array as BsonArray;
use bson::Bson;
use bson::Document as BsonDocument;
use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;
use serde_derive::Deserialize;

use zinc_bytecode::DataType;
use zinc_bytecode::ScalarType;

///
/// The MongoDB contract storage.
///
#[derive(Debug, Deserialize)]
pub struct Storage {
    /// The storage fields array.
    #[serde(rename = "storage")]
    pub data: Vec<Bson>,
}

impl Storage {
    ///
    /// Initializes a storage with zero values.
    ///
    pub fn new(types: Vec<DataType>) -> Self {
        let values: Vec<Vec<BigInt>> = types
            .iter()
            .map(|r#type| vec![BigInt::zero(); r#type.size()])
            .collect();

        Self::from_flat_values(types, values)
    }

    ///
    /// Converts a BSON document into an array form, which preserves the field order and
    /// can be stored in a persistent storage.
    ///
    pub fn from_bson(bson: Bson) -> Self {
        let storage: Vec<Bson> = bson
            .as_document()
            .cloned()
            .expect(zinc_const::panic::DATA_SERIALIZATION)
            .into_iter()
            .map(|(_name, value)| value)
            .collect();

        Self { data: storage }
    }

    ///
    /// Initializes a storage from the `flat_values`, taking the storage structure from `r#type`.
    ///
    pub fn from_flat_values(types: Vec<DataType>, values: Vec<Vec<BigInt>>) -> Self {
        let storage = values
            .into_iter()
            .zip(types)
            .filter_map(|(values, r#type)| Self::reshape_field(r#type, values.as_slice()))
            .collect();

        Self { data: storage }
    }

    ///
    /// Converts the storage into a generic BSON document.
    ///
    pub fn into_bson(self) -> Bson {
        Bson::Array(self.data.into_iter().collect::<BsonArray>())
    }

    ///
    /// Converts the storage into series of `BigInt`s.
    ///
    pub fn into_flat_values(self) -> Vec<Vec<BigInt>> {
        self.data.into_iter().map(Self::flatten_field).collect()
    }

    ///
    /// Fills values from slice, returns number of used values or None if there is not enough.
    ///
    fn reshape_field(r#type: DataType, flat_values: &[BigInt]) -> Option<Bson> {
        match r#type {
            DataType::Scalar(r#type) => match r#type {
                ScalarType::Boolean => flat_values
                    .first()
                    .cloned()
                    .map(|value| value != BigInt::zero())
                    .map(Bson::Boolean),
                ScalarType::Field | ScalarType::Integer(_) => flat_values
                    .first()
                    .map(|value| value.to_string())
                    .map(Bson::String),
            },
            DataType::Array(r#type, size) => {
                let mut offset = 0;
                let mut array = BsonArray::with_capacity(size);
                for _ in 0..size {
                    let slice = &flat_values[offset..];
                    let size = r#type.size();
                    if let Some(value) = Self::reshape_field(*r#type.clone(), slice) {
                        offset += size;
                        array.push(value);
                    }
                }
                Some(Bson::Array(array))
            }
            DataType::Tuple(types) => {
                let mut offset = 0;
                let mut tuple = BsonArray::with_capacity(types.len());
                for r#type in types.into_iter() {
                    let slice = &flat_values[offset..];
                    let size = r#type.size();
                    if let Some(value) = Self::reshape_field(r#type, slice) {
                        offset += size;
                        tuple.push(value);
                    }
                }
                Some(Bson::Array(tuple))
            }
            DataType::Structure(fields) | DataType::Contract(fields) => {
                let mut offset = 0;
                let mut document = BsonDocument::new();
                for (name, r#type) in fields.into_iter() {
                    let slice = &flat_values[offset..];
                    let size = r#type.size();
                    if let Some(value) = Self::reshape_field(r#type, slice) {
                        offset += size;
                        document.insert(name, value);
                    }
                }
                Some(Bson::Document(document))
            }
            _ => None,
        }
    }

    ///
    /// Flattens the value into an array of `BigInt`s.
    ///
    /// Is used to fill the VM storage.
    ///
    fn flatten_field(bson: Bson) -> Vec<BigInt> {
        match bson {
            Bson::Boolean(value) => vec![if value { BigInt::one() } else { BigInt::zero() }],
            Bson::String(value) => {
                vec![BigInt::from_str(value.as_str()).expect(zinc_const::panic::DATA_SERIALIZATION)]
            }
            Bson::Array(values) => values
                .into_iter()
                .map(Self::flatten_field)
                .flatten()
                .collect(),
            Bson::Document(fields) => fields
                .into_iter()
                .map(|(_name, value)| Self::flatten_field(value))
                .flatten()
                .collect(),
            _ => vec![],
        }
    }
}
