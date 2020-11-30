//!
//! The expression operand.
//!

use std::fmt;

use zinc_lexical::Location;

use crate::tree::expression::array::Expression as ArrayExpression;
use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::expression::conditional::Expression as ConditionalExpression;
use crate::tree::expression::list::Expression as ListExpression;
use crate::tree::expression::r#match::Expression as MatchExpression;
use crate::tree::expression::structure::Expression as StructureExpression;
use crate::tree::expression::tuple::Expression as TupleExpression;
use crate::tree::identifier::Identifier;
use crate::tree::literal::boolean::Literal as BooleanLiteral;
use crate::tree::literal::integer::Literal as IntegerLiteral;
use crate::tree::literal::string::Literal as StringLiteral;
use crate::tree::r#type::Type;
use crate::tree::tuple_index::TupleIndex;

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

impl Operand {
    ///
    /// Checks if the tree is a single block, conditional, or match expression.
    ///
    /// Is used to allow not terminating such expression with a semicolon.
    ///
    pub fn can_be_unterminated(&self) -> bool {
        match self {
            Self::Block(_) => true,
            Self::Conditional(_) => true,
            Self::Match(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(inner) => write!(f, "{}", inner.name),
            _ => todo!(),
        }
    }
}
