//!
//! The assignment operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::or::Parser as OrOperandParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    LogicalOrOperand,
    /// The operand has been parsed and an operator is expected.
    LogicalOrOperator,
}

impl Default for State {
    fn default() -> Self {
        Self::LogicalOrOperand
    }
}

///
/// The assignment operand parser.
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
    /// Parses a range expression operand, which is
    /// a lower precedence logical OR operator expression.
    ///
    /// 'true || false'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::LogicalOrOperand => {
                    let (expression, next) =
                        OrOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::LogicalOrOperator;
                }
                State::LogicalOrOperator => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleVerticalBar),
                            location,
                        } => {
                            self.builder.eat_operator(ExpressionOperator::Or, location);
                            self.state = State::LogicalOrOperand;
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
        let input = r#"true || false"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 6),
                ExpressionTreeNode::operator(ExpressionOperator::Or),
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
