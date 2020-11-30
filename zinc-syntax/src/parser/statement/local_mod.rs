//!
//! The module-local statement parser.
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
use crate::parser::attribute::Parser as AttributeParser;
use crate::parser::statement::contract::Parser as ContractStatementParser;
use crate::parser::statement::module::Parser as ModStatementParser;
use crate::parser::statement::r#const::Parser as ConstStatementParser;
use crate::parser::statement::r#enum::Parser as EnumStatementParser;
use crate::parser::statement::r#fn::Parser as FnStatementParser;
use crate::parser::statement::r#impl::Parser as ImplStatementParser;
use crate::parser::statement::r#struct::Parser as StructStatementParser;
use crate::parser::statement::r#type::Parser as TypeStatementParser;
use crate::parser::statement::r#use::Parser as UseStatementParser;
use crate::tree::attribute::Attribute;
use crate::tree::statement::local_mod::Statement as ModuleLocalStatement;

/// The invalid statement error hint.
pub static HINT_ONLY_SOME_STATEMENTS: &str =
    "only constants, types, functions, and type implementations may be declared at the module root";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    AttributeOrNext,
    /// The attribute list has been parsed so far. Expects the optional `pub` keyword.
    KeywordPubOrNext,
    /// The attribute list has been parsed so far. Expects the optional `const` keyword.
    KeywordConstOrNext,
    /// The attribute list with optional `pub` and `const` keywords have been parsed so far.
    Statement,
}

impl Default for State {
    fn default() -> Self {
        Self::AttributeOrNext
    }
}

