//!
//! The expression (function argument) list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Default)]
pub struct Parser {
    expressions: Vec<ExpressionTree>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Vec<ExpressionTree>, Location, Option<Token>), Error> {
        loop {
            match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                token
                @
                Token {
                    lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                    ..
                } => {
                    let location = self
                        .expressions
                        .first()
                        .map(|expression| expression.location)
                        .unwrap_or_default();
                    return Ok((self.expressions, location, Some(token)));
                }
                token
                @
                Token {
                    lexeme: Lexeme::Eof,
                    ..
                } => {
                    let location = self
                        .expressions
                        .first()
                        .map(|expression| expression.location)
                        .unwrap_or_default();
                    return Ok((self.expressions, location, Some(token)));
                }
                token => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    self.expressions.push(expression);
                }
            }

            match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => continue,
                token => {
                    let location = self
                        .expressions
                        .first()
                        .map(|expression| expression.location)
                        .unwrap_or_default();
                    return Ok((self.expressions, location, Some(token)));
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
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
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
            Vec::<ExpressionTree>::new(),
            Location::new(0, 0),
            Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"true || false"#;

        let expected = Ok((
            vec![ExpressionTree::new(
                Location::new(1, 6),
                ExpressionTreeNode::operator(ExpressionOperator::Or),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::r#true()),
                    )),
                    None,
                    None,
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 9),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(
                            Location::new(1, 9),
                            lexical::BooleanLiteral::r#false(),
                        ),
                    )),
                    None,
                    None,
                )),
            )],
            Location::new(1, 6),
            Some(Token::new(Lexeme::Eof, Location::new(1, 14))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"true || false,"#;

        let expected = Ok((
            vec![ExpressionTree::new(
                Location::new(1, 6),
                ExpressionTreeNode::operator(ExpressionOperator::Or),
                Some(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::r#true()),
                    )),
                    None,
                    None,
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 9),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(
                            Location::new(1, 9),
                            lexical::BooleanLiteral::r#false(),
                        ),
                    )),
                    None,
                    None,
                )),
            )],
            Location::new(1, 6),
            Some(Token::new(Lexeme::Eof, Location::new(1, 15))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"true || false, 42 as field"#;

        let expected = Ok((
            vec![
                ExpressionTree::new(
                    Location::new(1, 6),
                    ExpressionTreeNode::operator(ExpressionOperator::Or),
                    Some(ExpressionTree::new(
                        Location::new(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 1),
                                lexical::BooleanLiteral::r#true(),
                            ),
                        )),
                        None,
                        None,
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 9),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 9),
                                lexical::BooleanLiteral::r#false(),
                            ),
                        )),
                        None,
                        None,
                    )),
                ),
                ExpressionTree::new(
                    Location::new(1, 19),
                    ExpressionTreeNode::operator(ExpressionOperator::Casting),
                    Some(ExpressionTree::new(
                        Location::new(1, 16),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 16),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                        None,
                        None,
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 22),
                        ExpressionTreeNode::operand(ExpressionOperand::Type(Type::new(
                            Location::new(1, 22),
                            TypeVariant::field(),
                        ))),
                        None,
                        None,
                    )),
                ),
            ],
            Location::new(1, 6),
            Some(Token::new(Lexeme::Eof, Location::new(1, 27))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
