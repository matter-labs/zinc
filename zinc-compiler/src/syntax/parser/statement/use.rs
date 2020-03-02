//!
//! The use statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::path::Parser as PathOperandParser;
use crate::syntax::tree::statement::r#use::builder::Builder as UseStatementBuilder;
use crate::syntax::tree::statement::r#use::Statement as UseStatement;

#[derive(Default)]
pub struct Parser {
    builder: UseStatementBuilder,
}

impl Parser {
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

        let (path, mut next) = PathOperandParser::default().parse(stream.clone(), None)?;
        self.builder.set_path(path);

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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::statement::r#use::Statement as UseStatement;

    #[test]
    fn ok() {
        let input = r#"use mega::ultra::namespace;"#;

        let expected = Ok((
            UseStatement::new(
                Location::new(1, 1),
                Expression::new(
                    Location::new(1, 5),
                    vec![
                        ExpressionElement::new(
                            Location::new(1, 5),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 5), "mega".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 11),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 11), "ultra".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 9),
                            ExpressionObject::Operator(ExpressionOperator::Path),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 18),
                            ExpressionObject::Operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::new(1, 18), "namespace".to_owned()),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 16),
                            ExpressionObject::Operator(ExpressionOperator::Path),
                        ),
                    ],
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = "use jabberwocky";

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
