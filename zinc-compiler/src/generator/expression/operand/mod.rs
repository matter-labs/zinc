//!
//! The generator expression operand.
//!

pub mod array;
pub mod block;
pub mod conditional;
pub mod constant;
pub mod group;
pub mod r#match;
pub mod variable;

use self::array::Expression as ArrayExpression;
use self::block::Expression as BlockExpression;
use self::conditional::Expression as ConditionalExpression;
use self::constant::Constant;
use self::group::Expression as GroupExpression;
use self::r#match::Expression as MatchExpression;
use self::variable::Variable;

#[derive(Debug, Clone)]
pub enum Operand {
    Constant(Constant),
    Variable(Variable),
    Array(ArrayExpression),
    Group(GroupExpression),
    Block(BlockExpression),
    Conditional(ConditionalExpression),
    Match(MatchExpression),
}
