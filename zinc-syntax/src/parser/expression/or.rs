//!
//! The logical OR operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::xor::Parser as XorOperandParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    LogicalXorOperand,
    /// The operand has been parsed and an operator is expected.
    LogicalXorOperator,
}

impl Default for State {
    fn default() -> Self {
        Self::LogicalXorOperand
    }
}

///
/// The logical OR operand parser.
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
    /// Parses a logical OR expression operand, which is
    /// a lower precedence logical XOR operator expression.
    ///
    /// 'true ^^ false'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::LogicalXorOperand => {
                    let (expression, next) =
                        XorOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::LogicalXorOperator;
                }
                State::LogicalXorOperator => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleCircumflex),
                            location,
                        } => {
                            self.builder.eat_operator(ExpressionOperator::Xor, location);
                            self.state = State::LogicalXorOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::BooleanLiteral as LexicalBooleanLiteral;
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

    #[test]
    fn ok() {
        let input = r#"true ^^ false"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 6),
                ExpressionTreeNode::operator(ExpressionOperator::Xor),
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
}
