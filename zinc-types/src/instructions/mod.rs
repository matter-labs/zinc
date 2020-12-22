//!
//! The bytecode instruction.
//!

pub mod call_library;
pub mod contract;
pub mod data_stack;
pub mod dbg;
pub mod evaluation_stack;
pub mod flow;
pub mod marker;
pub mod noop;
pub mod operator;
pub mod require;

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use self::call_library::CallLibrary;
use self::contract::fetch::StorageFetch;
use self::contract::init::StorageInit;
use self::contract::load::StorageLoad;
use self::contract::store::StorageStore;
use self::data_stack::load::Load;
use self::data_stack::load_by_index::LoadByIndex;
use self::data_stack::store::Store;
use self::data_stack::store_by_index::StoreByIndex;
use self::dbg::Dbg;
use self::evaluation_stack::copy::Copy;
use self::evaluation_stack::push::Push;
use self::evaluation_stack::slice::Slice;
use self::flow::call::Call;
use self::flow::loop_begin::LoopBegin;
use self::flow::loop_end::LoopEnd;
use self::flow::r#else::Else;
use self::flow::r#endif::EndIf;
use self::flow::r#if::If;
use self::flow::r#return::Return;
use self::marker::column::ColumnMarker;
use self::marker::file::FileMarker;
use self::marker::function::FunctionMarker;
use self::marker::line::LineMarker;
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
use self::require::Require;

///
/// The bytecode instruction.
///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    /// The no-operation instruction.
    NoOperation(NoOperation),

    /// An evaluation stack instruction.
    Push(Push),
    /// An evaluation stack instruction.
    Slice(Slice),
    /// An evaluation stack instruction.
    Copy(Copy),

    /// A data stack instruction.
    Load(Load),
    /// A data stack instruction.
    LoadByIndex(LoadByIndex),
    /// A data stack instruction.
    Store(Store),
    /// A data stack instruction.
    StoreByIndex(StoreByIndex),

    /// A contract storage instruction.
    StorageInit(StorageInit),
    /// A contract storage instruction.
    StorageFetch(StorageFetch),
    /// A contract storage instruction.
    StorageStore(StorageStore),
    /// A contract storage instruction.
    StorageLoad(StorageLoad),

    /// An arithmetic operator instruction.
    Add(Add),
    /// An arithmetic operator instruction.
    Sub(Sub),
    /// An arithmetic operator instruction.
    Mul(Mul),
    /// An arithmetic operator instruction.
    Div(Div),
    /// An arithmetic operator instruction.
    Rem(Rem),
    /// An arithmetic operator instruction.
    Neg(Neg),

    /// A logical operator instruction.
    Not(Not),
    /// A logical operator instruction.
    And(And),
    /// A logical operator instruction.
    Or(Or),
    /// A logical operator instruction.
    Xor(Xor),

    /// A comparison operator instruction.
    Lt(Lt),
    /// A comparison operator instruction.
    Le(Le),
    /// A comparison operator instruction.
    Eq(Eq),
    /// A comparison operator instruction.
    Ne(Ne),
    /// A comparison operator instruction.
    Ge(Ge),
    /// A comparison operator instruction.
    Gt(Gt),

    /// A bitwise operator instruction.
    BitwiseShiftLeft(BitwiseShiftLeft),
    /// A bitwise operator instruction.
    BitwiseShiftRight(BitwiseShiftRight),
    /// A bitwise operator instruction.
    BitwiseAnd(BitwiseAnd),
    /// A bitwise operator instruction.
    BitwiseOr(BitwiseOr),
    /// A bitwise operator instruction.
    BitwiseXor(BitwiseXor),
    /// A bitwise operator instruction.
    BitwiseNot(BitwiseNot),

    /// The cast operator instruction.
    Cast(Cast),

    /// A flow control instruction.
    If(If),
    /// A flow control instruction.
    Else(Else),
    /// A flow control instruction.
    EndIf(EndIf),
    /// A flow control instruction.
    LoopBegin(LoopBegin),
    /// A flow control instruction.
    LoopEnd(LoopEnd),
    /// A flow control instruction.
    Call(Call),
    /// A flow control instruction.
    Return(Return),

    /// An intrinsic function call instruction.
    Dbg(Dbg),
    /// An intrinsic function call instruction.
    Require(Require),
    /// The standard library function call instruction.
    CallLibrary(CallLibrary),

    /// A debug location marker instruction.
    FileMarker(FileMarker),
    /// A debug location marker instruction.
    FunctionMarker(FunctionMarker),
    /// A debug location marker instruction.
    LineMarker(LineMarker),
    /// A debug location marker instruction.
    ColumnMarker(ColumnMarker),
}

