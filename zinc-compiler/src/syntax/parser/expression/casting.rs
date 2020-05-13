//!
//! The casting operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::parser::expression::access::Parser as AccessOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Default)]
pub struct Parser {
    builder: ExpressionTreeBuilder,
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
        mut initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
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
    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::boolean::Boolean as LexicalBooleanLiteral;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::tuple_index::TupleIndex;

    #[test]
    fn ok_access() {
        let input = r#"array[42].25.value"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 13),
                ExpressionTreeNode::operator(ExpressionOperator::Dot),
                Some(ExpressionTree::new_with_leaves(
                    Location::new(1, 10),
                    ExpressionTreeNode::operator(ExpressionOperator::Dot),
                    Some(ExpressionTree::new_with_leaves(
                        Location::new(1, 6),
                        ExpressionTreeNode::operator(ExpressionOperator::Index),
                        Some(ExpressionTree::new(
                            Location::new(1, 1),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 1), "array".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 7),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 7),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 11),
                        ExpressionTreeNode::operand(ExpressionOperand::TupleIndex(
                            TupleIndex::new(
                                Location::new(1, 11),
                                IntegerLiteral::new(
                                    Location::new(1, 11),
                                    LexicalIntegerLiteral::new_decimal("25".to_owned()),
                                ),
                            ),
                        )),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 14),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 14),
                        "value".to_owned(),
                    ))),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 19))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_negation() {
        let input = r#"-42"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 1),
                ExpressionTreeNode::operator(ExpressionOperator::Negation),
                Some(ExpressionTree::new(
                    Location::new(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 2),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )),
                None,
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 4))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_bitwise_not() {
        let input = r#"~0b101010"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 1),
                ExpressionTreeNode::operator(ExpressionOperator::BitwiseNot),
                Some(ExpressionTree::new(
                    Location::new(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 2),
                            LexicalIntegerLiteral::new_binary("101010".to_owned()),
                        ),
                    )),
                )),
                None,
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 10))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_not() {
        let input = r#"!false"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 1),
                ExpressionTreeNode::operator(ExpressionOperator::Not),
                Some(ExpressionTree::new(
                    Location::new(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::new(1, 2), LexicalBooleanLiteral::r#false()),
                    )),
                )),
                None,
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 7))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
