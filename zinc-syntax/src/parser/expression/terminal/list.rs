//!
//! The function argument list expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Location;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::Parser as ExpressionParser;
use crate::tree::expression::list::builder::Builder as ListExpressionBuilder;
use crate::tree::expression::list::Expression as ListExpression;

///
/// The function argument list expression parser.
///
#[derive(Default)]
pub struct Parser {
    /// The builder of the parsed value.
    builder: ListExpressionBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parser a function argument list.
    ///
    /// '(a + b, data, [1, 2, 3], foo(bar))'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
        location: Location,
    ) -> Result<(ListExpression, Option<Token>), ParsingError> {
        self.next = initial;

        self.builder.set_location(location);
        loop {
            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                token
                @
                Token {
                    lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                    ..
                } => {
                    return Ok((self.builder.finish(), Some(token)));
                }
                token
                @
                Token {
                    lexeme: Lexeme::Eof,
                    ..
                } => {
                    return Ok((self.builder.finish(), Some(token)));
                }
                token => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    self.builder.push_expression(expression);
                }
            }

            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => continue,
                token => {
                    return Ok((self.builder.finish(), Some(token)));
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
    use crate::tree::expression::list::Expression as ListExpression;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            ListExpression::new(Location::test(1, 1), vec![]),
            Some(Token::new(Lexeme::Eof, Location::test(1, 1))),
        ));

        let result =
            Parser::default().parse(TokenStream::test(input).wrap(), None, Location::test(1, 1));

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"true || false"#;

        let expected = Ok((
            ListExpression::new(
                Location::test(1, 1),
                vec![ExpressionTree::new_with_leaves(
                    Location::test(1, 6),
                    ExpressionTreeNode::operator(ExpressionOperator::Or),
                    Some(ExpressionTree::new(
                        Location::test(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::test(1, 1),
                                LexicalBooleanLiteral::r#true(),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 9),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::test(1, 9),
                                LexicalBooleanLiteral::r#false(),
                            ),
                        )),
                    )),
                )],
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 14))),
        ));

        let result =
            Parser::default().parse(TokenStream::test(input).wrap(), None, Location::test(1, 1));

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"true || false,"#;

        let expected = Ok((
            ListExpression::new(
                Location::test(1, 1),
                vec![ExpressionTree::new_with_leaves(
                    Location::test(1, 6),
                    ExpressionTreeNode::operator(ExpressionOperator::Or),
                    Some(ExpressionTree::new(
                        Location::test(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::test(1, 1),
                                LexicalBooleanLiteral::r#true(),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 9),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::test(1, 9),
                                LexicalBooleanLiteral::r#false(),
                            ),
                        )),
                    )),
                )],
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 15))),
        ));

        let result =
            Parser::default().parse(TokenStream::test(input).wrap(), None, Location::test(1, 1));

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"true || false, 42 as field"#;

        let expected = Ok((
            ListExpression::new(
                Location::test(1, 1),
                vec![
                    ExpressionTree::new_with_leaves(
                        Location::test(1, 6),
                        ExpressionTreeNode::operator(ExpressionOperator::Or),
                        Some(ExpressionTree::new(
                            Location::test(1, 1),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                                BooleanLiteral::new(
                                    Location::test(1, 1),
                                    LexicalBooleanLiteral::r#true(),
                                ),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 9),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                                BooleanLiteral::new(
                                    Location::test(1, 9),
                                    LexicalBooleanLiteral::r#false(),
                                ),
                            )),
                        )),
                    ),
                    ExpressionTree::new_with_leaves(
                        Location::test(1, 19),
                        ExpressionTreeNode::operator(ExpressionOperator::Casting),
                        Some(ExpressionTree::new(
                            Location::test(1, 16),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(1, 16),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 22),
                            ExpressionTreeNode::operand(ExpressionOperand::Type(Type::new(
                                Location::test(1, 22),
                                TypeVariant::field(),
                            ))),
                        )),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 27))),
        ));

        let result =
            Parser::default().parse(TokenStream::test(input).wrap(), None, Location::test(1, 1));

        assert_eq!(result, expected);
    }
}
