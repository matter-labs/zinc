//!
//! The match pattern parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Literal as LexicalLiteral;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::expression::terminal::Parser as TerminalOperandParser;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::identifier::Identifier;
use crate::tree::literal::boolean::Literal as BooleanLiteral;
use crate::tree::literal::integer::Literal as IntegerLiteral;
use crate::tree::pattern_match::builder::Builder as MatchPatternBuilder;
use crate::tree::pattern_match::Pattern as MatchPattern;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Start,
    /// The first path operand has been parsed so far.
    PathOperatorOrEnd,
    /// The first path operand and a `::` path operator have been parsed so far.
    PathOperand,
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

///
/// The match pattern parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: MatchPatternBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a match pattern.
    ///
    /// 'true'
    /// '42'
    /// 'variable'
    /// 'Path::To::Item'
    /// '_'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(MatchPattern, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::Start => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Literal(LexicalLiteral::Boolean(boolean)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_boolean_literal(BooleanLiteral::new(location, boolean));
                            return Ok((self.builder.finish(), None));
                        }
                        Token {
                            lexeme: Lexeme::Literal(LexicalLiteral::Integer(integer)),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_integer_literal(IntegerLiteral::new(location, integer));
                            return Ok((self.builder.finish(), None));
                        }
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_binding(Identifier::new(location, identifier.inner));
                            self.state = State::PathOperatorOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::Crate),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::Super),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::SelfLowercase),
                            location,
                        }
                        | Token {
                            lexeme: Lexeme::Keyword(keyword @ Keyword::SelfUppercase),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder
                                .set_binding(Identifier::new(location, keyword.to_string()));
                            self.state = State::PathOperatorOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Underscore),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.set_wildcard();
                            return Ok((self.builder.finish(), None));
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_match_pattern(
                                location, lexeme,
                            )));
                        }
                    }
                }
                State::PathOperatorOrEnd => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleColon),
                            location,
                        } => {
                            self.builder
                                .push_path_operator(ExpressionOperator::Path, location);
                            self.state = State::PathOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::PathOperand => {
                    let (expression, next) =
                        TerminalOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.push_path_element(expression);
                    self.state = State::PathOperatorOrEnd;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::BooleanLiteral as LexicalBooleanLiteral;
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Keyword;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::pattern_match::variant::Variant as MatchPatternVariant;
    use crate::tree::pattern_match::Pattern as MatchPattern;

    #[test]
    fn ok_literal_boolean() {
        let input = r#"true"#;

        let expected = Ok((
            MatchPattern::new(
                Location::test(1, 1),
                MatchPatternVariant::BooleanLiteral(BooleanLiteral::new(
                    Location::test(1, 1),
                    LexicalBooleanLiteral::r#true(),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_literal_integer() {
        let input = r#"42"#;

        let expected = Ok((
            MatchPattern::new(
                Location::test(1, 1),
                MatchPatternVariant::IntegerLiteral(IntegerLiteral::new(
                    Location::test(1, 1),
                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding() {
        let input = r#"value"#;

        let expected = Ok((
            MatchPattern::new(
                Location::test(1, 1),
                MatchPatternVariant::Binding(Identifier::new(
                    Location::test(1, 1),
                    "value".to_owned(),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 6))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_path() {
        let input = r#"data::Inner::Value"#;

        let expected = Ok((
            MatchPattern::new(
                Location::test(1, 1),
                MatchPatternVariant::Path(ExpressionTree::new_with_leaves(
                    Location::test(1, 12),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new_with_leaves(
                        Location::test(1, 5),
                        ExpressionTreeNode::operator(ExpressionOperator::Path),
                        Some(ExpressionTree::new(
                            Location::test(1, 1),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 1), "data".to_owned()),
                            )),
                        )),
                        Some(ExpressionTree::new(
                            Location::test(1, 7),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 7), "Inner".to_owned()),
                            )),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 14),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::test(1, 14), "Value".to_owned()),
                        )),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 19))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_path_alias() {
        let input = r#"Self::Value"#;

        let expected = Ok((
            MatchPattern::new(
                Location::test(1, 1),
                MatchPatternVariant::Path(ExpressionTree::new_with_leaves(
                    Location::test(1, 5),
                    ExpressionTreeNode::operator(ExpressionOperator::Path),
                    Some(ExpressionTree::new(
                        Location::test(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(
                                Location::test(1, 1),
                                Keyword::SelfUppercase.to_string(),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 7),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(Location::test(1, 7), "Value".to_owned()),
                        )),
                    )),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 12))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_wildcard() {
        let input = r#"_"#;

        let expected = Ok((
            MatchPattern::new(Location::test(1, 1), MatchPatternVariant::Wildcard),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
