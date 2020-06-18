//!
//! The implementation-local statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::attribute::Parser as AttributeParser;
use crate::syntax::parser::statement::r#const::Parser as ConstStatementParser;
use crate::syntax::parser::statement::r#fn::Parser as FnStatementParser;
use crate::syntax::tree::attribute::Attribute;
use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;

pub static HINT_ONLY_SOME_STATEMENTS: &str =
    "only constants and functions may be declared within a namespace";

#[derive(Debug, Clone, Copy)]
pub enum State {
    AttributeOrNext,
    KeywordPubOrNext,
    KeywordConstOrNext,
    Statement,
}

impl Default for State {
    fn default() -> Self {
        Self::AttributeOrNext
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    keyword_public: Option<Token>,
    keyword_constant: Option<Token>,
    attributes: Vec<Attribute>,
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a statement allowed in type implementations.
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ImplementationLocalStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::AttributeOrNext => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
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
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Pub),
                            ..
                        } => self.keyword_public = Some(token),
                        token => self.next = Some(token),
                    }

                    self.state = State::KeywordConstOrNext;
                    continue;
                }
                State::KeywordConstOrNext => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
                                        (ImplementationLocalStatement::Const(statement), next)
                                    });
                            }
                        }
                        token => self.next = Some(token),
                    }

                    self.state = State::Statement;
                    continue;
                }
                State::Statement => {
                    return match crate::syntax::parser::take_or_next(
                        self.next.take(),
                        stream.clone(),
                    )? {
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
                                builder.set_is_constant();
                            }
                            if let Some(token) = self.keyword_public {
                                builder.set_location(token.location);
                                builder.set_is_public();
                            }

                            builder.set_attributes(self.attributes);

                            Ok((ImplementationLocalStatement::Fn(builder.finish()), next))
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            location,
                        } => Ok((ImplementationLocalStatement::Empty(location), None)),
                        Token { lexeme, location } => {
                            Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["const", "fn"],
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
    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::location::Location;
    use crate::syntax::tree::attribute::Attribute;
    use crate::syntax::tree::expression::block::Expression as BlockExpression;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;
    use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

    #[test]
    fn ok_fn_public() {
        let input = r#"pub fn f(a: field) -> field {}"#;

        let expected = Ok((
            ImplementationLocalStatement::Fn(FnStatement::new(
                Location::new(1, 1),
                true,
                false,
                Identifier::new(Location::new(1, 8), "f".to_owned()),
                vec![BindingPattern::new(
                    Location::new(1, 10),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::new(1, 10), "a".to_owned()),
                        false,
                    ),
                    Type::new(Location::new(1, 13), TypeVariant::field()),
                )],
                Some(Type::new(Location::new(1, 23), TypeVariant::field())),
                BlockExpression::new(Location::new(1, 29), vec![], None),
                vec![],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_fn_constant() {
        let input = r#"const fn f(a: field) -> field {}"#;

        let expected = Ok((
            ImplementationLocalStatement::Fn(FnStatement::new(
                Location::new(1, 1),
                false,
                true,
                Identifier::new(Location::new(1, 10), "f".to_owned()),
                vec![BindingPattern::new(
                    Location::new(1, 12),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::new(1, 12), "a".to_owned()),
                        false,
                    ),
                    Type::new(Location::new(1, 15), TypeVariant::field()),
                )],
                Some(Type::new(Location::new(1, 25), TypeVariant::field())),
                BlockExpression::new(Location::new(1, 31), vec![], None),
                vec![],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_fn_public_constant() {
        let input = r#"pub const fn f(a: field) -> field {}"#;

        let expected = Ok((
            ImplementationLocalStatement::Fn(FnStatement::new(
                Location::new(1, 1),
                true,
                true,
                Identifier::new(Location::new(1, 14), "f".to_owned()),
                vec![BindingPattern::new(
                    Location::new(1, 16),
                    BindingPatternVariant::new_binding(
                        Identifier::new(Location::new(1, 16), "a".to_owned()),
                        false,
                    ),
                    Type::new(Location::new(1, 19), TypeVariant::field()),
                )],
                Some(Type::new(Location::new(1, 29), TypeVariant::field())),
                BlockExpression::new(Location::new(1, 35), vec![], None),
                vec![],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_fn_single_attribute() {
        let input = r#"
#[test]
fn test() {}
"#;

        let expected = Ok((
            ImplementationLocalStatement::Fn(FnStatement::new(
                Location::new(3, 1),
                false,
                false,
                Identifier::new(Location::new(3, 4), "test".to_owned()),
                vec![],
                None,
                BlockExpression::new(Location::new(3, 11), vec![], None),
                vec![Attribute::new(
                    Location::new(2, 1),
                    false,
                    Identifier::new(Location::new(2, 3), "test".to_owned()),
                )],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

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
            ImplementationLocalStatement::Fn(FnStatement::new(
                Location::new(5, 1),
                false,
                false,
                Identifier::new(Location::new(5, 4), "test".to_owned()),
                vec![],
                None,
                BlockExpression::new(Location::new(5, 11), vec![], None),
                vec![
                    Attribute::new(
                        Location::new(2, 1),
                        false,
                        Identifier::new(Location::new(2, 3), "test".to_owned()),
                    ),
                    Attribute::new(
                        Location::new(3, 1),
                        false,
                        Identifier::new(Location::new(3, 3), "should_panic".to_owned()),
                    ),
                    Attribute::new(
                        Location::new(4, 1),
                        false,
                        Identifier::new(Location::new(4, 3), "ignore".to_owned()),
                    ),
                ],
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
