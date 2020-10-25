//!
//! The multiplication/division/remainder operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::casting::Parser as CastingOperandParser;
use crate::parser::r#type::Parser as TypeParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    CastingFirstOperand,
    /// The first operand has been parsed and an operator is expected.
    CastingOperator,
    /// The first operand and the operator have been parsed, and the second operand is expected.
    CastingSecondOperand,
}

impl Default for State {
    fn default() -> Self {
        Self::CastingFirstOperand
    }
}

///
/// The multiplication/division/remainder operand parser.
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
    /// Parses a binary multiplication, division or remainder expression operand, which is
    /// a lower precedence casting operator expression.
    ///
    /// '42 as field'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::CastingFirstOperand => {
                    let (expression, next) =
                        CastingOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::CastingOperator;
                }
                State::CastingOperator => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::As),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Casting, location);
                            self.state = State::CastingSecondOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::CastingSecondOperand => {
                    let (r#type, next) =
                        TypeParser::default().parse(stream.clone(), self.next.take())?;
                    let location = r#type.location;
                    self.next = next;
                    self.builder
                        .eat_operand(ExpressionOperand::Type(r#type), location);
                    self.state = State::CastingOperator;
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
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok() {
        let input = r#"42 as field"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Casting),
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
                    ExpressionTreeNode::operand(ExpressionOperand::Type(Type::new(
                        Location::test(1, 7),
                        TypeVariant::field(),
                    ))),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 12))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
