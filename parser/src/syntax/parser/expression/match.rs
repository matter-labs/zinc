//!
//! The match expression parser.
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
use crate::syntax::MatchExpression;
use crate::syntax::MatchExpressionBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    MatchKeyword,
    MatchExpression,
    BracketCurlyLeft,
    BracketCurlyRightOrExpressionLeft,
    Select,
    ExpressionRight,
    CommaOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::MatchKeyword
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: MatchExpressionBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<MatchExpression, Error> {
        loop {
            match self.state {
                State::MatchKeyword => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Match),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::MatchExpression;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["match"],
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
                State::MatchExpression => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_match_expression(expression);
                    self.state = State::BracketCurlyLeft;
                }
                State::BracketCurlyLeft => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        })) => self.state = State::BracketCurlyRightOrExpressionLeft,
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{"],
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
                State::BracketCurlyRightOrExpressionLeft => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok(self.builder.finish());
                        },
                        Some(Ok(..)) => {
                            let expression = ExpressionParser::default().parse(stream.clone())?;
                            self.builder.push_branch_left(expression);
                            self.state = State::Select;
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::Select => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::EqualsGreater),
                            ..
                        })) => self.state = State::ExpressionRight,
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["=>"],
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
                State::ExpressionRight => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_branch_right(expression);
                    self.state = State::CommaOrBracketCurlyRight;
                }
                State::CommaOrBracketCurlyRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        })) => self.state = State::BracketCurlyRightOrExpressionLeft,
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
