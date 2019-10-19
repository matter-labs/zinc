//!
//! The structure expression parser.
//!
//! The result can be either of:
//! 1. An identifier
//! 2. A structure literal
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionParser;
use crate::syntax::Identifier;
use crate::syntax::PathExpressionParser;
use crate::syntax::StructureExpressionBuilder;
use crate::Error;

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
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::Identifier => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier { .. },
                            location,
                        })) => {
                            self.builder.set_location(location);
                            let path = PathExpressionParser::default().parse(stream.clone())?;
                            self.builder.set_path(path);
                            self.state = State::BracketCurlyLeftOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}"],
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::BracketCurlyLeftOrEnd => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.builder.set_bracket();
                            self.state = State::IdentifierOrBracketCurlyRight;
                        }
                        Some(Ok(..)) => return Ok(self.builder.finish()),
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::IdentifierOrBracketCurlyRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.push_field_identifier(identifier);
                            self.state = State::Colon;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}", "}"],
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::Colon => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        })) => self.state = State::Expression,
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![":"],
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::Expression => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_field_expression(expression);
                    self.state = State::CommaOrBracketCurlyRight;
                }
                State::CommaOrBracketCurlyRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        })) => self.state = State::IdentifierOrBracketCurlyRight,
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", "}"],
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
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
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::Error as SyntaxError;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::Literal;
    use crate::syntax::StructureExpression;
    use crate::Error;

    #[test]
    fn ok_single() {
        let input = r#"
    Test {
        a: 1,
    }
"#;

        let expected = Ok(Expression::new(
            Location::new(2, 5),
            vec![ExpressionElement::new(
                Location::new(2, 5),
                ExpressionObject::Operand(ExpressionOperand::Structure(StructureExpression::new(
                    Location::new(2, 5),
                    Expression::new(
                        Location::new(2, 5),
                        vec![
                            ExpressionElement::new(
                                Location::new(2, 5),
                                ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                                    Location::new(2, 5),
                                    "Test".to_owned(),
                                ))),
                            ),
                        ],
                    ),
                    vec![(
                        Identifier::new(Location::new(3, 9), "a".to_owned()),
                        Expression::new(
                            Location::new(3, 12),
                            vec![ExpressionElement::new(
                                Location::new(3, 12),
                                ExpressionObject::Operand(ExpressionOperand::Literal(
                                    Literal::new(
                                        Location::new(3, 12),
                                        lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                            "1".to_owned(),
                                        )),
                                    ),
                                )),
                            )],
                        ),
                    )],
                ))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"
    Test {
        a: 1,
        b: 2,
        c: 3,
    }
"#;

        let expected = Ok(Expression::new(
            Location::new(2, 5),
            vec![ExpressionElement::new(
                Location::new(2, 5),
                ExpressionObject::Operand(ExpressionOperand::Structure(StructureExpression::new(
                    Location::new(2, 5),
                    Expression::new(
                        Location::new(2, 5),
                        vec![
                            ExpressionElement::new(
                                Location::new(2, 5),
                                ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                                    Location::new(2, 5),
                                    "Test".to_owned(),
                                ))),
                            ),
                        ],
                    ),
                    vec![
                        (
                            Identifier::new(Location::new(3, 9), "a".to_owned()),
                            Expression::new(
                                Location::new(3, 12),
                                vec![ExpressionElement::new(
                                    Location::new(3, 12),
                                    ExpressionObject::Operand(ExpressionOperand::Literal(
                                        Literal::new(
                                            Location::new(3, 12),
                                            lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                                "1".to_owned(),
                                            )),
                                        ),
                                    )),
                                )],
                            ),
                        ),
                        (
                            Identifier::new(Location::new(4, 9), "b".to_owned()),
                            Expression::new(
                                Location::new(4, 12),
                                vec![ExpressionElement::new(
                                    Location::new(4, 12),
                                    ExpressionObject::Operand(ExpressionOperand::Literal(
                                        Literal::new(
                                            Location::new(4, 12),
                                            lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                                "2".to_owned(),
                                            )),
                                        ),
                                    )),
                                )],
                            ),
                        ),
                        (
                            Identifier::new(Location::new(5, 9), "c".to_owned()),
                            Expression::new(
                                Location::new(5, 12),
                                vec![ExpressionElement::new(
                                    Location::new(5, 12),
                                    ExpressionObject::Operand(ExpressionOperand::Literal(
                                        Literal::new(
                                            Location::new(5, 12),
                                            lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                                "3".to_owned(),
                                            )),
                                        ),
                                    )),
                                )],
                            ),
                        ),
                    ],
                ))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty() {
        let input = r#"
    Test {}
"#;

        let expected = Ok(Expression::new(
            Location::new(2, 5),
            vec![ExpressionElement::new(
                Location::new(2, 5),
                ExpressionObject::Operand(ExpressionOperand::Structure(StructureExpression::new(
                    Location::new(2, 5),
                    Expression::new(
                        Location::new(2, 5),
                        vec![
                            ExpressionElement::new(
                                Location::new(2, 5),
                                ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                                    Location::new(2, 5),
                                    "Test".to_owned(),
                                ))),
                            ),
                        ],
                    ),
                    vec![],
                ))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));;

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_identifier() {
        let input = r#"
    Test;
"#;

        let expected = Ok(Expression::new(
            Location::new(2, 5),
            vec![ExpressionElement::new(
                Location::new(2, 5),
                ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(Location::new(2, 5), "Test".to_owned()))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));;

        assert_eq!(expected, result);
    }

    #[test]
    fn err_expected_comma() {
        let input = r#"
    Test {
        a: 1;
    }
"#;

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(3, 13),
            vec![",", "}"],
            Lexeme::Symbol(Symbol::Semicolon),
        )));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
