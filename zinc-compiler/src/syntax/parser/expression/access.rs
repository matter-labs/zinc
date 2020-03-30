//!
//! The array/tuple/structure access operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::path::Parser as PathOperandParser;
use crate::syntax::parser::expression::terminal::list::Parser as ExpressionListParser;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
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
    next: Option<Token>,
    builder: ExpressionTreeBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), Error> {
        loop {
            match self.state {
                State::PathOperand => {
                    let (expression, next) =
                        PathOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::ExclamationMarkOrNext;
                }
                State::ExclamationMarkOrNext => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::CallBuiltIn, location);
                        }
                        token => self.next = Some(token),
                    }
                    self.state = State::AccessOrCallOrEnd;
                }
                State::AccessOrCallOrEnd => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Index, location);
                            self.state = State::IndexExpression;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Call, location);
                            self.state = State::ArgumentList;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Dot),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Field, location);
                            self.state = State::FieldDescriptor;
                        }
                        token => {
                            return Ok((self.builder.finish(), Some(token)));
                        }
                    }
                }
                State::IndexExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(expression);
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
                            self.builder.eat_operand(
                                ExpressionOperand::MemberInteger(builder.finish()),
                                location,
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let mut builder = MemberStringBuilder::default();
                            builder.set_location(location);
                            builder.set_name(identifier.name);
                            self.builder.eat_operand(
                                ExpressionOperand::MemberString(builder.finish()),
                                location,
                            );
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
                    let (expressions, location, next) =
                        ExpressionListParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder
                        .eat_operand(ExpressionOperand::List(expressions), location);
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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;

    #[test]
    fn ok() {
        let input = r#"mega::ultra::namespace;"#;

        let expected = Ok((
            ExpressionTree::new(
                Location::new(1, 12),
                ExpressionTreeNode::operator(ExpressionOperator::Path),
                Some(ExpressionTree::new(
                    Location::new(1, 5),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new(
                        Location::new(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::new(1, 1), "mega".to_owned()),
                        )),
                        None,
                        None,
                    )),
                    Some(ExpressionTree::new(
                        Location::new(1, 7),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::new(1, 7), "ultra".to_owned()),
                        )),
                        None,
                        None,
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::new(1, 14),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 14),
                        "namespace".to_owned(),
                    ))),
                    None,
                    None,
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
