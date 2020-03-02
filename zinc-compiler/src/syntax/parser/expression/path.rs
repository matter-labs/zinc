//!
//! The path expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::terminal::Parser as TerminalOperandParser;
use crate::syntax::tree::expression::auxiliary::Auxiliary as ExpressionAuxiliary;
use crate::syntax::tree::expression::builder::Builder as ExpressionBuilder;
use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::Expression;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Terminal,
    DoubleColonOrExclamationMarkOrEnd,
}

impl Default for State {
    fn default() -> Self {
        State::Terminal
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Expression, Option<Token>), Error> {
        loop {
            match self.state {
                State::Terminal => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        token => {
                            let (expression, next) = TerminalOperandParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.set_location_if_unset(expression.location);
                            self.builder.extend_with_expression(expression);
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::DoubleColonOrExclamationMarkOrEnd;
                        }
                    }
                }
                State::DoubleColonOrExclamationMarkOrEnd => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleColon),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Path));
                            self.state = State::Terminal;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                            location,
                        } => {
                            self.builder
                                .push_auxiliary(location, ExpressionAuxiliary::Instruction);
                            return Ok((self.builder.finish(), None));
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::identifier::Identifier;

    #[test]
    fn ok() {
        let input = r#"mega::ultra::namespace;"#;

        let expected =
            Ok((
                Expression::new(
                    Location::new(1, 1),
                    vec![
                        ExpressionElement::new(
                            Location::new(1, 1),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 1), "mega".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 7),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 7), "ultra".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 5),
                            ExpressionObject::Operator(ExpressionOperator::Path),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 14),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 14), "namespace".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 12),
                            ExpressionObject::Operator(ExpressionOperator::Path),
                        ),
                    ],
                ),
                Some(Token::new(
                    Lexeme::Symbol(Symbol::Semicolon),
                    Location::new(1, 23),
                )),
            ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
