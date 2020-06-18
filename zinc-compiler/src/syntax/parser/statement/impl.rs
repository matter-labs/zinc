//!
//! The impl statement parser.
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
use crate::syntax::parser::statement::local_impl::Parser as ImplementationLocalStatementParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#impl::builder::Builder as ImplStatementBuilder;
use crate::syntax::tree::statement::r#impl::Statement as ImplStatement;

pub static HINT_EXPECTED_IDENTIFIER: &str =
    "type implementation must have an identifier, e.g. `impl Data { ... }`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordImpl,
    Identifier,
    BracketCurlyLeft,
    StatementOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordImpl
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ImplStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an 'impl' statement.
    ///
    /// '
    /// impl Data {
    ///     fn sum(a: u8, b: u8) -> u8 {
    ///         a + b
    ///     }
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ImplStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordImpl => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Impl),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["impl"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::BracketCurlyLeft;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::BracketCurlyLeft => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            self.state = State::StatementOrBracketCurlyRight;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::StatementOrBracketCurlyRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), None)),
                        token => {
                            let (statement, next) = ImplementationLocalStatementParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_statement(statement);
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
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::block::Expression as BlockExpression;
    use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;
    use crate::syntax::tree::statement::r#const::Statement as ConstStatement;
    use crate::syntax::tree::statement::r#fn::Statement as FnStatement;
    use crate::syntax::tree::statement::r#impl::Statement as ImplStatement;

    #[test]
    fn ok_empty() {
        let input = r#"
    impl Test {}
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_constant() {
        let input = r#"
    impl Test {
        const VALUE: u64 = 42;
    }
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![ImplementationLocalStatement::Const(ConstStatement::new(
                    Location::new(3, 9),
                    Identifier::new(Location::new(3, 15), "VALUE".to_owned()),
                    Type::new(Location::new(3, 22), TypeVariant::integer_unsigned(64)),
                    ExpressionTree::new(
                        Location::new(3, 28),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(3, 28),
                                LexicalIntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    ),
                ))],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple_constants() {
        let input = r#"
    impl Test {
        const VALUE: u64 = 42;

        const ANOTHER: u64 = 42;

        const YET_ANOTHER: u64 = 42;
    }
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(3, 9),
                        Identifier::new(Location::new(3, 15), "VALUE".to_owned()),
                        Type::new(Location::new(3, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(3, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(3, 28),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(5, 9),
                        Identifier::new(Location::new(5, 15), "ANOTHER".to_owned()),
                        Type::new(Location::new(5, 24), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(5, 30),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(5, 30),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(7, 9),
                        Identifier::new(Location::new(7, 15), "YET_ANOTHER".to_owned()),
                        Type::new(Location::new(7, 28), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(7, 34),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(7, 34),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_function() {
        let input = r#"
    impl Test {
        fn f(a: field) -> field {}
    }
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![ImplementationLocalStatement::Fn(FnStatement::new(
                    Location::new(3, 9),
                    false,
                    false,
                    Identifier::new(Location::new(3, 12), "f".to_owned()),
                    vec![BindingPattern::new(
                        Location::new(3, 14),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::new(3, 14), "a".to_owned()),
                            false,
                        ),
                        Type::new(Location::new(3, 17), TypeVariant::field()),
                    )],
                    Some(Type::new(Location::new(3, 27), TypeVariant::field())),
                    BlockExpression::new(Location::new(3, 33), vec![], None),
                    vec![],
                ))],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple_functions() {
        let input = r#"
    impl Test {
        fn f1(a: field) -> field {}

        fn f2(a: field) -> field {}

        fn f3(a: field) -> field {}
    }
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::new(3, 9),
                        false,
                        false,
                        Identifier::new(Location::new(3, 12), "f1".to_owned()),
                        vec![BindingPattern::new(
                            Location::new(3, 15),
                            BindingPatternVariant::new_binding(
                                Identifier::new(Location::new(3, 15), "a".to_owned()),
                                false,
                            ),
                            Type::new(Location::new(3, 18), TypeVariant::field()),
                        )],
                        Some(Type::new(Location::new(3, 28), TypeVariant::field())),
                        BlockExpression::new(Location::new(3, 34), vec![], None),
                        vec![],
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::new(5, 9),
                        false,
                        false,
                        Identifier::new(Location::new(5, 12), "f2".to_owned()),
                        vec![BindingPattern::new(
                            Location::new(5, 15),
                            BindingPatternVariant::new_binding(
                                Identifier::new(Location::new(5, 15), "a".to_owned()),
                                false,
                            ),
                            Type::new(Location::new(5, 18), TypeVariant::field()),
                        )],
                        Some(Type::new(Location::new(5, 28), TypeVariant::field())),
                        BlockExpression::new(Location::new(5, 34), vec![], None),
                        vec![],
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::new(7, 9),
                        false,
                        false,
                        Identifier::new(Location::new(7, 12), "f3".to_owned()),
                        vec![BindingPattern::new(
                            Location::new(7, 15),
                            BindingPatternVariant::new_binding(
                                Identifier::new(Location::new(7, 15), "a".to_owned()),
                                false,
                            ),
                            Type::new(Location::new(7, 18), TypeVariant::field()),
                        )],
                        Some(Type::new(Location::new(7, 28), TypeVariant::field())),
                        BlockExpression::new(Location::new(7, 34), vec![], None),
                        vec![],
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_constant_single_function() {
        let input = r#"
    impl Test {
        const VALUE: u64 = 42;

        fn f(a: field) -> field {}
    }
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(3, 9),
                        Identifier::new(Location::new(3, 15), "VALUE".to_owned()),
                        Type::new(Location::new(3, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(3, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(3, 28),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::new(5, 9),
                        false,
                        false,
                        Identifier::new(Location::new(5, 12), "f".to_owned()),
                        vec![BindingPattern::new(
                            Location::new(5, 14),
                            BindingPatternVariant::new_binding(
                                Identifier::new(Location::new(5, 14), "a".to_owned()),
                                false,
                            ),
                            Type::new(Location::new(5, 17), TypeVariant::field()),
                        )],
                        Some(Type::new(Location::new(5, 27), TypeVariant::field())),
                        BlockExpression::new(Location::new(5, 33), vec![], None),
                        vec![],
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple_constants_multiple_functions() {
        let input = r#"
    impl Test {
        const VALUE: u64 = 42;

        const ANOTHER: u64 = 42;

        const YET_ANOTHER: u64 = 42;

        fn f1(a: field) -> field {}

        fn f2(a: field) -> field {}

        fn f3(a: field) -> field {}
    }
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(3, 9),
                        Identifier::new(Location::new(3, 15), "VALUE".to_owned()),
                        Type::new(Location::new(3, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(3, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(3, 28),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(5, 9),
                        Identifier::new(Location::new(5, 15), "ANOTHER".to_owned()),
                        Type::new(Location::new(5, 24), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(5, 30),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(5, 30),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::new(7, 9),
                        Identifier::new(Location::new(7, 15), "YET_ANOTHER".to_owned()),
                        Type::new(Location::new(7, 28), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::new(7, 34),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(7, 34),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::new(9, 9),
                        false,
                        false,
                        Identifier::new(Location::new(9, 12), "f1".to_owned()),
                        vec![BindingPattern::new(
                            Location::new(9, 15),
                            BindingPatternVariant::new_binding(
                                Identifier::new(Location::new(9, 15), "a".to_owned()),
                                false,
                            ),
                            Type::new(Location::new(9, 18), TypeVariant::field()),
                        )],
                        Some(Type::new(Location::new(9, 28), TypeVariant::field())),
                        BlockExpression::new(Location::new(9, 34), vec![], None),
                        vec![],
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::new(11, 9),
                        false,
                        false,
                        Identifier::new(Location::new(11, 12), "f2".to_owned()),
                        vec![BindingPattern::new(
                            Location::new(11, 15),
                            BindingPatternVariant::new_binding(
                                Identifier::new(Location::new(11, 15), "a".to_owned()),
                                false,
                            ),
                            Type::new(Location::new(11, 18), TypeVariant::field()),
                        )],
                        Some(Type::new(Location::new(11, 28), TypeVariant::field())),
                        BlockExpression::new(Location::new(11, 34), vec![], None),
                        vec![],
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::new(13, 9),
                        false,
                        false,
                        Identifier::new(Location::new(13, 12), "f3".to_owned()),
                        vec![BindingPattern::new(
                            Location::new(13, 15),
                            BindingPatternVariant::new_binding(
                                Identifier::new(Location::new(13, 15), "a".to_owned()),
                                false,
                            ),
                            Type::new(Location::new(13, 18), TypeVariant::field()),
                        )],
                        Some(Type::new(Location::new(13, 28), TypeVariant::field())),
                        BlockExpression::new(Location::new(13, 34), vec![], None),
                        vec![],
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"impl { const VALUE: u64 = 42; }"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 6),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
