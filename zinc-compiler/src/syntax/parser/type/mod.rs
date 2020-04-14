//!
//! The type parser.
//!

pub mod array;
pub mod path;
pub mod tuple;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::tree::r#type::builder::Builder as TypeBuilder;
use crate::syntax::tree::r#type::Type;

use self::array::Parser as ArrayParser;
use self::path::Parser as PathParser;
use self::tuple::Parser as TupleParser;

#[derive(Default)]
pub struct Parser {
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
    ) -> Result<(Type, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
            token
            @
            Token {
                lexeme: Lexeme::Identifier(_),
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
                _ => Err(Error::Syntax(SyntaxError::expected_type(
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
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::expected_type(
                location, lexeme, None,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::keyword::Keyword;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::literal::Literal as LexicalLiteral;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;

    #[test]
    fn ok_bool() {
        let input = r#"bool"#;

        let expected = Ok((Type::new(Location::new(1, 1), TypeVariant::boolean()), None));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_integer() {
        let input = r#"u232"#;

        let expected = Ok((
            Type::new(Location::new(1, 1), TypeVariant::integer_unsigned(232)),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_field() {
        let input = r#"field"#;

        let expected = Ok((Type::new(Location::new(1, 1), TypeVariant::field()), None));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_self_alias() {
        let input = r#"Self"#;

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::alias(ExpressionTree::new(
                    Location::new(1, 1),
                    ExpressionTreeNode::operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        Keyword::SelfUppercase.to_string(),
                    ))),
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 5))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type_keyword() {
        let input = r#"while"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_type(
            Location::new(1, 1),
            Lexeme::Keyword(Keyword::While),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"42"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_type(
            Location::new(1, 1),
            Lexeme::Literal(LexicalLiteral::Integer(LexicalIntegerLiteral::new_decimal(
                "42".to_owned(),
            ))),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
