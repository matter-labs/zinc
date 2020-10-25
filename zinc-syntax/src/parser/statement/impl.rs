//!
//! The `impl` statement parser.
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
use crate::parser::statement::local_impl::Parser as ImplementationLocalStatementParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#impl::builder::Builder as ImplStatementBuilder;
use crate::tree::statement::r#impl::Statement as ImplStatement;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str =
    "type implementation must have an identifier, e.g. `impl Data { ... }`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordImpl,
    /// The `impl` has been parsed so far.
    Identifier,
    /// The `impl {identifier}` has been parsed so far.
    BracketCurlyLeft,
    /// The `impl {identifier} {` has been parsed so far.
    StatementOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordImpl
    }
}

///
/// The `impl` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: ImplStatementBuilder,
    /// The token returned from a subparser.
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
        initial: Option<Token>,
    ) -> Result<(ImplStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordImpl => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Impl),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["impl"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::BracketCurlyLeft;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::BracketCurlyLeft => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::binding::Binding;
    use crate::tree::expression::block::Expression as BlockExpression;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::tree::pattern_binding::Pattern as BindingPattern;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;
    use crate::tree::statement::local_impl::Statement as ImplementationLocalStatement;
    use crate::tree::statement::r#const::Statement as ConstStatement;
    use crate::tree::statement::r#fn::Statement as FnStatement;
    use crate::tree::statement::r#impl::Statement as ImplStatement;

    #[test]
    fn ok_empty() {
        let input = r#"
    impl Test {}
"#;

        let expected = Ok((
            ImplStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

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
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![ImplementationLocalStatement::Const(ConstStatement::new(
                    Location::test(3, 9),
                    Identifier::new(Location::test(3, 15), "VALUE".to_owned()),
                    Type::new(Location::test(3, 22), TypeVariant::integer_unsigned(64)),
                    ExpressionTree::new(
                        Location::test(3, 28),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(3, 28),
                                LexicalIntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    ),
                ))],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

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
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::test(3, 9),
                        Identifier::new(Location::test(3, 15), "VALUE".to_owned()),
                        Type::new(Location::test(3, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(3, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(3, 28),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::test(5, 9),
                        Identifier::new(Location::test(5, 15), "ANOTHER".to_owned()),
                        Type::new(Location::test(5, 24), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(5, 30),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(5, 30),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::test(7, 9),
                        Identifier::new(Location::test(7, 15), "YET_ANOTHER".to_owned()),
                        Type::new(Location::test(7, 28), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(7, 34),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(7, 34),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

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
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![ImplementationLocalStatement::Fn(FnStatement::new(
                    Location::test(3, 9),
                    false,
                    false,
                    Identifier::new(Location::test(3, 12), "f".to_owned()),
                    vec![Binding::new(
                        Location::test(3, 14),
                        BindingPattern::new(
                            Location::test(3, 14),
                            BindingPatternVariant::new_binding(
                                Identifier::new(Location::test(3, 14), "a".to_owned()),
                                false,
                            ),
                        ),
                        Some(Type::new(Location::test(3, 17), TypeVariant::field())),
                    )],
                    Some(Type::new(Location::test(3, 27), TypeVariant::field())),
                    BlockExpression::new(Location::test(3, 33), vec![], None),
                    vec![],
                ))],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

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
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::test(3, 9),
                        false,
                        false,
                        Identifier::new(Location::test(3, 12), "f1".to_owned()),
                        vec![Binding::new(
                            Location::test(3, 15),
                            BindingPattern::new(
                                Location::test(3, 15),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(3, 15), "a".to_owned()),
                                    false,
                                ),
                            ),
                            Some(Type::new(Location::test(3, 18), TypeVariant::field())),
                        )],
                        Some(Type::new(Location::test(3, 28), TypeVariant::field())),
                        BlockExpression::new(Location::test(3, 34), vec![], None),
                        vec![],
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::test(5, 9),
                        false,
                        false,
                        Identifier::new(Location::test(5, 12), "f2".to_owned()),
                        vec![Binding::new(
                            Location::test(5, 15),
                            BindingPattern::new(
                                Location::test(5, 15),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(5, 15), "a".to_owned()),
                                    false,
                                ),
                            ),
                            Some(Type::new(Location::test(5, 18), TypeVariant::field())),
                        )],
                        Some(Type::new(Location::test(5, 28), TypeVariant::field())),
                        BlockExpression::new(Location::test(5, 34), vec![], None),
                        vec![],
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::test(7, 9),
                        false,
                        false,
                        Identifier::new(Location::test(7, 12), "f3".to_owned()),
                        vec![Binding::new(
                            Location::test(7, 15),
                            BindingPattern::new(
                                Location::test(7, 15),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(7, 15), "a".to_owned()),
                                    false,
                                ),
                            ),
                            Some(Type::new(Location::test(7, 18), TypeVariant::field())),
                        )],
                        Some(Type::new(Location::test(7, 28), TypeVariant::field())),
                        BlockExpression::new(Location::test(7, 34), vec![], None),
                        vec![],
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

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
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::test(3, 9),
                        Identifier::new(Location::test(3, 15), "VALUE".to_owned()),
                        Type::new(Location::test(3, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(3, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(3, 28),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::test(5, 9),
                        false,
                        false,
                        Identifier::new(Location::test(5, 12), "f".to_owned()),
                        vec![Binding::new(
                            Location::test(5, 14),
                            BindingPattern::new(
                                Location::test(5, 14),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(5, 14), "a".to_owned()),
                                    false,
                                ),
                            ),
                            Some(Type::new(Location::test(5, 17), TypeVariant::field())),
                        )],
                        Some(Type::new(Location::test(5, 27), TypeVariant::field())),
                        BlockExpression::new(Location::test(5, 33), vec![], None),
                        vec![],
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

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
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::test(3, 9),
                        Identifier::new(Location::test(3, 15), "VALUE".to_owned()),
                        Type::new(Location::test(3, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(3, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(3, 28),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::test(5, 9),
                        Identifier::new(Location::test(5, 15), "ANOTHER".to_owned()),
                        Type::new(Location::test(5, 24), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(5, 30),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(5, 30),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Const(ConstStatement::new(
                        Location::test(7, 9),
                        Identifier::new(Location::test(7, 15), "YET_ANOTHER".to_owned()),
                        Type::new(Location::test(7, 28), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(7, 34),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(7, 34),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::test(9, 9),
                        false,
                        false,
                        Identifier::new(Location::test(9, 12), "f1".to_owned()),
                        vec![Binding::new(
                            Location::test(9, 15),
                            BindingPattern::new(
                                Location::test(9, 15),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(9, 15), "a".to_owned()),
                                    false,
                                ),
                            ),
                            Some(Type::new(Location::test(9, 18), TypeVariant::field())),
                        )],
                        Some(Type::new(Location::test(9, 28), TypeVariant::field())),
                        BlockExpression::new(Location::test(9, 34), vec![], None),
                        vec![],
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::test(11, 9),
                        false,
                        false,
                        Identifier::new(Location::test(11, 12), "f2".to_owned()),
                        vec![Binding::new(
                            Location::test(11, 15),
                            BindingPattern::new(
                                Location::test(11, 15),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(11, 15), "a".to_owned()),
                                    false,
                                ),
                            ),
                            Some(Type::new(Location::test(11, 18), TypeVariant::field())),
                        )],
                        Some(Type::new(Location::test(11, 28), TypeVariant::field())),
                        BlockExpression::new(Location::test(11, 34), vec![], None),
                        vec![],
                    )),
                    ImplementationLocalStatement::Fn(FnStatement::new(
                        Location::test(13, 9),
                        false,
                        false,
                        Identifier::new(Location::test(13, 12), "f3".to_owned()),
                        vec![Binding::new(
                            Location::test(13, 15),
                            BindingPattern::new(
                                Location::test(13, 15),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(13, 15), "a".to_owned()),
                                    false,
                                ),
                            ),
                            Some(Type::new(Location::test(13, 18), TypeVariant::field())),
                        )],
                        Some(Type::new(Location::test(13, 28), TypeVariant::field())),
                        BlockExpression::new(Location::test(13, 34), vec![], None),
                        vec![],
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"impl { const VALUE: u64 = 42; }"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 6),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
