//!
//! The type parser.
//!

pub mod array;
pub mod path;
pub mod tuple;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::tree::r#type::builder::Builder as TypeBuilder;
use crate::tree::r#type::Type;

use self::array::Parser as ArrayParser;
use self::path::Parser as PathParser;
use self::tuple::Parser as TupleParser;

///
/// The type parser.
///
#[derive(Default)]
pub struct Parser {
    /// The builder of the parsed value.
    builder: TypeBuilder,
}

impl Parser {
    ///
    /// Parses a type literal.
    ///
    /// 'bool'
    /// '[u8; 16]'
    /// '(u8, field, bool)'
    /// 'Path::To::Type`
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Type, Option<Token>), ParsingError> {
        match crate::parser::take_or_next(initial.take(), stream.clone())? {
            token
            @
            Token {
                lexeme: Lexeme::Identifier(_),
                ..
            }
            | token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Crate),
                ..
            }
            | token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Super),
                ..
            }
            | token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::SelfLowercase),
                ..
            }
            | token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::SelfUppercase),
                ..
            } => {
                let location = token.location;
                let (expression, next) = PathParser::default().parse(stream, Some(token))?;
                self.builder.set_location(location);
                self.builder.set_path_expression(expression);
                Ok((self.builder.finish(), next))
            }
            Token {
                lexeme: Lexeme::Keyword(keyword),
                location,
            } => match keyword {
                keyword @ Keyword::Bool
                | keyword @ Keyword::IntegerSigned { .. }
                | keyword @ Keyword::IntegerUnsigned { .. }
                | keyword @ Keyword::Field => {
                    self.builder.set_location(location);
                    self.builder.set_keyword(keyword);
                    Ok((self.builder.finish(), None))
                }
                _ => Err(ParsingError::Syntax(SyntaxError::expected_type(
                    location,
                    Lexeme::Keyword(keyword),
                    None,
                ))),
            },
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                ..
            } => ArrayParser::default().parse(stream, Some(token)),
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => TupleParser::default().parse(stream, Some(token)),
            Token { lexeme, location } => Err(ParsingError::Syntax(SyntaxError::expected_type(
                location, lexeme, None,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Keyword;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Literal as LexicalLiteral;
    use zinc_lexical::Location;
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok_bool() {
        let input = r#"bool"#;

        let expected = Ok((
            Type::new(Location::test(1, 1), TypeVariant::boolean()),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_integer() {
        let input = r#"u232"#;

        let expected = Ok((
            Type::new(Location::test(1, 1), TypeVariant::integer_unsigned(232)),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_field() {
        let input = r#"field"#;

        let expected = Ok((Type::new(Location::test(1, 1), TypeVariant::field()), None));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_self_alias() {
        let input = r#"Self"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::alias(ExpressionTree::new(
                    Location::test(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::test(1, 1),
                        Keyword::SelfUppercase.to_string(),
                    ))),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 5))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type_keyword() {
        let input = r#"while"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_type(
            Location::test(1, 1),
            Lexeme::Keyword(Keyword::While),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"42"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_type(
            Location::test(1, 1),
            Lexeme::Literal(LexicalLiteral::Integer(LexicalIntegerLiteral::new_decimal(
                "42".to_owned(),
            ))),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
