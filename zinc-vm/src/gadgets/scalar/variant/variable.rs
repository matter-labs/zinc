use crate::IEngine;

#[derive(Debug, Clone)]
pub struct Variable<E: IEngine> {
    pub value: Option<E::Fr>,
    pub variable: franklin_crypto::bellman::Variable,
}

impl<E: IEngine> Variable<E> {
    pub fn new_unchecked(
        value: Option<E::Fr>,
        variable: franklin_crypto::bellman::Variable,
    ) -> Self {
        Self { value, variable }
    }
}
