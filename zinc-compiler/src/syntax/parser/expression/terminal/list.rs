//!
//! The expression (function argument) list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::location::Location;
use crate::lexical::token::Token;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::tree::expression::list::builder::Builder as ListExpressionBuilder;
use crate::syntax::tree::expression::list::Expression as ListExpression;

#[derive(Default)]
pub struct Parser {
    builder: ListExpressionBuilder,
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
        mut initial: Option<Token>,
        location: Location,
    ) -> Result<(ListExpression, Option<Token>), Error> {
        self.builder.set_location(location);
        loop {
            match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
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

            match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::boolean::Boolean as LexicalBooleanLiteral;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::tree::expression::list::Expression as ListExpression;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            ListExpression::new(Location::new(1, 1), vec![]),
            Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input))),
            None,
            Location::new(1, 1),
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"true || false"#;

        let expected = Ok((
            ListExpression::new(
                Location::new(1, 1),
                vec![ExpressionTree::new_with_leaves(
                    Location::new(1, 6),
                    ExpressionTreeNode::operator(ExpressionOperator::Or),
                    Some(ExpressionTree::new(
                        Location::new(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 1),
                                LexicalBooleanLiteral::r#true(),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 9),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 9),
                                LexicalBooleanLiteral::r#false(),
                            ),
                        )),
                    )),
                )],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 14))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input))),
            None,
            Location::new(1, 1),
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"true || false,"#;

        let expected = Ok((
            ListExpression::new(
                Location::new(1, 1),
                vec![ExpressionTree::new_with_leaves(
                    Location::new(1, 6),
                    ExpressionTreeNode::operator(ExpressionOperator::Or),
                    Some(ExpressionTree::new(
                        Location::new(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 1),
                                LexicalBooleanLiteral::r#true(),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 9),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 9),
                                LexicalBooleanLiteral::r#false(),
                            ),
                        )),
                    )),
                )],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 15))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input))),
            None,
            Location::new(1, 1),
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"true || false, 42 as field"#;

        let expected = Ok((
            ListExpression::new(
                Location::new(1, 1),
                vec![
                    ExpressionTree::new_with_leaves(
                        Location::new(1, 6),
                        ExpressionTreeNode::operator(ExpressionOperator::Or),
                        Some(ExpressionTree::new(
                            Location::new(1, 1),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                                BooleanLiteral::new(
                                    Location::new(1, 1),
                                    LexicalBooleanLiteral::r#true(),
                                ),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 9),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                                BooleanLiteral::new(
                                    Location::new(1, 9),
                                    LexicalBooleanLiteral::r#false(),
                                ),
                            )),
                        )),
                    ),
                    ExpressionTree::new_with_leaves(
                        Location::new(1, 19),
                        ExpressionTreeNode::operator(ExpressionOperator::Casting),
                        Some(ExpressionTree::new(
                            Location::new(1, 16),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 16),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 22),
                            ExpressionTreeNode::operand(ExpressionOperand::Type(Type::new(
                                Location::new(1, 22),
                                TypeVariant::field(),
                            ))),
                        )),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 27))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input))),
            None,
            Location::new(1, 1),
        );

        assert_eq!(result, expected);
    }
}
