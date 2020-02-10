use failure::Fail;
use franklin_crypto::bellman::SynthesisError;
use num_bigint::BigInt;

#[derive(Debug, Fail)]
pub enum TypeSizeError {
    #[fail(display = "expected input value of size {}, got {}", expected, actual)]
    Input { expected: usize, actual: usize },

    #[fail(display = "expected output value of size {}, got {}", expected, actual)]
    Output { expected: usize, actual: usize },
}

#[derive(Debug, Fail)]
pub enum MalformedBytecode {
    #[fail(display = "invalid arguments to built-in function: {}", _0)]
    InvalidArguments(String),

    #[fail(display = "unexpected `loop_end` instruction")]
    UnexpectedLoopEnd,

    #[fail(display = "unexpected `return` instruction")]
    UnexpectedReturn,

    #[fail(display = "unexpected `else` instruction")]
    UnexpectedElse,

    #[fail(display = "unexpected `end_if` instruction")]
    UnexpectedEndIf,

    #[fail(display = "stack underflow")]
    StackUnderflow,

    #[fail(display = "reading uninitialized memory")]
    UninitializedStorageAccess,

    #[fail(display = "conditional branches produced results of different sizes")]
    BranchStacksDoNotMatch,
}

#[derive(Debug, Fail)]
pub enum RuntimeError {
    #[fail(display = "synthesis error: {}", _0)]
    SynthesisError(SynthesisError),

    #[fail(display = "internal error in virtual machine: {}", _0)]
    InternalError(String),

    #[fail(display = "malformed bytecode: {}", _0)]
    MalformedBytecode(MalformedBytecode),

    #[fail(display = "assertion error: got false expression in `assert!`")]
    AssertionError,

    #[fail(
        display = "index out of bounds: expected index in range {}..{}, got {}",
        lower_bound, upper_bound, actual
    )]
    IndexOutOfBounds {
        lower_bound: usize,
        upper_bound: usize,
        actual: usize,
    },

    #[fail(display = "type error: expected {}, got {}", expected, actual)]
    TypeError { expected: String, actual: String },

    #[fail(display = "constant value expected, got variable (witness)")]
    ExpectedConstant,

    #[fail(display = "size is too large: {}", _0)]
    ExpectedUsize(BigInt),

    #[fail(display = "value overflow or constraint violation")]
    UnsatisfiedConstraint,

    #[fail(display = "division by zero")]
    ZeroDivisionError,

    #[fail(display = "type size mismatch: {}", _0)]
    TypeSize(TypeSizeError),
}

impl From<SynthesisError> for RuntimeError {
    fn from(error: SynthesisError) -> Self {
        RuntimeError::SynthesisError(error)
    }
}

impl From<MalformedBytecode> for RuntimeError {
    fn from(error: MalformedBytecode) -> Self {
        RuntimeError::MalformedBytecode(error)
    }
}

impl From<TypeSizeError> for RuntimeError {
    fn from(error: TypeSizeError) -> Self {
        RuntimeError::TypeSize(error)
    }
}
