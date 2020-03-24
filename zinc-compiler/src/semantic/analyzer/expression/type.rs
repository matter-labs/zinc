//!
//! The type semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::Type as SyntaxType;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(scope: Rc<RefCell<Scope>>, r#type: SyntaxType) -> Result<Element, Error> {
        Type::from_type_variant(&r#type.variant, scope).map(Element::Type)
    }
}
