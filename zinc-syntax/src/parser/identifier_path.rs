//!
//! The identifier path expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Identifier,
    /// The operand has been parsed and a `::` operator is expected.
    DoubleColonOrEnd,
}

impl Default for State {
    fn default() -> Self {
        Self::Identifier
    }
}

///
/// The type path expression parser.
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
    /// Parses a path type literal.
    ///
    /// 'Path::To::Type`
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::Identifier => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder
                                .eat_operand(ExpressionOperand::Identifier(identifier), location);
                            self.state = State::DoubleColonOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::Crate),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::Super),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::SelfLowercase),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::SelfUppercase),
                            location,
                        } => {
                            let identifier = Identifier::new(location, keyword.to_string());
                            self.builder
                                .eat_operand(ExpressionOperand::Identifier(identifier), location);
                            self.state = State::DoubleColonOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                                location, lexeme, None,
                            )));
                        }
                    }
                }
                State::DoubleColonOrEnd => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleColon),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Path, location);
                            self.state = State::Identifier;
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
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;

    #[test]
    fn ok_single() {
        let input = r#"id;"#;

        let expected = Ok((
            ExpressionTree::new(
                Location::test(1, 1),
                ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                    Location::test(1, 1),
                    "id".to_owned(),
                ))),
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::test(1, 3),
            )),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"mega::ultra::namespace;"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 12),
                ExpressionTreeNode::operator(ExpressionOperator::Path),
                Some(ExpressionTree::new_with_leaves(
                    Location::test(1, 5),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new(
                        Location::test(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::test(1, 1), "mega".to_owned()),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 7),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::test(1, 7), "ultra".to_owned()),
                        )),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 14),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(1, 14),
                        "namespace".to_owned(),
                    ))),
                )),
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::test(1, 23),
            )),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
