//!
//! The match expression exhausting data.
//!

use std::collections::HashMap;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::enumeration::Enumeration;

pub struct Data {
    patterns: HashMap<BigInt, Location>,
    enumeration_type: Option<Enumeration>,
}

impl Data {
    const DEFAULT_INITIAL_PATTERN_HASHMAP_SIZE: usize = 4;

    pub fn new() -> Self {
        Self {
            patterns: HashMap::with_capacity(Self::DEFAULT_INITIAL_PATTERN_HASHMAP_SIZE),
            enumeration_type: None,
        }
    }

    pub fn insert_boolean(&mut self, value: bool, location: Location) -> Option<Location> {
        self.patterns
            .insert(if value { BigInt::one() } else { BigInt::zero() }, location)
    }

    pub fn insert_integer(
        &mut self,
        value: BigInt,
        enumeration_type: Option<Enumeration>,
        location: Location,
    ) -> Option<Location> {
        self.enumeration_type = enumeration_type;
        self.patterns.insert(value, location)
    }

    pub fn is_exhausted_boolean(&self) -> bool {
        let mut current = self.patterns.keys().cloned().collect::<Vec<BigInt>>();
        current.sort();

        let full = vec![BigInt::zero(), BigInt::one()];

        current == full
    }

    pub fn is_exhausted_integer(&self) -> bool {
        match self.enumeration_type {
            Some(ref enumeration) => {
                let mut current = self.patterns.keys().cloned().collect::<Vec<BigInt>>();
                current.sort();

                current == enumeration.values
            }
            None => false,
        }
    }
}
