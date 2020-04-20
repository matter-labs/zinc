//!
//! The module-local statement parser.
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
use crate::syntax::parser::statement::contract::Parser as ContractStatementParser;
use crate::syntax::parser::statement::module::Parser as ModStatementParser;
use crate::syntax::parser::statement::r#const::Parser as ConstStatementParser;
use crate::syntax::parser::statement::r#enum::Parser as EnumStatementParser;
use crate::syntax::parser::statement::r#fn::Parser as FnStatementParser;
use crate::syntax::parser::statement::r#impl::Parser as ImplStatementParser;
use crate::syntax::parser::statement::r#struct::Parser as StructStatementParser;
use crate::syntax::parser::statement::r#type::Parser as TypeStatementParser;
use crate::syntax::parser::statement::r#use::Parser as UseStatementParser;
use crate::syntax::tree::statement::local_mod::Statement as ModLocalStatement;

static HINT_ONLY_SOME_STATEMENTS: &str =
    "only constants, types, functions, and type implementations may be declared at the module root";

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
    /// Parses a top-level statement allowed in modules.
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ModLocalStatement, Option<Token>), Error> {
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
                            .map(|(statement, next)| (ModLocalStatement::Const(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Type),
                            ..
                        } => TypeStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModLocalStatement::Type(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Struct),
                            ..
                        } => StructStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModLocalStatement::Struct(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Enum),
                            ..
                        } => EnumStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModLocalStatement::Enum(statement), next)),
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

                            return Ok((ModLocalStatement::Fn(builder.finish()), next));
                        }
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mod),
                            ..
                        } => ModStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModLocalStatement::Mod(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Use),
                            ..
                        } => UseStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModLocalStatement::Use(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Impl),
                            ..
                        } => ImplStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| (ModLocalStatement::Impl(statement), next)),
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Contract),
                            ..
                        } => ContractStatementParser::default()
                            .parse(stream.clone(), Some(token))
                            .map(|(statement, next)| {
                                (ModLocalStatement::Contract(statement), next)
                            }),
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            location,
                        } => Ok((ModLocalStatement::Empty(location), None)),
                        Token { lexeme, location } => {
                            Err(Error::Syntax(SyntaxError::expected_one_of(
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
    use crate::syntax::tree::statement::local_mod::Statement as ModLocalStatement;
    use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

    #[test]
    fn ok_fn_public() {
        let input = r#"pub fn f(a: field) -> field {}"#;

        let expected = Ok((
            ModLocalStatement::Fn(FnStatement::new(
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
