//!
//! The expression.
//!

mod array;
mod block;
mod builder;
mod conditional;
mod element;
mod object;
mod operand;
mod operator;
mod path;
mod structure;
mod tuple;

pub use self::array::Builder as ArrayExpressionBuilder;
pub use self::array::Expression as ArrayExpression;
pub use self::block::Builder as BlockExpressionBuilder;
pub use self::block::Expression as BlockExpression;
pub use self::builder::Builder as ExpressionBuilder;
pub use self::builder::Builder;
pub use self::conditional::Builder as ConditionalExpressionBuilder;
pub use self::conditional::Expression as ConditionalExpression;
pub use self::element::Element as ExpressionElement;
pub use self::element::Element;
pub use self::object::Object as ExpressionObject;
pub use self::object::Object;
pub use self::operand::Operand as ExpressionOperand;
pub use self::operand::Operand;
pub use self::operator::Operator as ExpressionOperator;
pub use self::operator::Operator;
pub use self::path::Builder as PathExpressionBuilder;
pub use self::path::Expression as PathExpression;
pub use self::structure::Builder as StructureExpressionBuilder;
pub use self::structure::Expression as StructureExpression;
pub use self::tuple::Builder as TupleExpressionBuilder;
pub use self::tuple::Expression as TupleExpression;

use std::fmt;

use crate::lexical::Location;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<Element>,
}

impl Expression {
    pub fn new(location: Location, elements: Vec<Element>) -> Self {
        Self { location, elements }
    }
}

impl IntoIterator for Expression {
    type Item = Element;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}
