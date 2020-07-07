//!
//! The Zinc Mongo contract storage tools.
//!

use std::str::FromStr;

use bson::Array as BsonArray;
use bson::Bson;
use bson::Document as BsonDocument;
use num_bigint::BigInt;

use serde_derive::Deserialize;

///
/// The MongoDB contract storage.
///
#[derive(Debug, Deserialize)]
pub struct Storage {
    storage: Vec<BsonDocument>,
}

impl Storage {
    ///
    /// Converts a BSON document into an array form, which preserves the field order and
    /// can be stored in a persistent storage.
    ///
    pub fn from_bson(bson: Bson) -> Self {
        let bson = bson
            .as_document()
            .cloned()
            .expect(zinc_const::panic::DATA_SERIALIZATION);

        let mut storage = Vec::with_capacity(bson.len());
        for (name, value) in bson.into_iter() {
            storage.push(bson::doc! {
                "name": Bson::String(name),
                "value": value,
            });
        }
        Self { storage }
    }

    ///
    /// Converts the storage into a generic BSON document.
    ///
    pub fn into_bson(self) -> Bson {
        Bson::Array(
            self.storage
                .into_iter()
                .map(Bson::Document)
                .collect::<BsonArray>(),
        )
    }

    ///
    /// Converts the storage into series of `BigInt`s.
    ///
    pub fn into_flat_bigints(self) -> Vec<Vec<BigInt>> {
        self.storage
            .into_iter()
            .map(|mut document| {
                document
                    .remove("value")
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
            })
            .map(Self::flatten_field)
            .collect()
    }

    ///
    /// Flattens the value into an array of `BigInt`s.
    ///
    /// Is used to fill the VM storage.
    ///
    fn flatten_field(bson: Bson) -> Vec<BigInt> {
        match bson {
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
