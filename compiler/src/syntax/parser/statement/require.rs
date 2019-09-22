//!
//! The require statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::RequireStatement;
use crate::syntax::RequireStatementBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordRequire,
    BracketOpen,
    Expression,
    CommaOrBracketClose,
    Tag,
    BracketClose,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordRequire
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: RequireStatementBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<RequireStatement, Error> {
        loop {
            match self.state {
                State::KeywordRequire => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Keyword(Keyword::Require),
                        location,
                    })) => {
                        self.builder.set_location(location);
                        self.state = State::BracketOpen;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["require"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::BracketOpen => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                        ..
                    })) => self.state = State::Expression,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["("].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Expression => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_expression(expression);
                    self.state = State::CommaOrBracketClose;
                }
                State::CommaOrBracketClose => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Comma),
                        ..
                    })) => self.state = State::Tag,
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                        ..
                    })) => return Ok(self.builder.finish()),
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [",", ")"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Tag => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Literal(Literal::String(tag)),
                        ..
                    })) => {
                        self.builder.set_tag(tag);
                        self.state = State::BracketClose;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["{string}"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::BracketClose => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                        ..
                    })) => return Ok(self.builder.finish()),
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [")"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
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
    use crate::lexical::BooleanLiteral;
    use crate::lexical::Location;
    use crate::lexical::StringLiteral;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::RequireStatement;

    #[test]
    fn ok() {
        let code = r#"require(true, "test")"#;

        let expected = RequireStatement::new(
            Location::new(1, 1),
            Expression::Operator(OperatorExpression::new(
                Location::new(1, 9),
                vec![OperatorExpressionElement::new(
                    Location::new(1, 9),
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::new(
                            Location::new(1, 9),
                            lexical::Literal::Boolean(BooleanLiteral::True),
                        ),
                    )),
                )],
            )),
            Some(StringLiteral::new("test".to_owned())),
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
