//!
//! The path expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Identifier,
    DoubleColonOrEnd,
}

impl Default for State {
    fn default() -> Self {
        State::Identifier
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
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
        mut initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), Error> {
        loop {
            match self.state {
                State::Identifier => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
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
                            lexeme: Lexeme::Keyword(keyword @ Keyword::SelfLowercase),
                            location,
                        } => {
                            let identifier = Identifier::new(location, keyword.to_string());
                            self.builder
                                .eat_operand(ExpressionOperand::Identifier(identifier), location);
                            self.state = State::DoubleColonOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::SelfUppercase),
                            location,
                        } => {
                            let identifier = Identifier::new(location, keyword.to_string());
                            self.builder
                                .eat_operand(ExpressionOperand::Identifier(identifier), location);
                            self.state = State::DoubleColonOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location, lexeme, None,
                            )));
                        }
                    }
                }
                State::DoubleColonOrEnd => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;

    #[test]
    fn ok_single() {
        let input = r#"id;"#;

        let expected = Ok((
            ExpressionTree::new(
                Location::new(1, 1),
                ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                    Location::new(1, 1),
                    "id".to_owned(),
                ))),
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::new(1, 3),
            )),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"mega::ultra::namespace;"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::new(1, 12),
                ExpressionTreeNode::operator(ExpressionOperator::Path),
                Some(ExpressionTree::new_with_leaves(
                    Location::new(1, 5),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new(
                        Location::new(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::new(1, 1), "mega".to_owned()),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 7),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::new(1, 7), "ultra".to_owned()),
                        )),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 14),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 14),
                        "namespace".to_owned(),
                    ))),
                )),
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::new(1, 23),
            )),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
