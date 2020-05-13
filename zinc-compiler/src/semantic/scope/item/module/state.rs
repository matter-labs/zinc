//!
//! The semantic analyzer scope module item state.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::scope::Scope;
use crate::source::module::Module as SourceModule;

#[derive(Debug, Clone)]
pub enum State {
    /// Waiting to be resolved during the second pass
    Unresolved { inner: SourceModule },
    /// Resolved element ready to be used from anywhere
    Resolved { inner: Rc<RefCell<Scope>> },
}

impl State {
    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        match self {
            Self::Resolved { inner } => Scope::get_intermediate(inner.to_owned()),
            _ => panic!("BANG"),
        }
    }
}
