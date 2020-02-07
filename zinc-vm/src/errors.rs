use franklin_crypto::bellman::SynthesisError;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidArguments(String),
    StackUnderflow,
    StackOverflow,
    UnexpectedEndOfFile,
    SynthesisError(SynthesisError),
    InternalError(String),
    IntegerOverflow,
    UnexpectedLoopEnd,
    UnexpectedReturn,
    UnexpectedFrameExit,
    UnexpectedElse,
    UnexpectedEndIf,
    AssertionError,
    FirstInstructionNotCall,
    WrongInputsCount,
    StackIndexOutOfRange,
    UninitializedStorageAccess,
    MissingArgument,
    BranchStacksDoNotMatch,
    IndexOutOfBounds,
    MergingNonValueTypes,
    UnexpectedNonValueType,
    TypeError,
    ExpectedConstant,
    ExpectedUsize,
    UnsatisfiedConstraint,
    ZeroDivisionError,
}

impl From<SynthesisError> for RuntimeError {
    fn from(error: SynthesisError) -> Self {
        RuntimeError::SynthesisError(error)
    }
}
