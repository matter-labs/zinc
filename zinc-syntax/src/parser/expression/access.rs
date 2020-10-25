//!
//! The array/tuple/structure access operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
use zinc_lexical::Lexeme;
use zinc_lexical::Literal as LexicalLiteral;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::expression::path::Parser as PathOperandParser;
use crate::parser::expression::terminal::list::Parser as ExpressionListParser;
use crate::parser::expression::Parser as ExpressionParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::builder::Builder as IdentifierBuilder;
use crate::tree::literal::integer::Literal as IntegerLiteral;
use crate::tree::tuple_index::builder::Builder as TupleIndexBuilder;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    PathOperand,
    /// The first path operand has been parsed so far.
    /// The optional exclamation mark quasi-operator identifies the intrinsic function call.
    ExclamationMarkOrNext,
    /// The first path operand with an optional exclamation mark has been parsed so far.
    /// Expects one of the access or call operators `(`, `[`, `.`.
    AccessOrCallOrEnd,
    /// The `{identifier} [` has been parsed so far.
    IndexExpression,
    /// The `{identifier} [ {expression}` has been parsed so far.
    BracketSquareRight,
    /// The `{identifier} .` has been parsed so far.
    FieldDescriptor,
    /// The `{identifier} (` with several `{expression} ,`s has been parsed so far.
    ParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        Self::PathOperand
    }
}

///
/// The array/tuple/structure access operand parser.
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
    /// Parses an access operator expression, which is a lowest-level terminal operand.
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::PathOperand => {
                    let (expression, next) =
                        PathOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(expression);
                    self.state = State::ExclamationMarkOrNext;
                }
                State::ExclamationMarkOrNext => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::CallIntrinsic, location);
                        }
                        token => self.next = Some(token),
                    }
                    self.state = State::AccessOrCallOrEnd;
                }
                State::AccessOrCallOrEnd => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
                            let (expression, next) = ExpressionListParser::default().parse(
                                stream.clone(),
                                self.next.take(),
                                location,
                            )?;
                            self.next = next;
                            self.builder
                                .eat_operator(ExpressionOperator::Call, location);
                            self.builder
                                .eat_operand(ExpressionOperand::List(expression), location);
                            self.state = State::ParenthesisRight;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Dot),
                            location,
                        } => {
                            self.builder.eat_operator(ExpressionOperator::Dot, location);
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
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_one_of_or_operator(
                                    location,
                                    vec!["]"],
                                    lexeme,
                                    None,
                                ),
                            ))
                        }
                    }
                }
                State::FieldDescriptor => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme:
                                Lexeme::Literal(LexicalLiteral::Integer(
                                    literal @ LexicalIntegerLiteral::Decimal { .. },
                                )),
                            location,
                        } => {
                            let mut builder = TupleIndexBuilder::default();
                            builder.set_location(location);
                            builder.set_literal(IntegerLiteral::new(location, literal));
                            self.builder.eat_operand(
                                ExpressionOperand::TupleIndex(builder.finish()),
                                location,
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let mut builder = IdentifierBuilder::default();
                            builder.set_location(location);
                            builder.set_name(identifier.inner);
                            self.builder.eat_operand(
                                ExpressionOperand::Identifier(builder.finish()),
                                location,
                            );
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_field_identifier(location, lexeme, None),
                            ))
                        }
                    }
                }
                State::ParenthesisRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_one_of_or_operator(
                                    location,
                                    vec![")"],
                                    lexeme,
                                    None,
                                ),
                            ))
                        }
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
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;

    #[test]
    fn ok() {
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

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"array[42)"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 9),
                vec!["]"],
                Lexeme::Symbol(Symbol::ParenthesisRight),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_parenthesis_right() {
        let input = r#"sort(42, 64]"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 12),
                vec![")"],
                Lexeme::Symbol(Symbol::BracketSquareRight),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
