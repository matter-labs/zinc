//!
//! The expression operand.
//!

use crate::syntax::tree::expression::array::Expression as ArrayExpression;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::expression::conditional::Expression as ConditionalExpression;
use crate::syntax::tree::expression::r#match::Expression as MatchExpression;
use crate::syntax::tree::expression::structure::Expression as StructureExpression;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::expression::tuple::Expression as TupleExpression;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::literal::string::Literal as StringLiteral;
use crate::syntax::tree::member_integer::MemberInteger;
use crate::syntax::tree::member_string::MemberString;
use crate::syntax::tree::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Unit,
    LiteralBoolean(BooleanLiteral),
    LiteralInteger(IntegerLiteral),
    LiteralString(StringLiteral),
    MemberInteger(MemberInteger),
    MemberString(MemberString),
    Identifier(Identifier),
    Type(Type),
    Array(ArrayExpression),
    Tuple(TupleExpression),
    Structure(StructureExpression),
    List(Vec<ExpressionTree>),
    Block(BlockExpression),
    Conditional(ConditionalExpression),
    Match(MatchExpression),
    Inner(Box<ExpressionTree>),
}
