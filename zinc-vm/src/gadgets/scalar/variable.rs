use crate::IEngine;

#[derive(Debug, Clone)]
pub struct Variable<E: IEngine> {
    pub value: Option<E::Fr>,
    pub variable: bellman::Variable,
}
