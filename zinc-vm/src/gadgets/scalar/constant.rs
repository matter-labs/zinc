use crate::IEngine;

#[derive(Debug, Clone)]
pub struct Constant<E: IEngine> {
    pub value: E::Fr,
}
