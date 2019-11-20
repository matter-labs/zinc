//!
//! The statement parser.
//!

mod r#enum;
mod r#fn;
mod inner;
mod r#let;
mod r#loop;
mod module;
mod outer;
mod r#struct;
mod r#type;
mod r#use;

pub use self::inner::Parser as InnerStatementParser;
pub use self::module::Parser as ModParser;
pub use self::outer::Parser as OuterStatementParser;
pub use self::r#enum::Parser as EnumParser;
pub use self::r#fn::Parser as FnParser;
pub use self::r#let::Parser as LetParser;
pub use self::r#loop::Parser as LoopParser;
pub use self::r#struct::Parser as StructParser;
pub use self::r#type::Parser as TypeParser;
pub use self::r#use::Parser as UseParser;
