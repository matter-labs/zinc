//!
//! The structure expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Keyword;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Identifier;
use crate::syntax::PathExpressionParser;
use crate::syntax::StructureExpression;
use crate::syntax::StructureExpressionBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordStruct,
    Path,
    BracketCurlyLeftOrEnd,
    IdentifierOrBracketCurlyRight,
    Colon,
    Expression,
    CommaOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordStruct
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: StructureExpressionBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>, mut initial: Option<Token>) -> Result<(StructureExpression, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordStruct => {
                    let next = match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    };
                    match next {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Struct),
                            ..
                        } => self.state = State::Path,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["struct"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Path => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        token @ Token {
                            lexeme: Lexeme::Identifier { .. },
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let (expression, next) = PathExpressionParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.set_path_expression(expression);
                            self.state = State::BracketCurlyLeftOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::BracketCurlyLeftOrEnd => {
                    match self.next.take().expect("Always contains a value") {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => self.state = State::IdentifierOrBracketCurlyRight,
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::IdentifierOrBracketCurlyRight => {
                    let next = stream.borrow_mut().next()?;
                    match next {
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
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}", "}"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Colon => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        } => self.state = State::Expression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![":"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Expression => {
                    let (expression, next) = ExpressionParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_field_expression(expression);
                    self.state = State::CommaOrBracketCurlyRight;
                }
                State::CommaOrBracketCurlyRight => {
                    match self.next.take().expect("Always contains a value") {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::IdentifierOrBracketCurlyRight,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
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
    fn error_expected_comma() {
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