impl Instruction {
    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        match self {
            Self::NoOperation(inner) => inner.is_debug(),

            Self::Push(inner) => inner.is_debug(),
            Self::Slice(inner) => inner.is_debug(),
            Self::Copy(inner) => inner.is_debug(),

            Self::Load(inner) => inner.is_debug(),
            Self::LoadByIndex(inner) => inner.is_debug(),
            Self::Store(inner) => inner.is_debug(),
            Self::StoreByIndex(inner) => inner.is_debug(),

            Self::StorageInit(inner) => inner.is_debug(),
            Self::StorageFetch(inner) => inner.is_debug(),
            Self::StorageStore(inner) => inner.is_debug(),
            Self::StorageLoad(inner) => inner.is_debug(),

            Self::Add(inner) => inner.is_debug(),
            Self::Sub(inner) => inner.is_debug(),
            Self::Mul(inner) => inner.is_debug(),
            Self::Div(inner) => inner.is_debug(),
            Self::Rem(inner) => inner.is_debug(),
            Self::Neg(inner) => inner.is_debug(),

            Self::Not(inner) => inner.is_debug(),
            Self::And(inner) => inner.is_debug(),
            Self::Or(inner) => inner.is_debug(),
            Self::Xor(inner) => inner.is_debug(),

            Self::Lt(inner) => inner.is_debug(),
            Self::Le(inner) => inner.is_debug(),
            Self::Eq(inner) => inner.is_debug(),
            Self::Ne(inner) => inner.is_debug(),
            Self::Ge(inner) => inner.is_debug(),
            Self::Gt(inner) => inner.is_debug(),

            Self::BitwiseShiftLeft(inner) => inner.is_debug(),
            Self::BitwiseShiftRight(inner) => inner.is_debug(),
            Self::BitwiseAnd(inner) => inner.is_debug(),
            Self::BitwiseOr(inner) => inner.is_debug(),
            Self::BitwiseXor(inner) => inner.is_debug(),
            Self::BitwiseNot(inner) => inner.is_debug(),

            Self::Cast(inner) => inner.is_debug(),

            Self::If(inner) => inner.is_debug(),
            Self::Else(inner) => inner.is_debug(),
            Self::EndIf(inner) => inner.is_debug(),
            Self::LoopBegin(inner) => inner.is_debug(),
            Self::LoopEnd(inner) => inner.is_debug(),
            Self::Call(inner) => inner.is_debug(),
            Self::Return(inner) => inner.is_debug(),

            Self::Dbg(inner) => inner.is_debug(),
            Self::Require(inner) => inner.is_debug(),
            Self::CallLibrary(inner) => inner.is_debug(),

            Self::FileMarker(inner) => inner.is_debug(),
            Self::FunctionMarker(inner) => inner.is_debug(),
            Self::LineMarker(inner) => inner.is_debug(),
            Self::ColumnMarker(inner) => inner.is_debug(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoOperation(inner) => write!(f, "{}", inner),

            Self::Push(inner) => write!(f, "{}", inner),
            Self::Slice(inner) => write!(f, "{}", inner),
            Self::Copy(inner) => write!(f, "{}", inner),

            Self::Load(inner) => write!(f, "{}", inner),
            Self::LoadByIndex(inner) => write!(f, "{}", inner),
            Self::Store(inner) => write!(f, "{}", inner),
            Self::StoreByIndex(inner) => write!(f, "{}", inner),

            Self::StorageInit(inner) => write!(f, "{}", inner),
            Self::StorageFetch(inner) => write!(f, "{}", inner),
            Self::StorageStore(inner) => write!(f, "{}", inner),
            Self::StorageLoad(inner) => write!(f, "{}", inner),

            Self::Add(inner) => write!(f, "{}", inner),
            Self::Sub(inner) => write!(f, "{}", inner),
            Self::Mul(inner) => write!(f, "{}", inner),
            Self::Div(inner) => write!(f, "{}", inner),
            Self::Rem(inner) => write!(f, "{}", inner),
            Self::Neg(inner) => write!(f, "{}", inner),

            Self::Not(inner) => write!(f, "{}", inner),
            Self::And(inner) => write!(f, "{}", inner),
            Self::Or(inner) => write!(f, "{}", inner),
            Self::Xor(inner) => write!(f, "{}", inner),

            Self::Lt(inner) => write!(f, "{}", inner),
            Self::Le(inner) => write!(f, "{}", inner),
            Self::Eq(inner) => write!(f, "{}", inner),
            Self::Ne(inner) => write!(f, "{}", inner),
            Self::Ge(inner) => write!(f, "{}", inner),
            Self::Gt(inner) => write!(f, "{}", inner),

            Self::BitwiseShiftLeft(inner) => write!(f, "{}", inner),
            Self::BitwiseShiftRight(inner) => write!(f, "{}", inner),
            Self::BitwiseAnd(inner) => write!(f, "{}", inner),
            Self::BitwiseOr(inner) => write!(f, "{}", inner),
            Self::BitwiseXor(inner) => write!(f, "{}", inner),
            Self::BitwiseNot(inner) => write!(f, "{}", inner),

            Self::Cast(inner) => write!(f, "{}", inner),

            Self::If(inner) => write!(f, "{}", inner),
            Self::Else(inner) => write!(f, "{}", inner),
            Self::EndIf(inner) => write!(f, "{}", inner),
            Self::LoopBegin(inner) => write!(f, "{}", inner),
            Self::LoopEnd(inner) => write!(f, "{}", inner),
            Self::Call(inner) => write!(f, "{}", inner),
            Self::Return(inner) => write!(f, "{}", inner),

            Self::Dbg(inner) => write!(f, "{}", inner),
            Self::Require(inner) => write!(f, "{}", inner),
            Self::CallLibrary(inner) => write!(f, "{}", inner),

            Self::FileMarker(inner) => write!(f, "{}", inner),
            Self::FunctionMarker(inner) => write!(f, "{}", inner),
            Self::LineMarker(inner) => write!(f, "{}", inner),
            Self::ColumnMarker(inner) => write!(f, "{}", inner),
        }
    }
}
