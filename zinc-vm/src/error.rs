//!
//! The Zinc virtual machine error.
//!

use num::BigInt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeSizeError {
    #[error("expected input value of size {expected}, found {found}")]
    Input { expected: usize, found: usize },

    #[error("expected output value of size {expected}, found {found}")]
    Output { expected: usize, found: usize },
}

#[derive(Debug, Error)]
pub enum MalformedBytecode {
    #[error("invalid arguments to an intrinsic function: {0}")]
    InvalidArguments(String),

    #[error("unexpected `loop_end` instruction")]
    UnexpectedLoopEnd,

    #[error("unexpected `return` instruction")]
    UnexpectedReturn,

    #[error("unexpected `else` instruction")]
    UnexpectedElse,

    #[error("unexpected `end_if` instruction")]
    UnexpectedEndIf,

    #[error("stack underflow")]
    StackUnderflow,

    #[error("reading uninitialized memory")]
    UninitializedStorageAccess,

    #[error("conditional branches produced results of different sizes")]
    BranchStacksDoNotMatch,
}

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("value overflow: value {0} is not in the field")]
    ValueOverflow(BigInt),

    #[error("failed to synthesize circuit: {0}")]
    SynthesisError(franklin_crypto::bellman::SynthesisError),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("synthesis error: {0}")]
    SynthesisError(#[from] franklin_crypto::bellman::SynthesisError),

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("zkSync error: {0}")]
    ZkSyncClient(#[from] zksync::error::ClientError),

    #[error("internal error in virtual machine: {0}")]
    InternalError(String),

    #[error("malformed bytecode: {0}")]
    MalformedBytecode(#[from] MalformedBytecode),

    #[error("require error: {0}")]
    RequireError(String),

    #[error(
        "index out of bounds: expected index in range {lower_bound}..{upper_bound}, found {found}"
    )]
    IndexOutOfBounds {
        lower_bound: usize,
        upper_bound: usize,
        found: usize,
    },

    #[error("type error: expected {expected}, found {found}")]
    TypeError { expected: String, found: String },

    #[error("constant value expected, found variable (witness)")]
    ExpectedConstant,

    #[error("size is too large: {0}")]
    ExpectedUsize(BigInt),

    #[error("value overflow or constraint violation")]
    UnsatisfiedConstraint,

    #[error("division by zero")]
    DivisionByZero,

    #[error("inverting zero")]
    ZeroInversion,

    #[error("type size mismatch: {0}")]
    TypeSize(#[from] TypeSizeError),

    #[error("overflow: value {value} is not in range of type {scalar_type}")]
    ValueOverflow {
        value: BigInt,
        scalar_type: zinc_types::ScalarType,
    },

    #[error("the unit test data is missing")]
    UnitTestDataMissing,

    #[error("the instruction is available only for contracts")]
    OnlyForContracts,

    #[error("invalid storage value")]
    InvalidStorageValue,

    #[error("contract {address} does not exist")]
    ContractNotFound { address: String },

    #[error("contract {address} already exists")]
    ContractAlreadyExists { address: String },

    #[error("contract instance {address} cannot be fetched twice")]
    ContractAlreadyFetched { address: String },

    #[error("contract method `{found}` does not exist")]
    MethodNotFound { found: String },
}
