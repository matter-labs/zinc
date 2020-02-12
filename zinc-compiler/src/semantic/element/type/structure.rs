//!
//! The semantic analyzer structure type element.
//!

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::scope::Scope;

#[derive(Debug, Clone)]
pub struct Structure {
    pub identifier: String,
    pub unique_id: usize,
    pub fields: Vec<(String, Type)>,
    pub scope: Rc<RefCell<Scope>>,
}

impl Structure {
    pub fn new(
        identifier: String,
        unique_id: usize,
        fields: Vec<(String, Type)>,
        scope_parent: Option<Rc<RefCell<Scope>>>,
    ) -> Self {
        let scope = Rc::new(RefCell::new(Scope::new(scope_parent)));

        let structure = Self {
            identifier,
            unique_id,
            fields,
            scope: scope.clone(),
        };
        scope
            .borrow_mut()
            .declare_self(Type::Structure(structure.clone()));

        structure
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "struct {} {{ {} }}",
            self.identifier,
            self.fields
                .iter()
                .map(|(name, r#type)| format!("{}: {}", name, r#type))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
