//!
//! The match expression exhausting data.
//!

use std::collections::HashMap;

use num::BigInt;
use num::One;
use num::Zero;

use crate::semantic::element::r#type::enumeration::Enumeration;
use zinc_lexical::Location;

///
/// The object, describing the `match` expression exhaustion process.
///
pub struct Data {
    /// The patterns, which appear in the `match` expression.
    patterns: HashMap<BigInt, Location>,
    /// The enumeration type, is the `match` expressions matches one.
    /// In this case, all the enumeration variant must be covered at least once.
    enumeration_type: Option<Enumeration>,
}

impl Data {
    /// The pattern hashmap default capacity.
    const DEFAULT_INITIAL_PATTERN_HASHMAP_SIZE: usize = 4;

    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self {
            patterns: HashMap::with_capacity(Self::DEFAULT_INITIAL_PATTERN_HASHMAP_SIZE),
            enumeration_type: None,
        }
    }

    ///
    /// Inserts a boolean pattern to the exhaustion hashmap.
    ///
    pub fn insert_boolean(&mut self, value: bool, location: Location) -> Option<Location> {
        self.patterns
            .insert(if value { BigInt::one() } else { BigInt::zero() }, location)
    }

    ///
    /// Inserts an integer pattern to the exhaustion hashmap.
    ///
    pub fn insert_integer(
        &mut self,
        value: BigInt,
        enumeration_type: Option<Enumeration>,
        location: Location,
    ) -> Option<Location> {
        self.enumeration_type = enumeration_type;
        self.patterns.insert(value, location)
    }

    ///
    /// Checks if the boolean patterns cover all the possible boolean values.
    ///
    pub fn has_exhausted_boolean(&self) -> bool {
        let mut current = self.patterns.keys().cloned().collect::<Vec<BigInt>>();
        current.sort();

        let full = vec![BigInt::zero(), BigInt::one()];

        current == full
    }

    ///
    /// Checks if the integer patterns cover all the possible integer values.
    ///
    pub fn has_exhausted_integer(&self) -> bool {
        match self.enumeration_type {
            Some(ref enumeration) => {
                let mut current = self.patterns.keys().cloned().collect::<Vec<BigInt>>();
                current.sort();

                let mut enum_values = enumeration.values.to_owned();
                enum_values.sort();
                current == enum_values
            }
            None => false,
        }
    }
}