///
/// The module-local statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The `pub` keyword token, which is stored to get its location as the statement location.
    keyword_public: Option<Token>,
    /// The `const` keyword token, which is stored to get its location as the statement location.
    keyword_constant: Option<Token>,
    /// The statement outer attributes.
    attributes: Vec<Attribute>,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a top-level statement allowed in modules.
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ModuleLocalStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::AttributeOrNext => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Number),
                            ..
                        } => {
                            let (attribute, next) =
                                AttributeParser::default().parse(stream.clone(), Some(token))?;
                            self.attributes.push(attribute);
                            self.next = next;
                            self.state = State::AttributeOrNext;
                        }
                        token => {
                            self.next = Some(token);
                            self.state = State::KeywordPubOrNext;
                        }
                    }
                }
                State::KeywordPubOrNext => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Pub),
                            ..
                        } => self.keyword_public = Some(token),
                        token => self.next = Some(token),
                    }

                    self.state = State::KeywordConstOrNext;
                }
                State::KeywordConstOrNext => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Const),
                            ..
                        } => {
                            let look_ahead = stream.borrow_mut().look_ahead(1)?.to_owned();
                            if let Token {
                                lexeme: Lexeme::Keyword(Keyword::Fn),
                                ..
                            } = look_ahead
                            {
                                self.keyword_constant = Some(token);
                            } else {
                                return ConstStatementParser::default()
                                    .parse(stream.clone(), Some(token))
                                    .map(|(statement, next)| {
                                        (ModuleLocalStatement::Const(statement), next)
                                    });
                            }
                        }
                        token => self.next = Some(token),
                    }

                    self.state = State::Statement;
                }
                State::Statement => {
                    return match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Type),
                            ..
                        } => TypeStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModuleLocalStatement::Type(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Struct),
                            ..
                        } => StructStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| {
                                (ModuleLocalStatement::Struct(statement), next)
                            }),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Enum),
                            ..
                        } => EnumStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModuleLocalStatement::Enum(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Fn),
                            ..
                        } => {
                            let (mut builder, next) =
                                FnStatementParser::default().parse(stream.clone(), Some(token))?;

                            if let Some(token) = self.keyword_constant {
                                builder.set_location(token.location);
                                builder.set_constant();
                            }
                            if let Some(token) = self.keyword_public {
                                builder.set_location(token.location);
                                builder.set_public();
                            }

                            builder.set_attributes(self.attributes);

                            return Ok((ModuleLocalStatement::Fn(builder.finish()), next));
                        }
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mod),
                            ..
                        } => ModStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModuleLocalStatement::Mod(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Use),
                            ..
                        } => UseStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModuleLocalStatement::Use(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Impl),
                            ..
                        } => ImplStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModuleLocalStatement::Impl(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Contract),
                            ..
                        } => ContractStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| {
                                (ModuleLocalStatement::Contract(statement), next)
                            }),
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            location,
                        } => Ok((ModuleLocalStatement::Empty(location), None)),
                        Token { lexeme, location } => {
                            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![
                                    "type", "struct", "enum", "fn", "mod", "use", "impl", "const",
                                ],
                                lexeme,
                                Some(HINT_ONLY_SOME_STATEMENTS),
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
    use zinc_lexical::Location;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::attribute::element::Element as AttributeElement;
    use crate::tree::attribute::Attribute;
    use crate::tree::binding::Binding;
    use crate::tree::expression::block::Expression as BlockExpression;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::tree::pattern_binding::Pattern as BindingPattern;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;
    use crate::tree::statement::local_mod::Statement as ModuleLocalStatement;
    use crate::tree::statement::r#fn::Statement as FnStatement;

    #[test]
    fn ok_fn_public() {
        let input = r#"pub fn f(a: field) -> field {}"#;

        let expected = Ok((
            ModuleLocalStatement::Fn(FnStatement::new(
                Location::test(1, 1),
                true,
                false,
                Identifier::new(Location::test(1, 8), "f".to_owned()),
                vec![Binding::new(
                    Location::test(1, 10),
                    BindingPattern::new(
                        Location::test(1, 10),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 10), "a".to_owned()),
                            false,
                        ),
                    ),
                    Some(Type::new(Location::test(1, 13), TypeVariant::field())),
                )],
                Some(Type::new(Location::test(1, 23), TypeVariant::field())),
                BlockExpression::new(Location::test(1, 29), vec![], None),
                vec![],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_fn_constant() {
        let input = r#"const fn f(a: field) -> field {}"#;

        let expected = Ok((
            ModuleLocalStatement::Fn(FnStatement::new(
                Location::test(1, 1),
                false,
                true,
                Identifier::new(Location::test(1, 10), "f".to_owned()),
                vec![Binding::new(
                    Location::test(1, 12),
                    BindingPattern::new(
                        Location::test(1, 12),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 12), "a".to_owned()),
                            false,
                        ),
                    ),
                    Some(Type::new(Location::test(1, 15), TypeVariant::field())),
                )],
                Some(Type::new(Location::test(1, 25), TypeVariant::field())),
                BlockExpression::new(Location::test(1, 31), vec![], None),
                vec![],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_fn_public_constant() {
        let input = r#"pub const fn f(a: field) -> field {}"#;

        let expected = Ok((
            ModuleLocalStatement::Fn(FnStatement::new(
                Location::test(1, 1),
                true,
                true,
                Identifier::new(Location::test(1, 14), "f".to_owned()),
                vec![Binding::new(
                    Location::test(1, 16),
                    BindingPattern::new(
                        Location::test(1, 16),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 16), "a".to_owned()),
                            false,
                        ),
                    ),
                    Some(Type::new(Location::test(1, 19), TypeVariant::field())),
                )],
                Some(Type::new(Location::test(1, 29), TypeVariant::field())),
                BlockExpression::new(Location::test(1, 35), vec![], None),
                vec![],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_fn_single_attribute() {
        let input = r#"
#[test]
fn test() {}
"#;

        let expected = Ok((
            ModuleLocalStatement::Fn(FnStatement::new(
                Location::test(3, 1),
                false,
                false,
                Identifier::new(Location::test(3, 4), "test".to_owned()),
                vec![],
                None,
                BlockExpression::new(Location::test(3, 11), vec![], None),
                vec![Attribute::new(
                    Location::test(2, 1),
                    false,
                    vec![AttributeElement::new(
                        Location::test(2, 3),
                        ExpressionTree::new(
                            Location::test(2, 3),
                            ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                Identifier::new(Location::test(2, 3), "test".to_owned()),
                            )),
                        ),
                        None,
                    )],
                )],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_fn_multiple_attributes() {
        let input = r#"
#[test]
#[should_panic]
#[ignore]
fn test() {}
"#;

        let expected = Ok((
            ModuleLocalStatement::Fn(FnStatement::new(
                Location::test(5, 1),
                false,
                false,
                Identifier::new(Location::test(5, 4), "test".to_owned()),
                vec![],
                None,
                BlockExpression::new(Location::test(5, 11), vec![], None),
                vec![
                    Attribute::new(
                        Location::test(2, 1),
                        false,
                        vec![AttributeElement::new(
                            Location::test(2, 3),
                            ExpressionTree::new(
                                Location::test(2, 3),
                                ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                    Identifier::new(Location::test(2, 3), "test".to_owned()),
                                )),
                            ),
                            None,
                        )],
                    ),
                    Attribute::new(
                        Location::test(3, 1),
                        false,
                        vec![AttributeElement::new(
                            Location::test(3, 3),
                            ExpressionTree::new(
                                Location::test(3, 3),
                                ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                    Identifier::new(
                                        Location::test(3, 3),
                                        "should_panic".to_owned(),
                                    ),
                                )),
                            ),
                            None,
                        )],
                    ),
                    Attribute::new(
                        Location::test(4, 1),
                        false,
                        vec![AttributeElement::new(
                            Location::test(4, 3),
                            ExpressionTree::new(
                                Location::test(4, 3),
                                ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                                    Identifier::new(Location::test(4, 3), "ignore".to_owned()),
                                )),
                            ),
                            None,
                        )],
                    ),
                ],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
