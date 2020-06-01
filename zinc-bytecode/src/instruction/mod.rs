//!
//! The instructions.
//!

pub mod assert;
pub mod call_std;
pub mod contract;
pub mod dbg;
pub mod evaluation_stack;
pub mod flow;
pub mod marker;
pub mod noop;
pub mod operator;
pub mod stack;

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use self::assert::Assert;
use self::call_std::CallStd;
use self::contract::load::StorageLoad;
use self::contract::store::StorageStore;
use self::dbg::Dbg;
use self::evaluation_stack::copy::Copy;
use self::evaluation_stack::push::Push;
use self::evaluation_stack::slice::Slice;
use self::flow::call::Call;
use self::flow::exit::Exit;
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
use self::stack::load::Load;
use self::stack::load_by_index::LoadByIndex;
use self::stack::store::Store;
use self::stack::store_by_index::StoreByIndex;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    NoOperation(NoOperation),

    // evaluation stack
    Push(Push),
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

impl Instruction {
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
            Self::Exit(inner) => inner.is_debug(),

            Self::CallStd(inner) => inner.is_debug(),
            Self::Assert(inner) => inner.is_debug(),
            Self::Dbg(inner) => inner.is_debug(),

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
            Self::Exit(inner) => write!(f, "{}", inner),

            Self::CallStd(inner) => write!(f, "{}", inner),
            Self::Assert(inner) => write!(f, "{}", inner),
            Self::Dbg(inner) => write!(f, "{}", inner),

            Self::FileMarker(inner) => write!(f, "{}", inner),
            Self::FunctionMarker(inner) => write!(f, "{}", inner),
            Self::LineMarker(inner) => write!(f, "{}", inner),
            Self::ColumnMarker(inner) => write!(f, "{}", inner),
        }
    }
}
