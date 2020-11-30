//!
//! The attribute parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::attribute::list::Parser as AttributeListParser;
use crate::parser::identifier_path::Parser as IdentifierPathParser;
use crate::tree::attribute::element::builder::Builder as AttributeElementBuilder;
use crate::tree::attribute::element::Element as AttributeElement;
use crate::tree::literal::boolean::Literal as BooleanLiteral;
use crate::tree::literal::integer::Literal as IntegerLiteral;
use crate::tree::literal::string::Literal as StringLiteral;
use crate::tree::literal::Literal;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Path,
    /// The `#[{identifier}` has been parsed so far.
    VariantOrBracketSquareRight,
    /// The `#[{identifier} =` has been parsed so far.
    Value,
    /// The `#[{identifier}(` has been parsed so far.
    Nested,
    /// The `#[{identifier}({nested}` has been parsed so far.
    ParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        Self::Path
    }
}

///
/// The attribute parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: AttributeElementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an attribute.
    ///
    /// 'test(default)'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(AttributeElement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::Path => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Identifier(_),
                            ..
                        } => {
                            self.builder.set_location(token.location);
                            let (path, next) = IdentifierPathParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.builder.set_path(path);
                            self.next = next;
                            self.state = State::VariantOrBracketSquareRight;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                                location, lexeme, None,
                            )));
                        }
                    }
                }
                State::VariantOrBracketSquareRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            ..
                        } => {
                            self.state = State::Value;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        } => {
                            self.state = State::Nested;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::Nested => {
                    let (nested, next) =
                        AttributeListParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_nested(nested);
                    self.next = next;
                    self.state = State::ParenthesisRight;
                }
                State::Value => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Literal(zinc_lexical::Literal::Boolean(inner)),
                            location,
                        } => {
                            self.builder
                                .set_value(Literal::Boolean(BooleanLiteral::new(location, inner)));
                        }
                        Token {
                            lexeme: Lexeme::Literal(zinc_lexical::Literal::Integer(inner)),
                            location,
                        } => {
                            self.builder
                                .set_value(Literal::Integer(IntegerLiteral::new(location, inner)));
                        }
                        Token {
                            lexeme: Lexeme::Literal(zinc_lexical::Literal::String(inner)),
                            location,
                        } => {
                            self.builder
                                .set_value(Literal::String(StringLiteral::new(location, inner)));
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_literal(
                                location, lexeme,
                            )));
                        }
                    }

                    return Ok((self.builder.finish(), self.next.take()));
                }
                State::ParenthesisRight => {
                    return match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => Ok((self.builder.finish(), self.next.take())),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of(location, vec![")"], lexeme, None),
                        )),
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
    use crate::tree::attribute::element::variant::Variant as AttributeElementVariant;
    use crate::tree::attribute::element::Element as AttributeElement;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::literal::string::Literal as StringLiteral;
    use crate::tree::literal::Literal;

    #[test]
    fn ok_simple() {
        let input = r#"test"#;

        let expected = Ok((
            AttributeElement::new(
                Location::test(1, 1),
                ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(1, 1),
                        "test".to_owned(),
                    ))),
                ),
                None,
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 5))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_variant_value_boolean() {
        let input = r#"test = true"#;

        let expected = Ok((
            AttributeElement::new(
                Location::test(1, 1),
                ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(1, 1),
                        "test".to_owned(),
                    ))),
                ),
                Some(AttributeElementVariant::Value(Literal::Boolean(
                    BooleanLiteral::new(Location::test(1, 8), zinc_lexical::BooleanLiteral::True),
                ))),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_variant_value_integer() {
        let input = r#"test = 42"#;

        let expected = Ok((
            AttributeElement::new(
                Location::test(1, 1),
                ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(1, 1),
                        "test".to_owned(),
                    ))),
                ),
                Some(AttributeElementVariant::Value(Literal::Integer(
                    IntegerLiteral::new(
                        Location::test(1, 8),
                        zinc_lexical::IntegerLiteral::new_decimal("42".to_owned()),
                    ),
                ))),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_variant_value_string() {
        let input = r#"test = "default""#;

        let expected = Ok((
            AttributeElement::new(
                Location::test(1, 1),
                ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(1, 1),
                        "test".to_owned(),
                    ))),
                ),
                Some(AttributeElementVariant::Value(Literal::String(
                    StringLiteral::new(
                        Location::test(1, 8),
                        zinc_lexical::StringLiteral::new("default".to_owned()),
                    ),
                ))),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_parenthesis_right() {
        let input = r#"test(default]"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 13),
            vec![")"],
            Lexeme::Symbol(Symbol::BracketSquareRight),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
