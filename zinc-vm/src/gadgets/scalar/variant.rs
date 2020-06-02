use crate::gadgets::scalar::constant::Constant;
use crate::gadgets::scalar::variable::Variable;
use crate::IEngine;

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
