//!
//! The identifier semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::Identifier;

use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::analyzer::expression::path::Translator as PathTranslator;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::path::Path;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The identifier semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the identifier.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        identifier: Identifier,
        translation_rule: TranslationRule,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        let location = identifier.location;

        let path = Path::new(location, identifier);

        PathTranslator::translate(scope, path, translation_rule)
    }
}
