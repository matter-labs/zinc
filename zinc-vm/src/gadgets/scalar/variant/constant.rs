use crate::IEngine;

#[derive(Debug, Clone)]
pub struct Constant<E: IEngine> {
    pub value: E::Fr,
}

impl<E: IEngine> Constant<E> {
    pub fn new_fr(value: E::Fr) -> Self {
        Self { value }
    }
}
