//!
//! The parser library.
//!

#![allow(clippy::large_enum_variant)]

mod error;
mod lexical;
mod syntax;

pub use self::error::Error;
pub use self::lexical::BooleanLiteral;
pub use self::lexical::IntegerLiteral;
pub use self::lexical::Literal as InnerLiteral;
pub use self::lexical::Location;
pub use self::syntax::ArrayExpression;
pub use self::syntax::BlockExpression;
pub use self::syntax::CircuitProgram;
pub use self::syntax::ConditionalExpression;
pub use self::syntax::DebugStatement;
pub use self::syntax::EnumStatement;
pub use self::syntax::Expression;
pub use self::syntax::ExpressionObject;
pub use self::syntax::ExpressionOperand;
pub use self::syntax::ExpressionOperator;
pub use self::syntax::Field;
pub use self::syntax::FnStatement;
pub use self::syntax::Identifier;
pub use self::syntax::LetStatement;
pub use self::syntax::Literal;
pub use self::syntax::LoopStatement;
pub use self::syntax::MatchExpression;
pub use self::syntax::ModStatement;
pub use self::syntax::Parser;
pub use self::syntax::Pattern;
pub use self::syntax::PatternVariant;
pub use self::syntax::RequireStatement;
pub use self::syntax::Statement;
pub use self::syntax::StructStatement;
pub use self::syntax::StructureExpression;
pub use self::syntax::TupleExpression;
pub use self::syntax::Type;
pub use self::syntax::TypeStatement;
pub use self::syntax::TypeVariant;
pub use self::syntax::UseStatement;
pub use self::syntax::Variant;

pub const BITLENGTH_FIELD: usize = 254;
