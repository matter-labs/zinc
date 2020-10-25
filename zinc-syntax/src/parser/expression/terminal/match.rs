//!
//! The match expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::expression::Parser as ExpressionParser;
use crate::parser::pattern_match::Parser as MatchPatternParser;
use crate::tree::expression::r#match::builder::Builder as MatchExpressionBuilder;
use crate::tree::expression::r#match::Expression as MatchExpression;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordMatch,
    /// The `match` has been parsed so far.
    ScrutineeExpression,
    /// The `match {expression}` has been parsed so far.
    BracketCurlyLeft,
    /// The `match {expression} {` has been parsed so far.
    BracketCurlyRightOrBranchPattern,
    /// The `match {expression} { {pattern}` has been parsed so far.
    Select,
    /// The `match {expression} { {pattern} =>` has been parsed so far.
    BranchExpression,
    /// The `match {expression} { {pattern} => {expression}` has been parsed so far.
    CommaOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordMatch
    }
}

///
/// The match expression parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed value.
    builder: MatchExpressionBuilder,
}

impl Parser {
    ///
    /// Parses a match expression.
    ///
    /// '
    /// match value {
    ///     1 => value * 5,
    ///     2 => value * 10,
    ///     another => another - 1,
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(MatchExpression, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordMatch => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Match),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::ScrutineeExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
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
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => self.state = State::BracketCurlyRightOrBranchPattern,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_one_of_or_operator(
                                    location,
                                    vec!["{"],
                                    lexeme,
                                    None,
                                ),
                            ));
                        }
                    }
                }
                State::BracketCurlyRightOrBranchPattern => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::EqualsGreater),
                            ..
                        } => self.state = State::BranchExpression,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
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
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_branch_expression(expression);
                    self.state = State::CommaOrBracketCurlyRight;
                }
                State::CommaOrBracketCurlyRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::BracketCurlyRightOrBranchPattern,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), self.next.take())),
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(
                                SyntaxError::expected_one_of_or_operator(
                                    location,
                                    vec![",", "}"],
                                    lexeme,
                                    None,
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::BooleanLiteral as LexicalBooleanLiteral;
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::r#match::Expression as MatchExpression;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::pattern_match::variant::Variant as MatchPatternVariant;
    use crate::tree::pattern_match::Pattern as MatchPattern;

    #[test]
    fn ok_empty() {
        let input = r#"
    match test {}
"#;

        let expected = Ok((
            MatchExpression::new(
                Location::test(2, 5),
                ExpressionTree::new(
                    Location::test(2, 11),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(2, 11),
                        "test".to_owned(),
                    ))),
                ),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

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
                Location::test(2, 5),
                ExpressionTree::new(
                    Location::test(2, 11),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(2, 11),
                        "test".to_owned(),
                    ))),
                ),
                vec![(
                    MatchPattern::new(
                        Location::test(3, 9),
                        MatchPatternVariant::new_boolean_literal(BooleanLiteral::new(
                            Location::test(3, 9),
                            LexicalBooleanLiteral::r#false(),
                        )),
                    ),
                    ExpressionTree::new(
                        Location::test(3, 18),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::test(3, 18),
                                LexicalBooleanLiteral::r#true(),
                            ),
                        )),
                    ),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

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
                Location::test(2, 5),
                ExpressionTree::new(
                    Location::test(2, 11),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(2, 11),
                        "test".to_owned(),
                    ))),
                ),
                vec![
                    (
                        MatchPattern::new(
                            Location::test(3, 9),
                            MatchPatternVariant::new_integer_literal(IntegerLiteral::new(
                                Location::test(3, 9),
                                LexicalIntegerLiteral::new_decimal("1".to_owned()),
                            )),
                        ),
                        ExpressionTree::new(
                            Location::test(3, 14),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(3, 14),
                                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        ),
                    ),
                    (
                        MatchPattern::new(
                            Location::test(4, 9),
                            MatchPatternVariant::new_integer_literal(IntegerLiteral::new(
                                Location::test(4, 9),
                                LexicalIntegerLiteral::new_decimal("2".to_owned()),
                            )),
                        ),
                        ExpressionTree::new(
                            Location::test(4, 14),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(4, 14),
                                    LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                ),
                            )),
                        ),
                    ),
                    (
                        MatchPattern::new(
                            Location::test(5, 9),
                            MatchPatternVariant::new_wildcard(),
                        ),
                        ExpressionTree::new(
                            Location::test(5, 14),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(5, 14),
                                    LexicalIntegerLiteral::new_decimal("3".to_owned()),
                                ),
                            )),
                        ),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_left() {
        let input = r#"match 42 * 2 )"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 14),
                vec!["{"],
                Lexeme::Symbol(Symbol::ParenthesisRight),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_select() {
        let input = r#"match 42 * 2 { value ->"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 22),
                vec!["=>"],
                Lexeme::Symbol(Symbol::MinusGreater),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_comma_or_bracket_curly_right() {
        let input = r#"match 42 * 2 { value => 42 )"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 28),
                vec![",", "}"],
                Lexeme::Symbol(Symbol::ParenthesisRight),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
