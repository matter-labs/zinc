//!
//! The identifier semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::operand::Operand as GeneratorOperand;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::path::Translator as PathTranslator;
use crate::semantic::element::path::Path;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::Identifier;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        translation_hint: TranslationHint,
    ) -> Result<(Element, Option<GeneratorOperand>), Error> {
        let location = identifier.location;

        let path = Path::new(location, identifier.into());

        PathTranslator::translate(scope, &path, translation_hint)
    }
}
