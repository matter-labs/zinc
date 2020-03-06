//!
//! The Zinc compiler file input error.
//!

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Opening(io::Error),
    Metadata(io::Error),
    Reading(io::Error),
}

impl PartialEq<Self> for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Opening(inner_1), Self::Opening(inner_2)) => {
                inner_1.to_string() == inner_2.to_string()
            }
            (Self::Metadata(inner_1), Self::Opening(inner_2)) => {
                inner_1.to_string() == inner_2.to_string()
            }
            (Self::Reading(inner_1), Self::Reading(inner_2)) => {
                inner_1.to_string() == inner_2.to_string()
            }
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Opening(inner) => write!(f, "{}", inner),
            Self::Metadata(inner) => write!(f, "{}", inner),
            Self::Reading(inner) => write!(f, "{}", inner),
        }
    }
}
