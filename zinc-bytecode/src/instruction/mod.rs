//!
//! The instruction.
//!

pub mod assert;
pub mod call_std;
pub mod contracts;
pub mod dbg;
pub mod flow;
pub mod markers;
pub mod memory;
pub mod noop;
pub mod operator;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use self::assert::Assert;
use self::call_std::CallStd;
use self::contracts::load::StorageLoad;
use self::contracts::store::StorageStore;
use self::dbg::Dbg;
use self::flow::call::Call;
use self::flow::exit::Exit;
use self::flow::loop_begin::LoopBegin;
use self::flow::loop_end::LoopEnd;
use self::flow::r#else::Else;
use self::flow::r#endif::EndIf;
use self::flow::r#if::If;
use self::flow::ret::Return;
use self::markers::column::ColumnMarker;
use self::markers::file::FileMarker;
use self::markers::function::FunctionMarker;
use self::markers::line::LineMarker;
use self::memory::copy::Copy;
use self::memory::load::Load;
use self::memory::load_by_index::LoadByIndex;
use self::memory::push_const::PushConst;
use self::memory::slice::Slice;
use self::memory::store::Store;
use self::memory::store_by_index::StoreByIndex;
use self::noop::NoOperation;
use self::operator::arithmetic::add::Add;
use self::operator::arithmetic::div::Div;
use self::operator::arithmetic::mul::Mul;
use self::operator::arithmetic::neg::Neg;
use self::operator::arithmetic::rem::Rem;
use self::operator::arithmetic::sub::Sub;
use self::operator::bitwise::and::BitwiseAnd;
use self::operator::bitwise::not::BitwiseNot;
use self::operator::bitwise::or::BitwiseOr;
use self::operator::bitwise::shift_left::BitwiseShiftLeft;
use self::operator::bitwise::shift_right::BitwiseShiftRight;
use self::operator::bitwise::xor::BitwiseXor;
use self::operator::cast::Cast;
use self::operator::comparison::eq::Eq;
use self::operator::comparison::ge::Ge;
use self::operator::comparison::gt::Gt;
use self::operator::comparison::le::Le;
use self::operator::comparison::lt::Lt;
use self::operator::comparison::ne::Ne;
use self::operator::logical::and::And;
use self::operator::logical::not::Not;
use self::operator::logical::or::Or;
use self::operator::logical::xor::Xor;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    NoOperation(NoOperation),

    // evaluation stack
    PushConst(PushConst),
    Slice(Slice),
    Copy(Copy),

    // data stack
    Load(Load),
    LoadByIndex(LoadByIndex),
    Store(Store),
    StoreByIndex(StoreByIndex),

    // contract storage
    StorageStore(StorageStore),
    StorageLoad(StorageLoad),

    // arithmetic operations
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Rem(Rem),
    Neg(Neg),

    // logical operations
    Not(Not),
    And(And),
    Or(Or),
    Xor(Xor),

    // comparison operations
    Lt(Lt),
    Le(Le),
    Eq(Eq),
    Ne(Ne),
    Ge(Ge),
    Gt(Gt),

    // bitwise operations
    BitwiseShiftLeft(BitwiseShiftLeft),
    BitwiseShiftRight(BitwiseShiftRight),
    BitwiseAnd(BitwiseAnd),
    BitwiseOr(BitwiseOr),
    BitwiseXor(BitwiseXor),
    BitwiseNot(BitwiseNot),

    // casting operation
    Cast(Cast),

    // flow control
    If(If),
    Else(Else),
    EndIf(EndIf),
    LoopBegin(LoopBegin),
    LoopEnd(LoopEnd),
    Call(Call),
    Return(Return),
    Exit(Exit),

    // built-in function calls
    CallStd(CallStd),
    Assert(Assert),
    Dbg(Dbg),

    // debug location markers
    FileMarker(FileMarker),
    FunctionMarker(FunctionMarker),
    LineMarker(LineMarker),
    ColumnMarker(ColumnMarker),
}

/// Useful macro to avoid duplicating `match` constructions.
///
/// ```
/// # use zinc_bytecode::dispatch_instruction;
/// # use zinc_bytecode::Instruction;
/// # use zinc_bytecode::InstructionInfo;
/// # use zinc_bytecode::NoOperation;
///
/// let instruction = NoOperation.wrap();
/// let assembly = dispatch_instruction!(instruction => instruction.to_assembly());
/// assert_eq!(assembly, "noop");
/// ```
#[macro_export]
macro_rules! dispatch_instruction {
    ($pattern:ident => $expression:expr) => {
        match $pattern {
            Instruction::NoOperation($pattern) => $expression,

            Instruction::PushConst($pattern) => $expression,
            Instruction::Slice($pattern) => $expression,
            Instruction::Copy($pattern) => $expression,

            Instruction::Load($pattern) => $expression,
            Instruction::LoadByIndex($pattern) => $expression,
            Instruction::Store($pattern) => $expression,
            Instruction::StoreByIndex($pattern) => $expression,

            Instruction::StorageStore($pattern) => $expression,
            Instruction::StorageLoad($pattern) => $expression,

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

            Instruction::BitwiseShiftLeft($pattern) => $expression,
            Instruction::BitwiseShiftRight($pattern) => $expression,
            Instruction::BitwiseAnd($pattern) => $expression,
            Instruction::BitwiseOr($pattern) => $expression,
            Instruction::BitwiseXor($pattern) => $expression,
            Instruction::BitwiseNot($pattern) => $expression,

            Instruction::Cast($pattern) => $expression,

            Instruction::If($pattern) => $expression,
            Instruction::Else($pattern) => $expression,
            Instruction::EndIf($pattern) => $expression,
            Instruction::LoopBegin($pattern) => $expression,
            Instruction::LoopEnd($pattern) => $expression,
            Instruction::Call($pattern) => $expression,
            Instruction::Return($pattern) => $expression,
            Instruction::Exit($pattern) => $expression,

            Instruction::CallStd($pattern) => $expression,
            Instruction::Assert($pattern) => $expression,
            Instruction::Dbg($pattern) => $expression,

            Instruction::FileMarker($pattern) => $expression,
            Instruction::FunctionMarker($pattern) => $expression,
            Instruction::LineMarker($pattern) => $expression,
            Instruction::ColumnMarker($pattern) => $expression,
        }
    };
}
