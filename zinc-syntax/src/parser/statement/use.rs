//!
//! The `use` statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::expression::path::Parser as PathOperandParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#use::builder::Builder as UseStatementBuilder;
use crate::tree::statement::r#use::Statement as UseStatement;

/// The missing alias identifier error hint.
pub static HINT_EXPECTED_ALIAS_IDENTIFIER: &str =
    "specify the alias identifier after the `as` keyword, e.g. `use crate::Data as GlobalData;`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordUse,
    /// The `use` has been parsed so far.
    Path,
    /// The `use {path}` has been parsed so far.
    AsOrNext,
    /// The `use {path} as` has been parsed so far.
    AliasIdentifier,
    /// The `use {path} as {identifier}` has been parsed so far.
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordUse
    }
}

///
/// The `use` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: UseStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'use' statement.
    ///
    /// 'use jabberwocky::gone;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(UseStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordUse => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Use),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Path;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["use"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Path => {
                    let (expression, next) =
                        PathOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_path(expression);
                    self.next = next;
                    self.state = State::AsOrNext;
                }
                State::AsOrNext => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::As),
                            ..
                        } => {
                            self.state = State::AliasIdentifier;
                        }
                        token => {
                            self.next = Some(token);
                            self.state = State::Semicolon;
                        }
                    }
                }
                State::AliasIdentifier => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_alias_identifier(identifier);
                            self.state = State::Semicolon;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_ALIAS_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::Semicolon => {
                    return match crate::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of(location, vec![";"], lexeme, None),
                        )),
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::statement::r#use::Statement as UseStatement;

    #[test]
    fn ok() {
        let input = r#"use mega::ultra::namespace;"#;

        let expected = Ok((
            UseStatement::new(
                Location::test(1, 1),
                ExpressionTree::new_with_leaves(
                    Location::test(1, 16),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new_with_leaves(
                        Location::test(1, 9),
                        ExpressionTreeNode::operator(ExpressionOperator::Path),
                        Some(ExpressionTree::new(
                            Location::test(1, 5),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 5), "mega".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 11),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 11), "ultra".to_owned()),
                            )),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 18),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::test(1, 18), "namespace".to_owned()),
                        )),
                    )),
                ),
                None,
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_with_alias() {
        let input = r#"use mega::ultra::namespace as MegaUltraNamespace;"#;

        let expected = Ok((
            UseStatement::new(
                Location::test(1, 1),
                ExpressionTree::new_with_leaves(
                    Location::test(1, 16),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new_with_leaves(
                        Location::test(1, 9),
                        ExpressionTreeNode::operator(ExpressionOperator::Path),
                        Some(ExpressionTree::new(
                            Location::test(1, 5),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 5), "mega".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 11),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 11), "ultra".to_owned()),
                            )),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 18),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::test(1, 18), "namespace".to_owned()),
                        )),
                    )),
                ),
                Some(Identifier::new(
                    Location::test(1, 31),
                    "MegaUltraNamespace".to_owned(),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"use jabberwocky"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 16),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
