//!
//! The Zinc VM bytecode library.
//!

pub(crate) mod data;
pub(crate) mod instruction;
pub(crate) mod program;

pub use self::data::r#type::scalar::integer::Type as IntegerType;
pub use self::data::r#type::scalar::Type as ScalarType;
pub use self::data::r#type::Type as DataType;
pub use self::data::value::JsonValueError as TemplateValueError;
pub use self::data::value::Value as TemplateValue;
pub use self::instruction::assert::Assert;
pub use self::instruction::call_std::BuiltinIdentifier;
pub use self::instruction::call_std::CallStd;
pub use self::instruction::contracts::load::StorageLoad;
pub use self::instruction::contracts::store::StorageStore;
pub use self::instruction::dbg::Dbg;
pub use self::instruction::flow::call::Call;
pub use self::instruction::flow::exit::Exit;
pub use self::instruction::flow::loop_begin::LoopBegin;
pub use self::instruction::flow::loop_end::LoopEnd;
pub use self::instruction::flow::r#else::Else;
pub use self::instruction::flow::r#endif::EndIf;
pub use self::instruction::flow::r#if::If;
pub use self::instruction::flow::ret::Return;
pub use self::instruction::markers::column::ColumnMarker;
pub use self::instruction::markers::file::FileMarker;
pub use self::instruction::markers::function::FunctionMarker;
pub use self::instruction::markers::line::LineMarker;
pub use self::instruction::memory::copy::Copy;
pub use self::instruction::memory::load::Load;
pub use self::instruction::memory::load_by_index::LoadByIndex;
pub use self::instruction::memory::push_const::Push;
pub use self::instruction::memory::slice::Slice;
pub use self::instruction::memory::store::Store;
pub use self::instruction::memory::store_by_index::StoreByIndex;
pub use self::instruction::noop::NoOperation;
pub use self::instruction::operator::arithmetic::add::Add;
pub use self::instruction::operator::arithmetic::div::Div;
pub use self::instruction::operator::arithmetic::mul::Mul;
pub use self::instruction::operator::arithmetic::neg::Neg;
pub use self::instruction::operator::arithmetic::rem::Rem;
pub use self::instruction::operator::arithmetic::sub::Sub;
pub use self::instruction::operator::bitwise::and::BitwiseAnd;
pub use self::instruction::operator::bitwise::not::BitwiseNot;
pub use self::instruction::operator::bitwise::or::BitwiseOr;
pub use self::instruction::operator::bitwise::shift_left::BitwiseShiftLeft;
pub use self::instruction::operator::bitwise::shift_right::BitwiseShiftRight;
pub use self::instruction::operator::bitwise::xor::BitwiseXor;
pub use self::instruction::operator::cast::Cast;
pub use self::instruction::operator::comparison::eq::Eq;
pub use self::instruction::operator::comparison::ge::Ge;
pub use self::instruction::operator::comparison::gt::Gt;
pub use self::instruction::operator::comparison::le::Le;
pub use self::instruction::operator::comparison::lt::Lt;
pub use self::instruction::operator::comparison::ne::Ne;
pub use self::instruction::operator::logical::and::And;
pub use self::instruction::operator::logical::not::Not;
pub use self::instruction::operator::logical::or::Or;
pub use self::instruction::operator::logical::xor::Xor;
pub use self::instruction::Instruction;
pub use self::program::Program;

use std::fmt;

pub trait InstructionInfo: fmt::Debug + Sized + PartialEq {
    fn to_assembly(&self) -> String;

    fn wrap(self) -> Instruction;
}
