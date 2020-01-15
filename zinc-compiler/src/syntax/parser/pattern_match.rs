//!
//! The match pattern parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BooleanLiteral;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchPattern;
use crate::syntax::MatchPatternBuilder;
use crate::syntax::TerminalOperandParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Start,
    PathOperatorOrEnd,
    PathOperand,
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: MatchPatternBuilder,
    operator: Option<(Location, ExpressionOperator)>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(MatchPattern, Option<Token>), Error> {
        loop {
            match self.state {
                State::Start => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Literal(lexical::Literal::Boolean(boolean)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_boolean_literal(BooleanLiteral::new(location, boolean));
                            return Ok((self.builder.finish(), None));
                        }
                        Token {
                            lexeme: Lexeme::Literal(lexical::Literal::Integer(integer)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_integer_literal(IntegerLiteral::new(location, integer));
                            return Ok((self.builder.finish(), None));
                        }
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_binding(Identifier::new(location, identifier.name));
                            self.state = State::PathOperatorOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Underscore),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.set_wildcard();
                            return Ok((self.builder.finish(), None));
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}", "{identifier}", "_"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::PathOperatorOrEnd => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleColon),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Path));
                            self.state = State::PathOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::PathOperand => {
                    let (expression, next) =
                        TerminalOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.extend_with_expression(expression);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_path_operator(location, operator);
                    }
                    self.state = State::PathOperatorOrEnd;
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
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::BooleanLiteral;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::MatchPattern;
    use crate::syntax::MatchPatternVariant;

    #[test]
    fn ok_literal_boolean() {
        let input = "true";

        let expected = Ok((
            MatchPattern::new(
                Location::new(1, 1),
                MatchPatternVariant::BooleanLiteral(BooleanLiteral::new(
                    Location::new(1, 1),
                    lexical::BooleanLiteral::True,
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_literal_integer() {
        let input = "42";

        let expected = Ok((
            MatchPattern::new(
                Location::new(1, 1),
                MatchPatternVariant::IntegerLiteral(IntegerLiteral::new(
                    Location::new(1, 1),
                    lexical::IntegerLiteral::new_decimal("42".to_owned()),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_binding() {
        let input = "value";

        let expected = Ok((
            MatchPattern::new(
                Location::new(1, 1),
                MatchPatternVariant::Binding(Identifier::new(
                    Location::new(1, 1),
                    "value".to_owned(),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 6))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_path() {
        let input = "data::Inner::VALUE";

        let expected = Ok((
            MatchPattern::new(
                Location::new(1, 1),
                MatchPatternVariant::Path(Expression::new(
                    Location::new(1, 1),
                    vec![
                        ExpressionElement::new(
                            Location::new(1, 1),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 1), "data".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 7),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 7), "Inner".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 5),
                            ExpressionObject::Operator(ExpressionOperator::Path),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 14),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 14), "VALUE".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 12),
                            ExpressionObject::Operator(ExpressionOperator::Path),
                        ),
                    ],
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 19))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_wildcard() {
        let input = "_";

        let expected = Ok((
            MatchPattern::new(Location::new(1, 1), MatchPatternVariant::Wildcard),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
