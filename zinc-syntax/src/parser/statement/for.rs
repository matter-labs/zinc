//!
//! The `for` statement parser.
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
use crate::parser::expression::terminal::block::Parser as BlockExpressionParser;
use crate::parser::expression::Parser as ExpressionParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#for::builder::Builder as ForStatementBuilder;
use crate::tree::statement::r#for::Statement as ForStatement;

/// The missing index identifier error hint.
pub static HINT_EXPECTED_INDEX_IDENTIFIER: &str =
    "for-loops must have the index identifier, e.g. `for i in 0..10 { ... }`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordFor,
    /// The `for` has been parsed so far.
    IndexIdentifier,
    /// The `for {identifier}` has been parsed so far.
    KeywordIn,
    /// The `for {identifier} in` has been parsed so far.
    BoundsExpression,
    /// The `for {identifier} in {expression}` has been parsed so far.
    BlockExpressionOrKeywordWhile,
    /// The `for {identifier} in {expression} while` has been parsed so far.
    WhileConditionExpression,
    /// The `for {identifier} in {expression}` with optional `while {expression}` has been parsed so far.
    BlockExpression,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordFor
    }
}

///
/// The `for` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: ForStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a for-loop statement.
    ///
    /// '
    /// for i in 0..100 while i < x {
    ///     x += i;
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ForStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordFor => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::For),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::IndexIdentifier;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["for"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::IndexIdentifier => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_index_identifier(identifier);
                            self.state = State::KeywordIn;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_INDEX_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::KeywordIn => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::In),
                            ..
                        } => {
                            self.state = State::BoundsExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["in"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::BoundsExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_bounds_expression(expression);
                    self.state = State::BlockExpressionOrKeywordWhile;
                }
                State::BlockExpressionOrKeywordWhile => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            let (block, next) =
                                BlockExpressionParser::default().parse(stream, Some(token))?;
                            self.builder.set_block(block);
                            return Ok((self.builder.finish(), next));
                        }
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::While),
                            ..
                        } => self.state = State::WhileConditionExpression,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_one_of_or_operator(
                                    location,
                                    vec!["{", "while"],
                                    lexeme,
                                    None,
                                ),
                            ));
                        }
                    }
                }
                State::WhileConditionExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_while_condition(expression);
                    self.state = State::BlockExpression;
                }
                State::BlockExpression => {
                    let (expression, next) =
                        BlockExpressionParser::default().parse(stream, self.next.take())?;
                    self.builder.set_block(expression);
                    return Ok((self.builder.finish(), next));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::block::Expression as BlockExpression;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::statement::r#for::Statement as ForStatement;

    #[test]
    fn ok_empty() {
        let input = r#"for i in 0..4 {}"#;

        let expected = Ok((
            ForStatement::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 5), "i".to_owned()),
                ExpressionTree::new_with_leaves(
                    Location::test(1, 11),
                    ExpressionTreeNode::operator(ExpressionOperator::Range),
                    Some(ExpressionTree::new(
                        Location::test(1, 10),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 10),
                                LexicalIntegerLiteral::new_decimal("0".to_owned()),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 13),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 13),
                                LexicalIntegerLiteral::new_decimal("4".to_owned()),
                            ),
                        )),
                    )),
                ),
                None,
                BlockExpression::new(Location::test(1, 15), vec![], None),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok() {
        let input = r#"for i in 0..=4 { 2 + 1 }"#;

        let expected = Ok((
            ForStatement::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 5), "i".to_owned()),
                ExpressionTree::new_with_leaves(
                    Location::test(1, 11),
                    ExpressionTreeNode::operator(ExpressionOperator::RangeInclusive),
                    Some(ExpressionTree::new(
                        Location::test(1, 10),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 10),
                                LexicalIntegerLiteral::new_decimal("0".to_owned()),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 14),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 14),
                                LexicalIntegerLiteral::new_decimal("4".to_owned()),
                            ),
                        )),
                    )),
                ),
                None,
                BlockExpression::new(
                    Location::test(1, 16),
                    vec![],
                    Some(ExpressionTree::new_with_leaves(
                        Location::test(1, 20),
                        ExpressionTreeNode::operator(ExpressionOperator::Addition),
                        Some(ExpressionTree::new(
                            Location::test(1, 18),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(1, 18),
                                    LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                ),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 22),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(1, 22),
                                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        )),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"for { 2 + 2 }"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 5),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_INDEX_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_keyword_in() {
        let input = r#"for i { 2 + 2 }"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 7),
            vec!["in"],
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_left_or_keyword_while() {
        let input = r#"for i in 0..10;"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 15),
            vec!["{", "while"],
            Lexeme::Symbol(Symbol::Semicolon),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
