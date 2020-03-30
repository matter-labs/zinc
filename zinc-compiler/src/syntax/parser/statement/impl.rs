//!
//! The impl statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::statement::local_impl::Parser as ImplementationLocalStatementParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#impl::builder::Builder as ImplStatementBuilder;
use crate::syntax::tree::statement::r#impl::Statement as ImplStatement;

static HINT_EXPECTED_IDENTIFIER: &str =
    "type implementation must have an identifier, e.g. `impl Data { ... }`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordImpl,
    Identifier,
    BracketCurlyLeft,
    StatementOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordImpl
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ImplStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ImplStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordImpl => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Impl),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["impl"],
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
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_identifier(identifier);
                            self.state = State::BracketCurlyLeft;
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
                State::BracketCurlyLeft => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            self.state = State::StatementOrBracketCurlyRight;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::StatementOrBracketCurlyRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), None)),
                        token => {
                            let (statement, next) = ImplementationLocalStatementParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_statement(statement);
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
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;
    use crate::syntax::tree::statement::r#const::Statement as ConstStatement;
    use crate::syntax::tree::statement::r#impl::Statement as ImplStatement;

    #[test]
    fn ok_empty() {
        let input = r#"
    impl Test {}
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"
    impl Test {
        const VALUE: u64 = 42;
    }
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![ImplementationLocalStatement::Const(ConstStatement::new(
                    Location::new(3, 9),
                    Identifier::new(Location::new(3, 15), "VALUE".to_owned()),
                    Type::new(Location::new(3, 22), TypeVariant::integer_unsigned(64)),
                    ExpressionTree::new(
                        Location::new(3, 28),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(3, 28),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                        None,
                        None,
                    ),
                ))],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"
    impl Test {
        const VALUE: u64 = 42;

        const ANOTHER: u64 = 42;

        const YET_ANOTHER: u64 = 42;
    }
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(3, 9),
                        Identifier::new(Location::new(3, 15), "VALUE".to_owned()),
                        Type::new(Location::new(3, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(3, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(3, 28),
                                    lexical::IntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                            None,
                            None,
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(5, 9),
                        Identifier::new(Location::new(5, 15), "ANOTHER".to_owned()),
                        Type::new(Location::new(5, 24), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(5, 30),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(5, 30),
                                    lexical::IntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                            None,
                            None,
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(7, 9),
                        Identifier::new(Location::new(7, 15), "YET_ANOTHER".to_owned()),
                        Type::new(Location::new(7, 28), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(7, 34),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(7, 34),
                                    lexical::IntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                            None,
                            None,
                        ),
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"impl { const VALUE: u64 = 42; }"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 6),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
