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
use crate::syntax::parser::statement::r#const::Parser as ConstStatementParser;
use crate::syntax::parser::statement::r#fn::Parser as FnStatementParser;
use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;

pub static HINT_ONLY_SOME_STATEMENTS: &str =
    "only constants and functions may be declared within a type implementation";

#[derive(Debug, Clone, Copy)]
pub enum State {
    PubOrNext,
    Statement,
}

impl Default for State {
    fn default() -> Self {
        Self::PubOrNext
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    is_public: bool,
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
                State::PubOrNext => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Pub),
                            ..
                        } => self.is_public = true,
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
                            lexeme: Lexeme::Keyword(Keyword::Const),
                            ..
                        } => ConstStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| {
                                (ImplementationLocalStatement::Const(statement), next)
                            }),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Fn),
                            ..
                        } => {
                            let (mut builder, next) =
                                FnStatementParser::default().parse(stream.clone(), Some(token))?;

                            if self.is_public {
                                builder.set_is_public();
                            }

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
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::location::Location;
    use crate::syntax::tree::expression::block::Expression as BlockExpression;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::local_impl::Statement as ImplLocalStatement;
    use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

    #[test]
    fn ok_fn_public() {
        let input = r#"pub fn f(a: field) -> field {}"#;

        let expected = Ok((
            ImplLocalStatement::Fn(FnStatement::new(
                Location::new(1, 5),
                true,
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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
