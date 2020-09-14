//!
//! The expression operand.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::array::Expression as ArrayExpression;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::expression::conditional::Expression as ConditionalExpression;
use crate::syntax::tree::expression::list::Expression as ListExpression;
use crate::syntax::tree::expression::r#match::Expression as MatchExpression;
use crate::syntax::tree::expression::structure::Expression as StructureExpression;
use crate::syntax::tree::expression::tuple::Expression as TupleExpression;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::literal::string::Literal as StringLiteral;
use crate::syntax::tree::r#type::Type;
use crate::syntax::tree::tuple_index::TupleIndex;

///
/// An expression tree operand.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// A unit value `()`.
    LiteralUnit(Location),
    /// `true` or `false`.
    LiteralBoolean(BooleanLiteral),
    /// `42`, `0x101010`, etc.
    LiteralInteger(IntegerLiteral),
    /// "Zinc is the best language for ZKP".
    LiteralString(StringLiteral),
    /// A tuple field identifier.
    TupleIndex(TupleIndex),
    /// An item identifier.
    Identifier(Identifier),
    /// A syntax type, e.g. a keyword, array, tuple, etc.
    Type(Type),
    /// An array literal expression.
    Array(ArrayExpression),
    /// A tuple literal expression.
    Tuple(TupleExpression),
    /// A structure literal expression.
    Structure(StructureExpression),
    /// A function argument list expression.
    List(ListExpression),
    /// A block expression `{ ... }`.
    Block(BlockExpression),
    /// A conditional expression `if x { ... } else { ... }`.
    Conditional(ConditionalExpression),
    /// A match expression `match value { 1 => 10, _ => 42 }`.
    Match(MatchExpression),
}
