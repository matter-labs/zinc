//!
//! The casting operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::access::Parser as AccessOperandParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The casting operand parser.
///
#[derive(Default)]
pub struct Parser {
    /// The builder of the parsed value.
    builder: ExpressionTreeBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a casting expression operand, which is
    /// a lower precedence unary logical NOT, bitwise NOT or negation, or binary access
    /// operator expression.
    ///
    /// '-42'
    /// '~0b101010'
    /// '!true'
    /// 'foo(bar, 42, true)[42][0..5].4.value
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        match crate::parser::take_or_next(self.next.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                location,
            } => {
                let (expression, next) = Self::default().parse(stream, None)?;
                self.builder.eat(expression);
                self.builder.eat_operator(ExpressionOperator::Not, location);
                Ok((self.builder.finish(), next))
            }
            Token {
                lexeme: Lexeme::Symbol(Symbol::Tilde),
                location,
            } => {
                let (expression, next) = Self::default().parse(stream, None)?;
                self.builder.eat(expression);
                self.builder
                    .eat_operator(ExpressionOperator::BitwiseNot, location);
                Ok((self.builder.finish(), next))
            }
            Token {
                lexeme: Lexeme::Symbol(Symbol::Minus),
                location,
            } => {
                let (expression, next) = Self::default().parse(stream, None)?;
                self.builder.eat(expression);
                self.builder
                    .eat_operator(ExpressionOperator::Negation, location);
                Ok((self.builder.finish(), next))
            }
            token => {
                let (expression, next) =
                    AccessOperandParser::default().parse(stream, Some(token))?;
                self.builder.eat(expression);
                Ok((self.builder.finish(), next))
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
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::tuple_index::TupleIndex;

    #[test]
    fn ok_access() {
        let input = r#"(array[42].25).value"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 15),
                ExpressionTreeNode::operator(ExpressionOperator::Dot),
                Some(ExpressionTree::new_with_leaves(
                    Location::test(1, 11),
                    ExpressionTreeNode::operator(ExpressionOperator::Dot),
                    Some(ExpressionTree::new_with_leaves(
                        Location::test(1, 7),
                        ExpressionTreeNode::operator(ExpressionOperator::Index),
                        Some(ExpressionTree::new(
                            Location::test(1, 2),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 2), "array".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 8),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(1, 8),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 12),
                        ExpressionTreeNode::operand(ExpressionOperand::TupleIndex(
                            TupleIndex::new(
                                Location::test(1, 12),
                                IntegerLiteral::new(
                                    Location::test(1, 12),
                                    LexicalIntegerLiteral::new_decimal("25".to_owned()),
                                ),
                            ),
                        )),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 16),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(1, 16),
                        "value".to_owned(),
                    ))),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 21))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_negation() {
        let input = r#"-42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 1),
                ExpressionTreeNode::operator(ExpressionOperator::Negation),
                Some(ExpressionTree::new(
                    Location::test(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 2),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                None,
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 4))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_bitwise_not() {
        let input = r#"~0b101010"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 1),
                ExpressionTreeNode::operator(ExpressionOperator::BitwiseNot),
                Some(ExpressionTree::new(
                    Location::test(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 2),
                            LexicalIntegerLiteral::new_binary("101010".to_owned()),
                        ),
                    )),
                )),
                None,
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 10))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_not() {
        let input = r#"!false"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 1),
                ExpressionTreeNode::operator(ExpressionOperator::Not),
                Some(ExpressionTree::new(
                    Location::test(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::test(1, 2), LexicalBooleanLiteral::r#false()),
                    )),
                )),
                None,
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 7))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
