//!
//! The statement parser.
//!

mod r#const;
mod r#enum;
mod extern_fn;
mod r#fn;
mod r#impl;
mod r#let;
mod local_function;
mod local_implementation;
mod local_module;
mod r#loop;
mod module;
mod r#static;
mod r#struct;
mod r#type;
mod r#use;

pub use self::extern_fn::Parser as ExternFnParser;
pub use self::local_function::Parser as FunctionLocalStatementParser;
pub use self::local_implementation::Parser as ImplementationLocalStatementParser;
pub use self::local_module::Parser as ModuleLocalStatementParser;
pub use self::module::Parser as ModParser;
pub use self::r#const::Parser as ConstParser;
pub use self::r#enum::Parser as EnumParser;
pub use self::r#fn::Parser as FnParser;
pub use self::r#impl::Parser as ImplParser;
pub use self::r#let::Parser as LetParser;
pub use self::r#loop::Parser as LoopParser;
pub use self::r#static::Parser as StaticParser;
pub use self::r#struct::Parser as StructParser;
pub use self::r#type::Parser as TypeParser;
pub use self::r#use::Parser as UseParser;
