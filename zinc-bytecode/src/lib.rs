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
    UTF8Error,
}

#[derive(Debug)]
pub enum InstructionCode {
    NoOperation,

    // Evalution Stack
    PushConst,
    Pop,

    // Data Stack
    Load,
    Store,
    LoadSequence,
    StoreSequence,
    LoadByIndex,
    StoreByIndex,
    LoadSequenceByIndex,
    StoreSequenceByIndex,

    LoadGlobal,
    LoadSequenceGlobal,
    LoadByIndexGlobal,
    LoadSequenceByIndexGlobal,

    StoreGlobal,
    StoreSequenceGlobal,

    Ref,
    RefStore,
    RefStoreSequence,

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
    If,
    Else,
    EndIf,
    ConditionalSelect,
    LoopBegin,
    LoopEnd,
    Call,
    Return,

    Assert,
    Log,

    Exit,

    MerkleInit,
    MerkleGet,
    MerkleSet,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    NoOperation(NoOperation),

    // Stack
    PushConst(PushConst),
    Pop(Pop),

    // Storage
    Load(Load),
    Store(Store),
    LoadSequence(LoadSequence),
    StoreSequence(StoreSequence),
    LoadByIndex(LoadByIndex),
    StoreByIndex(StoreByIndex),
    LoadSequenceByIndex(LoadSequenceByIndex),
    StoreSequenceByIndex(StoreSequenceByIndex),
    LoadGlobal(LoadGlobal),
    LoadSequenceGlobal(LoadSequenceGlobal),
    LoadByIndexGlobal(LoadByIndexGlobal),
    LoadSequenceByIndexGlobal(LoadSequenceByIndexGlobal),
    StoreGlobal(StoreGlobal),
    StoreSequenceGlobal(StoreSequenceGlobal),

    Ref(Ref),
    RefStore(RefStore),
    RefStoreSequence(RefStoreSequence),

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
    If(If),
    Else(Else),
    EndIf(EndIf),
    ConditionalSelect(ConditionalSelect),
    LoopBegin(LoopBegin),
    LoopEnd(LoopEnd),
    Call(Call),
    Return(Return),

    // Condition utils
    Assert(Assert),
    Log(Dbg),

    Exit(Exit),

    MerkleInit(MerkleInit),
    MerkleGet(MerkleGet),
    MerkleSet(MerkleSet),
}

/// Useful macro to avoid duplicating `match` constructions.
///
/// ```
/// # use zinc_bytecode::{dispatch_instruction, Instruction, InstructionInfo, NoOperation};
/// # use zinc_bytecode::instructions::Add;
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
            Instruction::Load($pattern) => $expression,
            Instruction::Store($pattern) => $expression,
            Instruction::LoadSequence($pattern) => $expression,
            Instruction::StoreSequence($pattern) => $expression,
            Instruction::LoadByIndex($pattern) => $expression,
            Instruction::StoreByIndex($pattern) => $expression,
            Instruction::LoadSequenceByIndex($pattern) => $expression,
            Instruction::StoreSequenceByIndex($pattern) => $expression,
            Instruction::LoadGlobal($pattern) => $expression,
            Instruction::LoadSequenceGlobal($pattern) => $expression,
            Instruction::LoadByIndexGlobal($pattern) => $expression,
            Instruction::LoadSequenceByIndexGlobal($pattern) => $expression,
            Instruction::StoreGlobal($pattern) => $expression,
            Instruction::StoreSequenceGlobal($pattern) => $expression,
            Instruction::Ref($pattern) => $expression,
            Instruction::RefStore($pattern) => $expression,
            Instruction::RefStoreSequence($pattern) => $expression,


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

            Instruction::If($pattern) => $expression,
            Instruction::Else($pattern) => $expression,
            Instruction::EndIf($pattern) => $expression,
            Instruction::ConditionalSelect($pattern) => $expression,
            Instruction::LoopBegin($pattern) => $expression,
            Instruction::LoopEnd($pattern) => $expression,
            Instruction::Call($pattern) => $expression,
            Instruction::Return($pattern) => $expression,

            Instruction::Assert($pattern) => $expression,
            Instruction::Log($pattern) => $expression,

            Instruction::Exit($pattern) => $expression,

            Instruction::MerkleInit($pattern) => $expression,
            Instruction::MerkleGet($pattern) => $expression,
            Instruction::MerkleSet($pattern) => $expression,
        }
    };
}
