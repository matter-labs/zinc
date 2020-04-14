//!
//! The use statement parser.
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
use crate::syntax::parser::expression::path::Parser as PathOperandParser;
use crate::syntax::tree::statement::r#use::builder::Builder as UseStatementBuilder;
use crate::syntax::tree::statement::r#use::Statement as UseStatement;

#[derive(Default)]
pub struct Parser {
    builder: UseStatementBuilder,
}

impl Parser {
    ///
    /// Parses a 'use' statement.
    ///
    /// 'use jabberwocky::existence;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(UseStatement, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Use),
                location,
            } => {
                self.builder.set_location(location);
            }
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::expected_one_of(
                    location,
                    vec!["use"],
                    lexeme,
                    None,
                )));
            }
        }

        let (expression, mut next) = PathOperandParser::default().parse(stream.clone(), None)?;
        self.builder.set_path(expression);

        match crate::syntax::parser::take_or_next(next.take(), stream)? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                ..
            } => Ok((self.builder.finish(), None)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::expected_one_of(
                location,
                vec![";"],
                lexeme,
                None,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::statement::r#use::Statement as UseStatement;

    #[test]
    fn ok() {
        let input = r#"use mega::ultra::namespace;"#;

        let expected = Ok((
            UseStatement::new(
                Location::new(1, 1),
                ExpressionTree::new_with_leaves(
                    Location::new(1, 16),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new_with_leaves(
                        Location::new(1, 9),
                        ExpressionTreeNode::operator(ExpressionOperator::Path),
                        Some(ExpressionTree::new(
                            Location::new(1, 5),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 5), "mega".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::new(1, 11),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 11), "ultra".to_owned()),
                            )),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 18),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::new(1, 18), "namespace".to_owned()),
                        )),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"use jabberwocky"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 16),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
