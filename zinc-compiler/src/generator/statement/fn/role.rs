//!
//! The generator function role.
//!

///
/// The generator function role.
///
/// Describes special function traits, e.g. circuit entry or contract method.
///
#[derive(Debug, Clone, Copy)]
pub enum Role {
    /// The ordinar user-defined function.
    Ordinar,
    /// The circuit entry.
    CircuitEntry,
    /// A contract constructor.
    ContractConstuctor,
    /// A contract method entry.
    ContractMethodEntry,
    /// A unit test.
    UnitTest,
}
