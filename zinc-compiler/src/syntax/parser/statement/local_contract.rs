//!
//! The contract-local statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::parser::statement::field::Parser as FieldStatementParser;
use crate::syntax::parser::statement::r#const::Parser as ConstStatementParser;
use crate::syntax::parser::statement::r#fn::Parser as FnStatementParser;
use crate::syntax::tree::statement::local_contract::Statement as ContractLocalStatement;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordPubOrNext,
    KeywordConstOrNext,
    Statement,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordPubOrNext
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    keyword_public: Option<Token>,
    keyword_constant: Option<Token>,
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
    ) -> Result<(ContractLocalStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordPubOrNext => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
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
                            self.keyword_constant = Some(token.clone());

                            let look_ahead_1 = stream.borrow_mut().look_ahead(1)?.to_owned();

                            if let Token {
                                lexeme: Lexeme::Keyword(Keyword::Fn),
                                ..
                            } = look_ahead_1
                            {
                                self.state = State::Statement;
                                continue;
                            }

                            return ConstStatementParser::default()
                                .parse(stream.clone(), Some(token))
                                .map(|(statement, next)| {
                                    (ContractLocalStatement::Const(statement), next)
                                });
                        }
                        token => {
                            self.next = Some(token);
                            self.state = State::Statement;
                            continue;
                        }
                    }
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

                            Ok((ContractLocalStatement::Fn(builder.finish()), next))
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            location,
                        } => Ok((ContractLocalStatement::Empty(location), None)),
                        token => {
                            let (mut builder, next) = FieldStatementParser::default()
                                .parse(stream.clone(), Some(token))?;

                            if let Some(token) = self.keyword_public {
                                builder.set_location(token.location);
                                builder.set_is_public();
                            }

                            Ok((ContractLocalStatement::Field(builder.finish()), next))
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
    use crate::syntax::tree::expression::block::Expression as BlockExpression;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::local_contract::Statement as ContractLocalStatement;
    use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

    #[test]
    fn ok_fn_public() {
        let input = r#"pub fn f(a: field) -> field {}"#;

        let expected = Ok((
            ContractLocalStatement::Fn(FnStatement::new(
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
            ContractLocalStatement::Fn(FnStatement::new(
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
            ContractLocalStatement::Fn(FnStatement::new(
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
            )),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
