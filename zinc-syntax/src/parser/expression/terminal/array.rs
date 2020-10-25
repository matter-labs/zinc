//!
//! The array expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::expression::Parser as ExpressionParser;
use crate::tree::expression::array::builder::Builder as ArrayExpressionBuilder;
use crate::tree::expression::array::Expression as ArrayExpression;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    BracketSquareLeft,
    /// The `[` has been parsed so far.
    FirstExpressionOrBracketSquareRight,
    /// The `[ {expression},`s has been parsed so far.
    ExpressionOrBracketSquareRight,
    /// The `[ {expression}, {expression}`s has been parsed so far.
    CommaOrBracketSquareRight,
    /// The `[ {expression}` has been parsed so far.
    CommaOrSemicolonOrBracketSquareRight,
    /// The `[ {expression} ;` has been parsed so far.
    SizeExpression,
    /// The `[ {expression} ; {expression}` has been parsed so far.
    BracketSquareRight,
}

impl Default for State {
    fn default() -> Self {
        Self::BracketSquareLeft
    }
}

///
/// The array expression parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed value.
    builder: ArrayExpressionBuilder,
}

impl Parser {
    ///
    /// Parses an array literal.
    ///
    /// '[1, 2, 3]'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ArrayExpression, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::BracketSquareLeft => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::FirstExpressionOrBracketSquareRight;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["["],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::FirstExpressionOrBracketSquareRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            return Ok((self.builder.finish(), self.next.take()));
                        }
                        token => {
                            let (expression, next) =
                                ExpressionParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_expression(expression);
                            self.state = State::CommaOrSemicolonOrBracketSquareRight;
                        }
                    }
                }
                State::ExpressionOrBracketSquareRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            return Ok((self.builder.finish(), self.next.take()));
                        }
                        token => {
                            let (expression, next) =
                                ExpressionParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_expression(expression);
                            self.state = State::CommaOrBracketSquareRight;
                        }
                    }
                }
                State::CommaOrBracketSquareRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => {
                            self.state = State::ExpressionOrBracketSquareRight;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            return Ok((self.builder.finish(), self.next.take()));
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![",", "]"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::CommaOrSemicolonOrBracketSquareRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => {
                            self.state = State::ExpressionOrBracketSquareRight;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => {
                            self.state = State::SizeExpression;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            return Ok((self.builder.finish(), self.next.take()));
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![",", ";", "]"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::SizeExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;

                    self.next = next;
                    self.builder.set_size_expression(expression);
                    self.state = State::BracketSquareRight;
                }
                State::BracketSquareRight => {
                    return match crate::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            return Ok((self.builder.finish(), self.next.take()));
                        }
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of_or_operator(
                                location,
                                vec!["]"],
                                lexeme,
                                None,
                            ),
                        )),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::array::Expression as ArrayExpression;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_empty() {
        let input = r#"[]"#;

        let expected = Ok((
            ArrayExpression::new_list(Location::test(1, 1), vec![]),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"[1]"#;

        let expected = Ok((
            ArrayExpression::new_list(
                Location::test(1, 1),
                vec![ExpressionTree::new(
                    Location::test(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 2),
                            LexicalIntegerLiteral::new_decimal("1".to_owned()),
                        ),
                    )),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"[1, 2, 3]"#;

        let expected = Ok((
            ArrayExpression::new_list(
                Location::test(1, 1),
                vec![
                    ExpressionTree::new(
                        Location::test(1, 2),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 2),
                                LexicalIntegerLiteral::new_decimal("1".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionTree::new(
                        Location::test(1, 5),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 5),
                                LexicalIntegerLiteral::new_decimal("2".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionTree::new(
                        Location::test(1, 8),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 8),
                                LexicalIntegerLiteral::new_decimal("3".to_owned()),
                            ),
                        )),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_with_size_expression() {
        let input = r#"[1; 10]"#;

        let expected = Ok((
            ArrayExpression::new_repeated(
                Location::test(1, 1),
                ExpressionTree::new(
                    Location::test(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 2),
                            LexicalIntegerLiteral::new_decimal("1".to_owned()),
                        ),
                    )),
                ),
                ExpressionTree::new(
                    Location::test(1, 5),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 5),
                            LexicalIntegerLiteral::new_decimal("10".to_owned()),
                        ),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_semicolon_or_bracket_square_right() {
        let input = r#"[42)"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 4),
                vec![",", ";", "]"],
                Lexeme::Symbol(Symbol::ParenthesisRight),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_bracket_square_right() {
        let input = r#"[42, 64)"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 8),
                vec![",", "]"],
                Lexeme::Symbol(Symbol::ParenthesisRight),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"[42; 8)"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 7),
                vec!["]"],
                Lexeme::Symbol(Symbol::ParenthesisRight),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
