//!
//! The casting operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::access::Parser as AccessOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Default)]
pub struct Parser {
    builder: ExpressionTreeBuilder,
}

impl Parser {
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
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::member_integer::MemberInteger;
    use crate::syntax::tree::member_string::MemberString;

    #[test]
    fn ok() {
        let input = r#"array[42].25.value"#;

        let expected = Ok((
            ExpressionTree::new(
                Location::new(1, 13),
                ExpressionTreeNode::operator(ExpressionOperator::Field),
                Some(ExpressionTree::new(
                    Location::new(1, 10),
                    ExpressionTreeNode::operator(ExpressionOperator::Field),
                    Some(ExpressionTree::new(
                        Location::new(1, 6),
                        ExpressionTreeNode::operator(ExpressionOperator::Index),
                        Some(ExpressionTree::new(
                            Location::new(1, 1),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 1), "array".to_owned()),
                            )),
                            None,
                            None,
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 7),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 7),
                                    lexical::IntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                            None,
                            None,
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 11),
                        ExpressionTreeNode::operand(ExpressionOperand::MemberInteger(
                            MemberInteger::new(
                                Location::new(1, 11),
                                IntegerLiteral::new(
                                    Location::new(1, 11),
                                    lexical::IntegerLiteral::new_decimal("25".to_owned()),
                                ),
                            ),
                        )),
                        None,
                        None,
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 14),
                    ExpressionTreeNode::operand(ExpressionOperand::MemberString(
                        MemberString::new(Location::new(1, 14), "value".to_owned()),
                    )),
                    None,
                    None,
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 19))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
