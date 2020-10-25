//!
//! The conditional expression parser.
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
use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::expression::conditional::builder::Builder as ConditionalExpressionBuilder;
use crate::tree::expression::conditional::Expression as ConditionalExpression;
use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordIf,
    /// The `if` has been parsed so far.
    Condition,
    /// The `if {expression}` has been parsed so far.
    MainBlock,
    /// The `if {expression} {block}` has been parsed so far.
    ElseKeywordOrEnd,
    /// The `if {expression} {block} else` has been parsed so far.
    KeywordIfOrElseBlock,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordIf
    }
}

///
/// The conditional expression parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed value.
    builder: ConditionalExpressionBuilder,
}

impl Parser {
    ///
    /// Parses a conditional expression.
    ///
    /// '
    /// if a > b {
    ///     a
    /// } else {
    ///     b
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ConditionalExpression, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordIf => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::If),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Condition;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["if"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Condition => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_condition(expression);
                    self.state = State::MainBlock;
                }
                State::MainBlock => {
                    let (block, next) =
                        BlockExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_main_block(block);
                    self.state = State::ElseKeywordOrEnd;
                }
                State::ElseKeywordOrEnd => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Else),
                            ..
                        } => self.state = State::KeywordIfOrElseBlock,
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::KeywordIfOrElseBlock => {
                    return match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::If),
                            ..
                        } => {
                            let (expression, next) = Self::default().parse(stream, Some(token))?;
                            let block = BlockExpression::new(
                                expression.location,
                                vec![],
                                Some(ExpressionTree::new(
                                    expression.location,
                                    ExpressionTreeNode::operand(ExpressionOperand::Conditional(
                                        expression,
                                    )),
                                )),
                            );
                            self.builder.set_else_block(block);
                            Ok((self.builder.finish(), next))
                        }
                        token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            let (block, next) =
                                BlockExpressionParser::default().parse(stream, Some(token))?;
                            self.next = next;
                            self.builder.set_else_block(block);
                            Ok((self.builder.finish(), None))
                        }
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of(location, vec!["if", "{"], lexeme, None),
                        )),
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::BooleanLiteral as LexicalBooleanLiteral;
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::block::Expression as BlockExpression;
    use crate::tree::expression::conditional::Expression as ConditionalExpression;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_nested() {
        let input = r#"if true { 1 } else if false { 2 } else { 3 }"#;

        let expected = Ok((
            ConditionalExpression::new(
                Location::test(1, 1),
                ExpressionTree::new(
                    Location::test(1, 4),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::test(1, 4), LexicalBooleanLiteral::r#true()),
                    )),
                ),
                BlockExpression::new(
                    Location::test(1, 9),
                    vec![],
                    Some(ExpressionTree::new(
                        Location::test(1, 11),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 11),
                                LexicalIntegerLiteral::new_decimal("1".to_owned()),
                            ),
                        )),
                    )),
                ),
                Some(BlockExpression::new(
                    Location::test(1, 20),
                    vec![],
                    Some(ExpressionTree::new(
                        Location::test(1, 20),
                        ExpressionTreeNode::operand(ExpressionOperand::Conditional(
                            ConditionalExpression::new(
                                Location::test(1, 20),
                                ExpressionTree::new(
                                    Location::test(1, 23),
                                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                                        BooleanLiteral::new(
                                            Location::test(1, 23),
                                            LexicalBooleanLiteral::r#false(),
                                        ),
                                    )),
                                ),
                                BlockExpression::new(
                                    Location::test(1, 29),
                                    vec![],
                                    Some(ExpressionTree::new(
                                        Location::test(1, 31),
                                        ExpressionTreeNode::operand(
                                            ExpressionOperand::LiteralInteger(IntegerLiteral::new(
                                                Location::test(1, 31),
                                                LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                            )),
                                        ),
                                    )),
                                ),
                                Some(BlockExpression::new(
                                    Location::test(1, 40),
                                    vec![],
                                    Some(ExpressionTree::new(
                                        Location::test(1, 42),
                                        ExpressionTreeNode::operand(
                                            ExpressionOperand::LiteralInteger(IntegerLiteral::new(
                                                Location::test(1, 42),
                                                LexicalIntegerLiteral::new_decimal("3".to_owned()),
                                            )),
                                        ),
                                    )),
                                )),
                            ),
                        )),
                    )),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"if true { 42 } else ("#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 21),
                vec!["if", "{"],
                Lexeme::Symbol(Symbol::ParenthesisLeft),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
