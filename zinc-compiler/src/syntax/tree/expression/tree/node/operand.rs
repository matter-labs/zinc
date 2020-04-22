//!
//! The expression operand.
//!

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

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// a unit value `()`
    LiteralUnit,
    /// `true` or `false`
    LiteralBoolean(BooleanLiteral),
    /// `42`, `0x101010`, etc.
    LiteralInteger(IntegerLiteral),
    /// "Zinc is the best language for ZKP"
    LiteralString(StringLiteral),
    /// a tuple field identifier
    TupleIndex(TupleIndex),
    /// an item identifier
    Identifier(Identifier),
    /// a syntax type, e.g. a keyword, array, tuple, etc.
    Type(Type),
    /// an array literal expression
    Array(ArrayExpression),
    /// a tuple literal expression
    Tuple(TupleExpression),
    /// a structure literal expression
    Structure(StructureExpression),
    /// a function argument list expression
    List(ListExpression),
    /// a block expression `{ ... }`
    Block(BlockExpression),
    /// a conditional expression `if x { ... } else { ... }`
    Conditional(ConditionalExpression),
    /// a match expression `match value { 1 => 10, _ => 42 }`
    Match(MatchExpression),
}
