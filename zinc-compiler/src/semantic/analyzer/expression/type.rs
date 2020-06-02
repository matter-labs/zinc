//!
//! The type semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::r#type::Type as SyntaxType;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Converts the syntax type to a semantic type.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, r#type: SyntaxType) -> Result<Element, Error> {
        Type::try_from_syntax(r#type, scope).map(Element::Type)
    }
}
