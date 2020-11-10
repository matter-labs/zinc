//!
//! The literal semantic analyzer.
//!

use std::convert::TryFrom;

use zinc_syntax::BooleanLiteral;
use zinc_syntax::IntegerLiteral;
use zinc_syntax::StringLiteral;

use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::string::String as StringConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

///
/// The literal semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the boolean literal.
    ///
    /// Returns the semantic element and the intermediate representation if it is available.
    ///
    pub fn boolean(
        literal: BooleanLiteral,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        let constant = Constant::Boolean(BooleanConstant::from(literal));

        let intermediate = GeneratorConstant::try_from_semantic(&constant)
            .map(GeneratorExpressionOperand::Constant);
        let element = Element::Constant(constant);

        Ok((element, intermediate))
    }

    ///
    /// Analyzes the integer literal.
    ///
    /// Returns the semantic element and the intermediate representation if it is available.
    ///
    pub fn integer(
        literal: IntegerLiteral,
    ) -> Result<(Element, Option<GeneratorExpressionOperand>), Error> {
        let constant = IntegerConstant::try_from(&literal).map(Constant::Integer)?;

        let intermediate = GeneratorConstant::try_from_semantic(&constant)
            .map(GeneratorExpressionOperand::Constant);
        let element = Element::Constant(constant);

        Ok((element, intermediate))
    }

    ///
    /// Converts the syntax string literal to a semantic string literal.
    ///
    pub fn string(literal: StringLiteral) -> Result<Element, Error> {
        Ok(Element::Constant(Constant::String(StringConstant::new(
            literal.location,
            literal.into(),
        ))))
    }
}
