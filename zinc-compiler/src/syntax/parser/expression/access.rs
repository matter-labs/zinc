//!
//! The array/tuple/structure access operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ArrayExpressionParser;
use crate::syntax::BlockExpressionParser;
use crate::syntax::BooleanLiteral;
use crate::syntax::ConditionalExpressionParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionListParser;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchExpressionParser;
use crate::syntax::MemberIntegerBuilder;
use crate::syntax::MemberStringBuilder;
use crate::syntax::PathExpressionParser;
use crate::syntax::StringLiteral;
use crate::syntax::StructureExpressionParser;
use crate::syntax::TupleExpressionParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Operand,
    InstructionCallOrNext,
    AccessOrCallOrEnd,
    IndexExpression,
    BracketSquareRight,
    FieldDescriptor,
    ArgumentList,
    ParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::Operand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Expression, Option<Token>), Error> {
        loop {
            match self.state {
                State::Operand => {
                    let next = match initial.take() {
                        Some(next) => next,
                        None => stream.borrow_mut().next()?,
                    };
                    match next {
                        token @ Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let expression = TupleExpressionParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.builder.extend_with_expression(expression);
                            self.state = State::AccessOrCallOrEnd;
                        }
                        token @ Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let block = BlockExpressionParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.builder
                                .push_operand(block.location, ExpressionOperand::Block(block));
                            self.state = State::AccessOrCallOrEnd;
                        }
                        token @ Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let array = ArrayExpressionParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.builder
                                .push_operand(array.location, ExpressionOperand::Array(array));
                            self.state = State::AccessOrCallOrEnd;
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::If),
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let (expression, next) = ConditionalExpressionParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_operand(
                                expression.location,
                                ExpressionOperand::Conditional(expression),
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::Match),
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let expression = MatchExpressionParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.builder.push_operand(
                                expression.location,
                                ExpressionOperand::Match(expression),
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::Struct),
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let (expression, next) = StructureExpressionParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_operand(
                                expression.location,
                                ExpressionOperand::Structure(expression),
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Literal(lexical::Literal::Boolean(boolean)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::BooleanLiteral(BooleanLiteral::new(
                                    location, boolean,
                                )),
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Literal(lexical::Literal::Integer(integer)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::IntegerLiteral(IntegerLiteral::new(
                                    location, integer,
                                )),
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Literal(lexical::Literal::String(string)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::StringLiteral(StringLiteral::new(
                                    location, string,
                                )),
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        token @ Token {
                            lexeme: Lexeme::Identifier(_),
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let (expression, next) = PathExpressionParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.extend_with_expression(expression);
                            self.state = State::InstructionCallOrNext;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![
                                    "(",
                                    "{",
                                    "[",
                                    "if",
                                    "match",
                                    "struct",
                                    "{literal}",
                                    "{identifier}",
                                ],
                                lexeme,
                            )))
                        }
                    }
                }
                State::InstructionCallOrNext => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                            location,
                        } => self
                            .builder
                            .push_operator(location, ExpressionOperator::InstructionCall),
                        token => self.next = Some(token),
                    }
                    self.state = State::AccessOrCallOrEnd;
                }
                State::AccessOrCallOrEnd => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Indexing));
                            self.state = State::IndexExpression;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Call));
                            self.state = State::ArgumentList;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Dot),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Field));
                            self.state = State::FieldDescriptor;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::IndexExpression => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Literal(lexical::Literal::Integer(integer)),
                            location,
                        } => {
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::IntegerLiteral(IntegerLiteral::new(
                                    location, integer,
                                )),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::BracketSquareRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::BracketSquareRight => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["]"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::FieldDescriptor => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme:
                                Lexeme::Literal(lexical::Literal::Integer(
                                    literal @ lexical::IntegerLiteral::Decimal { .. },
                                )),
                            location,
                        } => {
                            let mut builder = MemberIntegerBuilder::default();
                            builder.set_location(location);
                            builder.set_literal(literal);
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::MemberInteger(builder.finish()),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let mut builder = MemberStringBuilder::default();
                            builder.set_location(location);
                            builder.set_name(identifier.name);
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::MemberString(builder.finish()),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{decimal}", "{identifier}"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::ArgumentList => {
                    let (expression_list, next) =
                        ExpressionListParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder
                            .push_operand(location, ExpressionOperand::List(expression_list));
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::ParenthesisRight;
                }
                State::ParenthesisRight => {
                    match self
                        .next
                        .take()
                        .expect(crate::syntax::PANIC_VALUE_ALWAYS_EXISTS)
                    {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![")"],
                                lexeme,
                            )))
                        }
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
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::IntegerLiteral;

    #[test]
    fn ok() {
        let input = r#"42"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::IntegerLiteral(
                        IntegerLiteral::new(
                            Location::new(1, 1),
                            lexical::IntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 3))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
