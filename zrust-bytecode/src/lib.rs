mod decode;
pub mod instructions;
pub mod vlq;

pub use decode::*;
pub use instructions::*;

use std::fmt;

pub trait InstructionInfo: PartialEq + fmt::Debug + Sized {
    fn to_assembly(&self) -> String;
    fn code() -> InstructionCode;
    fn encode(&self) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError>;
    fn inputs_count(&self) -> usize;
    fn outputs_count(&self) -> usize;
    fn wrap(&self) -> Instruction;
}

#[derive(Debug, PartialEq)]
pub enum DecodingError {
    UnexpectedEOF,
    UnknownInstructionCode(u8),
    ConstantTooLong,
}

#[derive(Debug)]
pub enum InstructionCode {
    NoOperation,

    // Stack
    PushConst,
    Pop,
    CopyGlobal,

    // Storage
    LoadPush,
    PopStore,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,

    // Boolean
    Not,
    And,
    Or,
    Xor,

    // Comparison
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,

    Cast,

    // Flow control
    ConditionalSelect,
    LoopBegin,
    LoopEnd,
    Call,
    Return,

    // Condition utils
    Assert,
    Log,
    PushCondition,
    PopCondition,

    Exit,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    NoOperation(NoOperation),

    // Stack
    PushConst(PushConst),
    Pop(Pop),
    CopyGlobal(CopyGlobal),

    // Storage
    LoadPush(LoadPush),
    PopStore(PopStore),

    // Arithmetic
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Rem(Rem),
    Neg(Neg),

    // Boolean
    Not(Not),
    And(And),
    Or(Or),
    Xor(Xor),

    // Comparison
    Lt(Lt),
    Le(Le),
    Eq(Eq),
    Ne(Ne),
    Ge(Ge),
    Gt(Gt),

    Cast(Cast),

    // Flow control
    ConditionalSelect(ConditionalSelect),
    LoopBegin(LoopBegin),
    LoopEnd(LoopEnd),
    Call(Call),
    Return(Return),

    // Condition utils
    Assert(Assert),
    Log(Dbg),
    PushCondition(PushCondition),
    PopCondition(PopCondition),

    Exit(Exit),
}

/// Useful macro to avoid duplicating `match` constructions.
///
/// ```
/// # use zrust_bytecode::{dispatch_instruction, Instruction, InstructionInfo, NoOperation};
/// # use zrust_bytecode::instructions::Add;
/// let ins = NoOperation.wrap();
/// let assembly = dispatch_instruction!(ins => ins.to_assembly());
/// assert_eq!(assembly, "noop");
/// ```
#[macro_export]
macro_rules! dispatch_instruction {
    ($pattern:ident => $expression:expr) => {
        match $pattern {
            Instruction::NoOperation($pattern) => $expression,

            Instruction::PushConst($pattern) => $expression,
            Instruction::Pop($pattern) => $expression,
            Instruction::CopyGlobal($pattern) => $expression,
            Instruction::LoadPush($pattern) => $expression,
            Instruction::PopStore($pattern) => $expression,

            Instruction::Add($pattern) => $expression,
            Instruction::Sub($pattern) => $expression,
            Instruction::Mul($pattern) => $expression,
            Instruction::Div($pattern) => $expression,
            Instruction::Rem($pattern) => $expression,
            Instruction::Neg($pattern) => $expression,

            Instruction::Not($pattern) => $expression,
            Instruction::And($pattern) => $expression,
            Instruction::Or($pattern) => $expression,
            Instruction::Xor($pattern) => $expression,

            Instruction::Lt($pattern) => $expression,
            Instruction::Le($pattern) => $expression,
            Instruction::Eq($pattern) => $expression,
            Instruction::Ne($pattern) => $expression,
            Instruction::Ge($pattern) => $expression,
            Instruction::Gt($pattern) => $expression,

            Instruction::Cast($pattern) => $expression,

            Instruction::ConditionalSelect($pattern) => $expression,
            Instruction::LoopBegin($pattern) => $expression,
            Instruction::LoopEnd($pattern) => $expression,
            Instruction::Call($pattern) => $expression,
            Instruction::Return($pattern) => $expression,

            Instruction::Assert($pattern) => $expression,
            Instruction::Log($pattern) => $expression,
            Instruction::PushCondition($pattern) => $expression,
            Instruction::PopCondition($pattern) => $expression,

            Instruction::Exit($pattern) => $expression,
        }
    };
}
