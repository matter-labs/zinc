//!
//! The semantic analyzer place element resolution time.
//!

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolutionTime {
    Static,
    Dynamic,
}
