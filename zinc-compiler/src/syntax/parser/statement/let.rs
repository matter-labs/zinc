//!
//! The let statement parser.
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
use crate::syntax::parser::r#type::Parser as TypeParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#let::builder::Builder as LetStatementBuilder;
use crate::syntax::tree::statement::r#let::Statement as LetStatement;

static HINT_EXPECTED_IDENTIFIER: &str =
    "variable must have an identifier, e.g. `let value: u8 = 42;`";
static HINT_EXPECTED_VALUE: &str = "variable must be initialized, e.g. `let value: u8 = 42;`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordLet,
    MutOrIdentifier,
    Identifier,
    ColonOrEquals,
    Type,
    Equals,
    Expression,
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordLet
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: LetStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'let' statement.
    ///
    /// 'let mut value: field = 42;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(LetStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordLet => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Let),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::MutOrIdentifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["let"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::MutOrIdentifier => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mut),
                            ..
                        } => {
                            self.builder.set_mutable();
                            self.state = State::Identifier;
                        }
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::ColonOrEquals;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_mut_or_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::ColonOrEquals;
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
                State::ColonOrEquals => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        } => self.state = State::Type,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            ..
                        } => self.state = State::Expression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_type_or_value(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_VALUE),
                            )));
                        }
                    }
                }
                State::Type => {
                    let (r#type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_type(r#type);
                    self.state = State::Equals;
                }
                State::Equals => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
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
                    self.builder.set_expression(expression);
                    self.next = next;
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    return match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => {
                            Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec![";"],
                                lexeme,
                                None,
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
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::r#let::Statement as LetStatement;

    #[test]
    fn ok_simple() {
        let input = r#"let a = 42;"#;

        let expected = Ok((
            LetStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 5), "a".to_owned()),
                false,
                None,
                ExpressionTree::new(
                    Location::new(1, 9),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 9),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_mut_with_type() {
        let input = r#"let mut a: u232 = 42;"#;

        let expected = Ok((
            LetStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 9), "a".to_owned()),
                true,
                Some(Type::new(
                    Location::new(1, 12),
                    TypeVariant::integer_unsigned(232),
                )),
                ExpressionTree::new(
                    Location::new(1, 19),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 19),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_mut_or_identifier() {
        let input = r#"let = 42;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_mut_or_identifier(
            Location::new(1, 5),
            Lexeme::Symbol(Symbol::Equals),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"let mut = 42;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 9),
            Lexeme::Symbol(Symbol::Equals),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type_or_value() {
        let input = r#"let a;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_type_or_value(
            Location::new(1, 6),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_value() {
        let input = r#"let a: u64;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_value(
            Location::new(1, 11),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"let a: u64 = 42"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 16),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
