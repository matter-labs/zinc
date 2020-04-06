//!
//! The for-loop statement parser.
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
use crate::syntax::parser::expression::terminal::block::Parser as BlockExpressionParser;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#for::builder::Builder as ForStatementBuilder;
use crate::syntax::tree::statement::r#for::Statement as ForStatement;

static HINT_EXPECTED_INDEX_IDENTIFIER: &str =
    "for-loops must have the index identifier, e.g. `for i in 0..10 { ... }`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordFor,
    IndexIdentifier,
    KeywordIn,
    BoundsExpression,
    BlockExpressionOrKeywordWhile,
    WhileConditionExpression,
    BlockExpression,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordFor
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ForStatementBuilder,
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
        mut initial: Option<Token>,
    ) -> Result<(ForStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordFor => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::For),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::IndexIdentifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["for"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::IndexIdentifier => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_index_identifier(identifier);
                            self.state = State::KeywordIn;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_INDEX_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::KeywordIn => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::In),
                            ..
                        } => {
                            self.state = State::BoundsExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
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
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
                            return Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec!["{", "while"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::WhileConditionExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), None)?;
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
    use crate::syntax::tree::expression::block::Expression as BlockExpression;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::statement::r#for::Statement as ForStatement;

    #[test]
    fn ok_empty() {
        let input = r#"for i in 0..4 {}"#;

        let expected = Ok((
            ForStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 5), "i".to_owned()),
                ExpressionTree::new_with_leaves(
                    Location::new(1, 11),
                    ExpressionTreeNode::operator(ExpressionOperator::Range),
                    Some(ExpressionTree::new(
                        Location::new(1, 10),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 10),
                                LexicalIntegerLiteral::new_decimal("0".to_owned()),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 13),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 13),
                                LexicalIntegerLiteral::new_decimal("4".to_owned()),
                            ),
                        )),
                    )),
                ),
                None,
                BlockExpression::new(Location::new(1, 15), vec![], None),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok() {
        let input = r#"for i in 0..=4 { 2 + 1 }"#;

        let expected = Ok((
            ForStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 5), "i".to_owned()),
                ExpressionTree::new_with_leaves(
                    Location::new(1, 11),
                    ExpressionTreeNode::operator(ExpressionOperator::RangeInclusive),
                    Some(ExpressionTree::new(
                        Location::new(1, 10),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 10),
                                LexicalIntegerLiteral::new_decimal("0".to_owned()),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 14),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 14),
                                LexicalIntegerLiteral::new_decimal("4".to_owned()),
                            ),
                        )),
                    )),
                ),
                None,
                BlockExpression::new(
                    Location::new(1, 16),
                    vec![],
                    Some(ExpressionTree::new_with_leaves(
                        Location::new(1, 20),
                        ExpressionTreeNode::operator(ExpressionOperator::Addition),
                        Some(ExpressionTree::new(
                            Location::new(1, 18),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 18),
                                    LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                ),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 22),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 22),
                                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        )),
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
        let input = r#"for { 2 + 2 }"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 5),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_INDEX_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_keyword_in() {
        let input = r#"for i { 2 + 2 }"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 7),
            vec!["in"],
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_left_or_keyword_while() {
        let input = r#"for i in 0..10;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 15),
            vec!["{", "while"],
            Lexeme::Symbol(Symbol::Semicolon),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
