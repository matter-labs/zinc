//!
//! The const statement parser.
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
use crate::syntax::tree::statement::r#const::builder::Builder as ConstStatementBuilder;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;

static HINT_EXPECTED_IDENTIFIER: &str =
    "constant must have an identifier, e.g. `const DATA: u8 = 42;`";
static HINT_EXPECTED_TYPE: &str = "constant must have a type, e.g. `const DATA: u8 = 42;`";
static HINT_EXPECTED_VALUE: &str = "constant must be initialized, e.g. `const DATA: u8 = 42;`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordConst,
    Identifier,
    Colon,
    Type,
    Equals,
    Expression,
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordConst
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ConstStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'const' statement.
    ///
    /// 'const VALUE: u8 = 100;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ConstStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordConst => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Const),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["const"],
                                lexeme,
                                None,
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
                        } => self.state = State::Type,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_type(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_TYPE),
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
    use crate::syntax::tree::statement::r#const::Statement as ConstStatement;

    #[test]
    fn ok() {
        let input = r#"const A: u64 = 42;"#;

        let expected = Ok((
            ConstStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 7), "A".to_owned()),
                Type::new(Location::new(1, 10), TypeVariant::integer_unsigned(64)),
                ExpressionTree::new(
                    Location::new(1, 16),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 16),
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
    fn error_expected_identifier() {
        let input = r#"const = 42;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 7),
            Lexeme::Symbol(Symbol::Equals),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"const VALUE = 42;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_type(
            Location::new(1, 13),
            Lexeme::Symbol(Symbol::Equals),
            Some(super::HINT_EXPECTED_TYPE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_value() {
        let input = r#"const A: u64;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_value(
            Location::new(1, 13),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"const A: u64 = 42"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 18),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
