//!
//! The Zinc smart contract constants.
//!

/// The default constructor name.
pub static CONSTRUCTOR_IDENTIFIER: &str = "new";

/// The implicit fields count.
pub const IMPLICIT_FIELDS_COUNT: usize = 2;

/// The implicit fields size.
pub const IMPLICIT_FIELDS_SIZE: usize = 1;

/// The first default implicit field index.
pub const FIELD_INDEX_ADDRESS: usize = 0;

/// The first default implicit field name.
pub static FIELD_NAME_ADDRESS: &str = "address";

/// The second default implicit field index.
pub const FIELD_INDEX_BALANCES: usize = 1;

/// The second default implicit field name.
pub static FIELD_NAME_BALANCES: &str = "balances";

/// The implicit transaction variable name.
pub static TRANSACTION_VARIABLE_NAME: &str = "msg";

/// The implicit transaction variable fields count.
pub const TRANSACTION_FIELDS_COUNT: usize = 4;

/// The implicit transaction variable size.
pub const TRANSACTION_SIZE: usize = 4;
