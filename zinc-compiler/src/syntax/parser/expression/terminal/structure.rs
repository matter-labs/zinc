//!
//! The structure expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::tree::expression::structure::builder::Builder as StructureExpressionBuilder;
use crate::syntax::tree::expression::structure::Expression as StructureExpression;
use crate::syntax::tree::identifier::Identifier;

pub static HINT_EXPECTED_IDENTIFIER: &str =
    "structure field must have an identifier, e.g. `Data { a: 42 }`";
pub static HINT_EXPECTED_VALUE: &str = "structure field must be initialized, e.g. `Data { a: 42 }`";

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
    ///
    /// Parses a structure literal.
    ///
    /// '
    /// Data { a: 1, b: true, c: (10, 20) }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(StructureExpression, Option<Token>), Error> {
        loop {
            match self.state {
                State::Identifier => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            self.builder.set_location(location);
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::BracketCurlyLeftOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::Crate),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::Super),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::SelfLowercase),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::SelfUppercase),
                            location,
                        } => {
                            self.builder.set_location(location);
                            let identifier = Identifier::new(location, keyword.to_string());
                            self.builder.set_identifier(identifier);
                            self.state = State::BracketCurlyLeftOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location, lexeme, None,
                            )));
                        }
                    }
                }
                State::BracketCurlyLeftOrEnd => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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

                            self.builder.set_is_structure();
                            self.state = State::IdentifierOrBracketCurlyRight;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::IdentifierOrBracketCurlyRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), None)),
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.push_field_identifier(identifier);
                            self.state = State::Colon;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::Colon => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        } => self.state = State::Expression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_value(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_VALUE),
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
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
                                None,
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
    use super::Error;
    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::keyword::Keyword;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::structure::Expression as StructureExpression;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

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

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_keyword_crate() {
        let input = r#"crate"#;

        let expected = Ok((
            StructureExpression::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), Keyword::Crate.to_string()),
                false,
                vec![],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 6))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_keyword_super() {
        let input = r#"super"#;

        let expected = Ok((
            StructureExpression::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), Keyword::Super.to_string()),
                false,
                vec![],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 6))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_keyword_self_lowercase() {
        let input = r#"self"#;

        let expected = Ok((
            StructureExpression::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), Keyword::SelfLowercase.to_string()),
                false,
                vec![],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 5))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_keyword_self_uppercase() {
        let input = r#"Self"#;

        let expected = Ok((
            StructureExpression::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), Keyword::SelfUppercase.to_string()),
                false,
                vec![],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 5))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

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
                    ExpressionTree::new(
                        Location::new(3, 8),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(3, 8),
                                LexicalIntegerLiteral::new_decimal("1".to_owned()),
                            ),
                        )),
                    ),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

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
                        ExpressionTree::new(
                            Location::new(3, 8),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(3, 8),
                                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        ),
                    ),
                    (
                        Identifier::new(Location::new(4, 5), "b".to_owned()),
                        ExpressionTree::new(
                            Location::new(4, 8),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(4, 8),
                                    LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                ),
                            )),
                        ),
                    ),
                    (
                        Identifier::new(Location::new(5, 5), "c".to_owned()),
                        ExpressionTree::new(
                            Location::new(5, 8),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(5, 8),
                                    LexicalIntegerLiteral::new_decimal("3".to_owned()),
                                ),
                            )),
                        ),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier_or_bracket_curly_right() {
        let input = r#"Data { ) : 42 }"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 8),
            Lexeme::Symbol(Symbol::ParenthesisRight),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_value() {
        let input = r#"Data { a: 42, b }"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_value(
            Location::new(1, 17),
            Lexeme::Symbol(Symbol::BracketCurlyRight),
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_bracket_curly_right() {
        let input = r#"Data { a: 42 )"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 14),
            vec![",", "}"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
