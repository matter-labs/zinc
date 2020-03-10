//!
//! The array/tuple/structure access operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::list::Parser as ExpressionListParser;
use crate::syntax::parser::expression::path::Parser as PathOperandParser;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::tree::expression::auxiliary::Auxiliary as ExpressionAuxiliary;
use crate::syntax::tree::expression::builder::Builder as ExpressionBuilder;
use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::Expression;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::member_integer::builder::Builder as MemberIntegerBuilder;
use crate::syntax::tree::member_string::builder::Builder as MemberStringBuilder;

#[derive(Debug, Clone, Copy)]
pub enum State {
    PathOperand,
    ExclamationMarkOrNext,
    AccessOrCallOrEnd,
    IndexExpression,
    BracketSquareRight,
    FieldDescriptor,
    ArgumentList,
    ParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::PathOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
    auxiliary: Option<(Location, ExpressionAuxiliary)>,
    next: Option<Token>,
    is_indexed: bool,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Expression, Option<Token>), Error> {
        loop {
            match self.state {
                State::PathOperand => {
                    let (expression, next) =
                        PathOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.set_location(expression.location);
                    self.builder.extend_with_expression(expression);
                    self.state = State::ExclamationMarkOrNext;
                }
                State::ExclamationMarkOrNext => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                            location,
                        } => {
                            self.auxiliary = Some((location, ExpressionAuxiliary::CallBuiltIn));
                        }
                        token => {
                            self.next = Some(token);
                        }
                    }
                    self.state = State::AccessOrCallOrEnd;
                }
                State::AccessOrCallOrEnd => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Index));
                            self.is_indexed = true;
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
                            self.is_indexed = true;
                            self.state = State::FieldDescriptor;
                        }
                        token => {
                            match token.lexeme {
                                Lexeme::Symbol(Symbol::Equals) => {}
                                Lexeme::Symbol(Symbol::PlusEquals)
                                | Lexeme::Symbol(Symbol::MinusEquals)
                                | Lexeme::Symbol(Symbol::AsteriskEquals)
                                | Lexeme::Symbol(Symbol::SlashEquals)
                                | Lexeme::Symbol(Symbol::PercentEquals)
                                    if self.is_indexed =>
                                {
                                    self.builder.push_auxiliary(
                                        token.location,
                                        ExpressionAuxiliary::PlaceCopy,
                                    )
                                }
                                _ if self.is_indexed => self
                                    .builder
                                    .push_auxiliary(token.location, ExpressionAuxiliary::PlaceEnd),
                                _ => {}
                            }
                            return Ok((self.builder.finish(), Some(token)));
                        }
                    }
                }
                State::IndexExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.extend_with_expression(expression);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::BracketSquareRight;
                }
                State::BracketSquareRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec!["]"],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
                State::FieldDescriptor => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme:
                                Lexeme::Literal(lexical::Literal::Integer(
                                    literal @ lexical::IntegerLiteral::Decimal { .. },
                                )),
                            location,
                        } => {
                            let mut builder = MemberIntegerBuilder::default();
                            builder.set_location(location);
                            builder.set_literal(IntegerLiteral::new(location, literal));
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
                            return Err(Error::Syntax(SyntaxError::expected_field_identifier(
                                location, lexeme, None,
                            )))
                        }
                    }
                }
                State::ArgumentList => {
                    let (expression_list, next) =
                        ExpressionListParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operand(
                            location,
                            ExpressionOperand::ExpressionList(expression_list),
                        );
                        if let Some((location, auxiliary)) = self.auxiliary.take() {
                            self.builder.push_auxiliary(location, auxiliary);
                        }
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::ParenthesisRight;
                }
                State::ParenthesisRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec![")"],
                                lexeme,
                                None,
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

    use super::Error;
    use super::Parser;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::auxiliary::Auxiliary as ExpressionAuxiliary;
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::member_integer::MemberInteger;
    use crate::syntax::tree::member_string::MemberString;

    #[test]
    fn ok() {
        let input = r#"array[42].25.value"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                            Location::new(1, 1),
                            "array".to_owned(),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 7),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 7),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 6),
                        ExpressionObject::Operator(ExpressionOperator::Index),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 11),
                        ExpressionObject::Operand(ExpressionOperand::MemberInteger(
                            MemberInteger::new(
                                Location::new(1, 11),
                                IntegerLiteral::new(
                                    Location::new(1, 11),
                                    lexical::IntegerLiteral::new_decimal("25".to_owned()),
                                ),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 10),
                        ExpressionObject::Operator(ExpressionOperator::Field),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 14),
                        ExpressionObject::Operand(ExpressionOperand::MemberString(
                            MemberString::new(Location::new(1, 14), "value".to_owned()),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 13),
                        ExpressionObject::Operator(ExpressionOperator::Field),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 19),
                        ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceEnd),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 19))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"array[42)"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 9),
            vec!["]"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_parenthesis_right() {
        let input = r#"sort(42, 69]"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 12),
            vec![")"],
            Lexeme::Symbol(Symbol::BracketSquareRight),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
