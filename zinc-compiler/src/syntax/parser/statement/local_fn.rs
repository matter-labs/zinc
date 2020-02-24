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
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::parser::statement::r#const::Parser as ConstStatementParser;
use crate::syntax::parser::statement::r#let::Parser as LetStatementParser;
use crate::syntax::parser::statement::r#loop::Parser as LoopStatementParser;
use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;

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
        let statement = match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
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
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                ..
            } => return Ok((FunctionLocalStatement::Empty, None, false)),
            token => {
                let (expression, next) =
                    ExpressionParser::default().parse(stream.clone(), Some(token))?;
                self.next = next;
                FunctionLocalStatement::Expression(expression)
            }
        };
        match statement {
            statement @ FunctionLocalStatement::Expression { .. } => {
                match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
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
    use crate::syntax::tree::expression::block::Expression as BlockExpression;
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;
    use crate::syntax::tree::statement::r#let::Statement as LetStatement;

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
                    TypeVariant::integer_unsigned(232),
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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
