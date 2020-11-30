//!
//! The type parser.
//!

pub mod array;
pub mod generics;
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
use crate::parser::identifier_path::Parser as IdentifierPathParser;
use crate::tree::r#type::builder::Builder as TypeBuilder;
use crate::tree::r#type::Type;

use self::array::Parser as ArrayParser;
use self::generics::Parser as GenericsParser;
use self::tuple::Parser as TupleParser;

///
/// The type parser.
///
#[derive(Default)]
pub struct Parser {
    /// The builder of the parsed value.
    builder: TypeBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
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
        initial: Option<Token>,
    ) -> Result<(Type, Option<Token>), ParsingError> {
        self.next = initial;

        match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
                let (expression, mut next) =
                    IdentifierPathParser::default().parse(stream.clone(), Some(token))?;
                self.builder.set_location(location);
                self.builder.set_path_expression(expression);

                match crate::parser::take_or_next(next.take(), stream.clone())? {
                    token
                    @
                    Token {
                        lexeme: Lexeme::Symbol(Symbol::Lesser),
                        ..
                    } => {
                        let (generics, next) =
                            GenericsParser::default().parse(stream, Some(token))?;
                        self.builder.set_generics(generics);
                        Ok((self.builder.finish(), next))
                    }
                    token => Ok((self.builder.finish(), Some(token))),
                }
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
            } => ArrayParser::default().parse(stream.clone(), Some(token)),
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => TupleParser::default().parse(stream.clone(), Some(token)),
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
                TypeVariant::alias(
                    ExpressionTree::new(
                        Location::test(1, 1),
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(
                                Location::test(1, 1),
                                Keyword::SelfUppercase.to_string(),
                            ),
                        )),
                    ),
                    None,
                ),
            ),
            Some(Token::new(Lexeme::Eof, Location::test(1, 5))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_path() {
        let input = r#"mega::ultra::namespace;"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::alias(
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
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 14), "namespace".to_owned()),
                            )),
                        )),
                    ),
                    None,
                ),
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
    fn ok_path_with_one_generic() {
        let input = r#"mega::ultra::namespace<u8>;"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::alias(
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
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 14), "namespace".to_owned()),
                            )),
                        )),
                    ),
                    Some(vec![Type::new(
                        Location::test(1, 24),
                        TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                    )]),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_path_with_multiple_generics() {
        let input = r#"mega::ultra::namespace<bool, u8, field>;"#;

        let expected = Ok((
            Type::new(
                Location::test(1, 1),
                TypeVariant::alias(
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
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(1, 14), "namespace".to_owned()),
                            )),
                        )),
                    ),
                    Some(vec![
                        Type::new(Location::test(1, 24), TypeVariant::boolean()),
                        Type::new(
                            Location::test(1, 30),
                            TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                        ),
                        Type::new(Location::test(1, 34), TypeVariant::field()),
                    ]),
                ),
            ),
            None,
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
