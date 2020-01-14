//!
//! The expression.
//!

mod array;
mod auxiliary;
mod block;
mod builder;
mod conditional;
mod element;
mod r#match;
mod object;
mod operand;
mod operator;
mod structure;
mod tuple;

pub use self::array::Builder as ArrayExpressionBuilder;
pub use self::array::Expression as ArrayExpression;
pub use self::auxiliary::Auxiliary as ExpressionAuxiliary;
pub use self::block::Builder as BlockExpressionBuilder;
pub use self::block::Expression as BlockExpression;
pub use self::builder::Builder as ExpressionBuilder;
pub use self::conditional::Builder as ConditionalExpressionBuilder;
pub use self::conditional::Expression as ConditionalExpression;
pub use self::element::Element as ExpressionElement;
pub use self::object::Object as ExpressionObject;
pub use self::operand::Operand as ExpressionOperand;
pub use self::operator::Operator as ExpressionOperator;
pub use self::r#match::Builder as MatchExpressionBuilder;
pub use self::r#match::Expression as MatchExpression;
pub use self::structure::Builder as StructureExpressionBuilder;
pub use self::structure::Expression as StructureExpression;
pub use self::tuple::Builder as TupleExpressionBuilder;
pub use self::tuple::Expression as TupleExpression;

use crate::lexical::Location;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<ExpressionElement>,
}

impl Expression {
    pub fn new(location: Location, elements: Vec<ExpressionElement>) -> Self {
        Self { location, elements }
    }
}

impl IntoIterator for Expression {
    type Item = ExpressionElement;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}
