pub mod constant;
pub mod variable;

use crate::IEngine;

use self::constant::Constant;
use self::variable::Variable;

#[derive(Debug, Clone)]
pub enum Variant<E: IEngine> {
    Constant(Constant<E>),
    Variable(Variable<E>),
}

impl<E: IEngine> From<Constant<E>> for Variant<E> {
    fn from(constant: Constant<E>) -> Self {
        Self::Constant(constant)
    }
}

impl<E: IEngine> From<Variable<E>> for Variant<E> {
    fn from(variable: Variable<E>) -> Self {
        Self::Variable(variable)
    }
}
