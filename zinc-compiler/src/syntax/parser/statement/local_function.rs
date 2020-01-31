//!
//! The function-local statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ConstStatementParser;
use crate::syntax::ExpressionParser;
use crate::syntax::FunctionLocalStatement;
use crate::syntax::LetStatementParser;
use crate::syntax::LoopStatementParser;

#[derive(Default)]
pub struct Parser {
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(FunctionLocalStatement, Option<Token>, bool), Error> {
        let statement = match crate::syntax::take_or_next(initial.take(), stream.clone())? {
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Let),
                ..
            } => {
                let (statement, next) =
                    LetStatementParser::default().parse(stream.clone(), Some(token))?;
                self.next = next;
                FunctionLocalStatement::Let(statement)
            }
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Const),
                ..
            } => {
                let (statement, next) =
                    ConstStatementParser::default().parse(stream.clone(), Some(token))?;
                self.next = next;
                FunctionLocalStatement::Const(statement)
            }
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::For),
                ..
            } => {
                let statement =
                    LoopStatementParser::default().parse(stream.clone(), Some(token))?;
                FunctionLocalStatement::Loop(statement)
            }
            token => {
                let (expression, next) =
                    ExpressionParser::default().parse(stream.clone(), Some(token))?;
                self.next = next;
                FunctionLocalStatement::Expression(expression)
            }
        };
        match statement {
            statement @ FunctionLocalStatement::Expression { .. } => {
                match crate::syntax::take_or_next(self.next.take(), stream)? {
                    Token {
                        lexeme: Lexeme::Symbol(Symbol::Semicolon),
                        ..
                    } => Ok((statement, None, false)),
                    token => Ok((statement, Some(token), true)),
                }
            }
            statement => Ok((statement, None, false)),
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
    use crate::syntax::BlockExpression;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::FunctionLocalStatement;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::LetStatement;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_semicolon_terminated() {
        let input = r#"let mut a: u232 = 42;"#;

        let expected = Ok((
            FunctionLocalStatement::Let(LetStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 9), "a".to_owned()),
                true,
                Some(Type::new(
                    Location::new(1, 12),
                    TypeVariant::new_integer_unsigned(232),
                )),
                Expression::new(
                    Location::new(1, 19),
                    vec![ExpressionElement::new(
                        Location::new(1, 19),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 19),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    )],
                ),
            )),
            None,
            false,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_semicolon_unterminated() {
        let input = r#"{ 42 }"#;

        let expected = Ok((
            FunctionLocalStatement::Expression(Expression::new(
                Location::new(1, 1),
                vec![ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::Block(BlockExpression::new(
                        Location::new(1, 1),
                        vec![],
                        Some(Expression::new(
                            Location::new(1, 3),
                            vec![ExpressionElement::new(
                                Location::new(1, 3),
                                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::new(1, 3),
                                        lexical::IntegerLiteral::new_decimal("42".to_owned()),
                                    ),
                                )),
                            )],
                        )),
                    ))),
                )],
            )),
            Some(Token::new(Lexeme::Eof, Location::new(1, 7))),
            true,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
