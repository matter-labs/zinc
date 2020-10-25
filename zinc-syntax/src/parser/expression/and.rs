//!
//! The logical AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::comparison::Parser as ComparisonOperandParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    ComparisonFirstOperand,
    /// The first operand has been parsed and an operator is expected.
    ComparisonOperator,
    /// The first operand and the operator have been parsed, and the second operand is expected.
    ComparisonSecondOperand,
}

impl Default for State {
    fn default() -> Self {
        Self::ComparisonFirstOperand
    }
}

///
/// The logical AND operand parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed value.
    builder: ExpressionTreeBuilder,
}

impl Parser {
    ///
    /// Parses a logical AND expression operand, which is
    /// a lower precedence comparison operator expression.
    ///
    /// 'true == false'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::ComparisonFirstOperand => {
                    let (expression, next) = ComparisonOperandParser::default()
                        .parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::ComparisonOperator;
                }
                State::ComparisonOperator => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Equals, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMarkEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::NotEquals, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::GreaterEquals, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserEquals),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::LesserEquals, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Greater),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Greater, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Lesser),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Lesser, location);
                            self.state = State::ComparisonSecondOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::ComparisonSecondOperand => {
                    let (expression, next) =
                        ComparisonOperandParser::default().parse(stream, None)?;
                    self.builder.eat(expression);
                    return Ok((self.builder.finish(), next));
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
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_equals() {
        let input = r#"true == false"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 6),
                ExpressionTreeNode::operator(ExpressionOperator::Equals),
                Some(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::test(1, 1), LexicalBooleanLiteral::r#true()),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 9),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::test(1, 9), LexicalBooleanLiteral::r#false()),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 14))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_not_equals() {
        let input = r#"true != false"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 6),
                ExpressionTreeNode::operator(ExpressionOperator::NotEquals),
                Some(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::test(1, 1), LexicalBooleanLiteral::r#true()),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 9),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::test(1, 9), LexicalBooleanLiteral::r#false()),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 14))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_greater_equals() {
        let input = r#"42 >= 25"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::GreaterEquals),
                Some(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 7),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 7),
                            LexicalIntegerLiteral::new_decimal("25".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_lesser_equals() {
        let input = r#"42 <= 25"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::LesserEquals),
                Some(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 7),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 7),
                            LexicalIntegerLiteral::new_decimal("25".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_greater() {
        let input = r#"42 > 25"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Greater),
                Some(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 6),
                            LexicalIntegerLiteral::new_decimal("25".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 8))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_lesser() {
        let input = r#"42 < 25"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Lesser),
                Some(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 1),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 6),
                            LexicalIntegerLiteral::new_decimal("25".to_owned()),
                        ),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 8))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
