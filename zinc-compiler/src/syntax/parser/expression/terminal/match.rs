//!
//! The match expression parser.
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
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::parser::pattern_match::Parser as MatchPatternParser;
use crate::syntax::tree::expression::r#match::builder::Builder as MatchExpressionBuilder;
use crate::syntax::tree::expression::r#match::Expression as MatchExpression;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordMatch,
    ScrutineeExpression,
    BracketCurlyLeft,
    BracketCurlyRightOrBranchPattern,
    Select,
    BranchExpression,
    CommaOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordMatch
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: MatchExpressionBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(MatchExpression, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordMatch => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Match),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::ScrutineeExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["match"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::ScrutineeExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_scrutinee_expression(expression);
                    self.state = State::BracketCurlyLeft;
                }
                State::BracketCurlyLeft => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => self.state = State::BracketCurlyRightOrBranchPattern,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec!["{"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::BracketCurlyRightOrBranchPattern => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), self.next.take())),
                        token => {
                            let (pattern, next) =
                                MatchPatternParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_branch_pattern(pattern);
                            self.state = State::Select;
                        }
                    }
                }
                State::Select => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::EqualsGreater),
                            ..
                        } => self.state = State::BranchExpression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["=>"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::BranchExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_branch_expression(expression);
                    self.state = State::CommaOrBracketCurlyRight;
                }
                State::CommaOrBracketCurlyRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::BracketCurlyRightOrBranchPattern,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), self.next.take())),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec![",", "}"],
                                lexeme,
                                None,
                            )));
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
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::r#match::Expression as MatchExpression;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::pattern_match::variant::Variant as MatchPatternVariant;
    use crate::syntax::tree::pattern_match::Pattern as MatchPattern;

    #[test]
    fn ok_empty() {
        let input = r#"
    match test {}
"#;

        let expected = Ok((
            MatchExpression::new(
                Location::new(2, 5),
                ExpressionTree::new(
                    Location::new(2, 11),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(2, 11),
                        "test".to_owned(),
                    ))),
                    None,
                    None,
                ),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"
    match test {
        false => true,
    }
"#;

        let expected = Ok((
            MatchExpression::new(
                Location::new(2, 5),
                ExpressionTree::new(
                    Location::new(2, 11),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(2, 11),
                        "test".to_owned(),
                    ))),
                    None,
                    None,
                ),
                vec![(
                    MatchPattern::new(
                        Location::new(3, 9),
                        MatchPatternVariant::new_boolean_literal(BooleanLiteral::new(
                            Location::new(3, 9),
                            lexical::BooleanLiteral::r#false(),
                        )),
                    ),
                    ExpressionTree::new(
                        Location::new(3, 18),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(3, 18),
                                lexical::BooleanLiteral::r#true(),
                            ),
                        )),
                        None,
                        None,
                    ),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"
    match test {
        1 => 1,
        2 => 2,
        _ => 3,
    }
"#;
        let expected = Ok((
            MatchExpression::new(
                Location::new(2, 5),
                ExpressionTree::new(
                    Location::new(2, 11),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(2, 11),
                        "test".to_owned(),
                    ))),
                    None,
                    None,
                ),
                vec![
                    (
                        MatchPattern::new(
                            Location::new(3, 9),
                            MatchPatternVariant::new_integer_literal(IntegerLiteral::new(
                                Location::new(3, 9),
                                lexical::IntegerLiteral::new_decimal("1".to_owned()),
                            )),
                        ),
                        ExpressionTree::new(
                            Location::new(3, 14),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(3, 14),
                                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                            None,
                            None,
                        ),
                    ),
                    (
                        MatchPattern::new(
                            Location::new(4, 9),
                            MatchPatternVariant::new_integer_literal(IntegerLiteral::new(
                                Location::new(4, 9),
                                lexical::IntegerLiteral::new_decimal("2".to_owned()),
                            )),
                        ),
                        ExpressionTree::new(
                            Location::new(4, 14),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(4, 14),
                                    lexical::IntegerLiteral::new_decimal("2".to_owned()),
                                ),
                            )),
                            None,
                            None,
                        ),
                    ),
                    (
                        MatchPattern::new(Location::new(5, 9), MatchPatternVariant::new_wildcard()),
                        ExpressionTree::new(
                            Location::new(5, 14),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(5, 14),
                                    lexical::IntegerLiteral::new_decimal("3".to_owned()),
                                ),
                            )),
                            None,
                            None,
                        ),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_left() {
        let input = r#"match 42 * 2 )"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 14),
            vec!["{"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_select() {
        let input = r#"match 42 * 2 { value ->"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 22),
            vec!["=>"],
            Lexeme::Symbol(Symbol::MinusGreater),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_bracket_curly_right() {
        let input = r#"match 42 * 2 { value => 42 )"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 28),
            vec![",", "}"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
