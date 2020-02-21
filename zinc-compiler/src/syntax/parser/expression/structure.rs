//!
//! The structure expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Identifier;
use crate::syntax::StructureExpression;
use crate::syntax::StructureExpressionBuilder;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Identifier,
    BracketCurlyLeftOrEnd,
    IdentifierOrBracketCurlyRight,
    Colon,
    Expression,
    CommaOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::Identifier
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: StructureExpressionBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(StructureExpression, Option<Token>), Error> {
        loop {
            match self.state {
                State::Identifier => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            self.builder.set_location(location);
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_identifier(identifier);
                            self.state = State::BracketCurlyLeftOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location, lexeme,
                            )));
                        }
                    }
                }
                State::BracketCurlyLeftOrEnd => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            match stream.borrow_mut().look_ahead(2)? {
                                Token {
                                    lexeme: Lexeme::Symbol(Symbol::Colon),
                                    ..
                                } => {}
                                _ => return Ok((self.builder.finish(), Some(token))),
                            }

                            self.builder.set_struct();
                            self.state = State::IdentifierOrBracketCurlyRight;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::IdentifierOrBracketCurlyRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), None)),
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.push_field_identifier(identifier);
                            self.state = State::Colon;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location, lexeme,
                            )));
                        }
                    }
                }
                State::Colon => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        } => self.state = State::Expression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![":"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Expression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_field_expression(expression);
                    self.state = State::CommaOrBracketCurlyRight;
                }
                State::CommaOrBracketCurlyRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::IdentifierOrBracketCurlyRight,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec![",", "}"],
                                lexeme,
                            )));
                        }
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
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::StructureExpression;

    #[test]
    fn ok_struct_single() {
        let input = r#"
Test {
    a: 1,
}
"#;

        let expected = Ok((
            StructureExpression::new(
                Location::new(2, 1),
                Identifier::new(Location::new(2, 1), "Test".to_owned()),
                true,
                vec![(
                    Identifier::new(Location::new(3, 5), "a".to_owned()),
                    Expression::new(
                        Location::new(3, 8),
                        vec![ExpressionElement::new(
                            Location::new(3, 8),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(3, 8),
                                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        )],
                    ),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_struct_multiple() {
        let input = r#"
Test {
    a: 1,
    b: 2,
    c: 3,
}
"#;

        let expected = Ok((
            StructureExpression::new(
                Location::new(2, 1),
                Identifier::new(Location::new(2, 1), "Test".to_owned()),
                true,
                vec![
                    (
                        Identifier::new(Location::new(3, 5), "a".to_owned()),
                        Expression::new(
                            Location::new(3, 8),
                            vec![ExpressionElement::new(
                                Location::new(3, 8),
                                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::new(3, 8),
                                        lexical::IntegerLiteral::new_decimal("1".to_owned()),
                                    ),
                                )),
                            )],
                        ),
                    ),
                    (
                        Identifier::new(Location::new(4, 5), "b".to_owned()),
                        Expression::new(
                            Location::new(4, 8),
                            vec![ExpressionElement::new(
                                Location::new(4, 8),
                                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::new(4, 8),
                                        lexical::IntegerLiteral::new_decimal("2".to_owned()),
                                    ),
                                )),
                            )],
                        ),
                    ),
                    (
                        Identifier::new(Location::new(5, 5), "c".to_owned()),
                        Expression::new(
                            Location::new(5, 8),
                            vec![ExpressionElement::new(
                                Location::new(5, 8),
                                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::new(5, 8),
                                        lexical::IntegerLiteral::new_decimal("3".to_owned()),
                                    ),
                                )),
                            )],
                        ),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_identifier() {
        let input = r#"test"#;

        let expected = Ok((
            StructureExpression::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), "test".to_owned()),
                false,
                vec![],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 5))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
