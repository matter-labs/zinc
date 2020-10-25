//!
//! The `contract` statement parser.
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
use crate::parser::statement::local_contract::Parser as ContractLocalStatementParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::contract::builder::Builder as ContractStatementBuilder;
use crate::tree::statement::contract::Statement as ContractStatement;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str =
    "contract must have an identifier, e.g. `contract Uniswap { ... }`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordContract,
    /// The `contract` has been parsed so far.
    Identifier,
    /// The `contract {identifier}` has been parsed so far.
    BracketCurlyLeftOrEnd,
    /// The `contract {identifier} {` has been parsed so far.
    StatementOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordContract
    }
}

///
/// The `contract` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: ContractStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'contract' statement.
    ///
    /// '
    /// contract Uniswap {
    ///     ...
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ContractStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordContract => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Contract),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["contract"],
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
                            self.state = State::BracketCurlyLeftOrEnd;
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
                State::BracketCurlyLeftOrEnd => {
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
                            let (statement, next) = ContractLocalStatementParser::default()
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
    use zinc_lexical::Token;
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
    use crate::tree::statement::contract::Statement as ContractStatement;
    use crate::tree::statement::field::Statement as FieldStatement;
    use crate::tree::statement::local_contract::Statement as ContractLocalStatement;
    use crate::tree::statement::r#const::Statement as ConstStatement;
    use crate::tree::statement::r#fn::Statement as FnStatement;

    #[test]
    fn ok_empty_with_brackets() {
        let input = r#"
    contract Test {}
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_empty_with_semicolon() {
        let input = r#"
    contract Test;
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![],
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::test(2, 18),
            )),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_field() {
        let input = r#"
    contract Test {
        a: u232;
    }
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![ContractLocalStatement::Field(FieldStatement::new(
                    Location::test(3, 9),
                    false,
                    Identifier::new(Location::test(3, 9), "a".to_owned()),
                    Type::new(Location::test(3, 12), TypeVariant::integer_unsigned(232)),
                ))],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple_fields() {
        let input = r#"
    contract Test {
        a: u232;
        pub b: u232;
        pub c: u232;
    }
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![
                    ContractLocalStatement::Field(FieldStatement::new(
                        Location::test(3, 9),
                        false,
                        Identifier::new(Location::test(3, 9), "a".to_owned()),
                        Type::new(Location::test(3, 12), TypeVariant::integer_unsigned(232)),
                    )),
                    ContractLocalStatement::Field(FieldStatement::new(
                        Location::test(4, 9),
                        true,
                        Identifier::new(Location::test(4, 13), "b".to_owned()),
                        Type::new(Location::test(4, 16), TypeVariant::integer_unsigned(232)),
                    )),
                    ContractLocalStatement::Field(FieldStatement::new(
                        Location::test(5, 9),
                        true,
                        Identifier::new(Location::test(5, 13), "c".to_owned()),
                        Type::new(Location::test(5, 16), TypeVariant::integer_unsigned(232)),
                    )),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_constant() {
        let input = r#"
    contract Test {
        const VALUE: u64 = 42;
    }
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![ContractLocalStatement::Const(ConstStatement::new(
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
    contract Test {
        const VALUE: u64 = 42;
        const ANOTHER: u64 = 42;
        const YET_ANOTHER: u64 = 42;
    }
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![
                    ContractLocalStatement::Const(ConstStatement::new(
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
                    ContractLocalStatement::Const(ConstStatement::new(
                        Location::test(4, 9),
                        Identifier::new(Location::test(4, 15), "ANOTHER".to_owned()),
                        Type::new(Location::test(4, 24), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(4, 30),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(4, 30),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ContractLocalStatement::Const(ConstStatement::new(
                        Location::test(5, 9),
                        Identifier::new(Location::test(5, 15), "YET_ANOTHER".to_owned()),
                        Type::new(Location::test(5, 28), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(5, 34),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(5, 34),
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
    contract Test {
        fn f(a: field) -> field {}
    }
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![ContractLocalStatement::Fn(FnStatement::new(
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
    contract Test {
        fn f1(a: field) -> field {}

        fn f2(a: field) -> field {}

        fn f3(a: field) -> field {}
    }
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![
                    ContractLocalStatement::Fn(FnStatement::new(
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
                    ContractLocalStatement::Fn(FnStatement::new(
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
                    ContractLocalStatement::Fn(FnStatement::new(
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
    fn ok_single_field_single_constant_single_function() {
        let input = r#"
    contract Test {
        pub a: u232;

        const VALUE: u64 = 42;

        fn f1(a: field) -> field {}
    }
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![
                    ContractLocalStatement::Field(FieldStatement::new(
                        Location::test(3, 9),
                        true,
                        Identifier::new(Location::test(3, 13), "a".to_owned()),
                        Type::new(Location::test(3, 16), TypeVariant::integer_unsigned(232)),
                    )),
                    ContractLocalStatement::Const(ConstStatement::new(
                        Location::test(5, 9),
                        Identifier::new(Location::test(5, 15), "VALUE".to_owned()),
                        Type::new(Location::test(5, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(5, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(5, 28),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ContractLocalStatement::Fn(FnStatement::new(
                        Location::test(7, 9),
                        false,
                        false,
                        Identifier::new(Location::test(7, 12), "f1".to_owned()),
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
    fn ok_multiple_fields_multiple_constants_multiple_functions() {
        let input = r#"
    contract Test {
        a: u232;
        pub b: u232;
        pub c: u232;

        const VALUE: u64 = 42;
        const ANOTHER: u64 = 42;
        const YET_ANOTHER: u64 = 42;

        fn f1(a: field) -> field {}

        fn f2(a: field) -> field {}

        fn f3(a: field) -> field {}
    }
"#;

        let expected = Ok((
            ContractStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 14), "Test".to_owned()),
                vec![
                    ContractLocalStatement::Field(FieldStatement::new(
                        Location::test(3, 9),
                        false,
                        Identifier::new(Location::test(3, 9), "a".to_owned()),
                        Type::new(Location::test(3, 12), TypeVariant::integer_unsigned(232)),
                    )),
                    ContractLocalStatement::Field(FieldStatement::new(
                        Location::test(4, 9),
                        true,
                        Identifier::new(Location::test(4, 13), "b".to_owned()),
                        Type::new(Location::test(4, 16), TypeVariant::integer_unsigned(232)),
                    )),
                    ContractLocalStatement::Field(FieldStatement::new(
                        Location::test(5, 9),
                        true,
                        Identifier::new(Location::test(5, 13), "c".to_owned()),
                        Type::new(Location::test(5, 16), TypeVariant::integer_unsigned(232)),
                    )),
                    ContractLocalStatement::Const(ConstStatement::new(
                        Location::test(7, 9),
                        Identifier::new(Location::test(7, 15), "VALUE".to_owned()),
                        Type::new(Location::test(7, 22), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(7, 28),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(7, 28),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ContractLocalStatement::Const(ConstStatement::new(
                        Location::test(8, 9),
                        Identifier::new(Location::test(8, 15), "ANOTHER".to_owned()),
                        Type::new(Location::test(8, 24), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(8, 30),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(8, 30),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ContractLocalStatement::Const(ConstStatement::new(
                        Location::test(9, 9),
                        Identifier::new(Location::test(9, 15), "YET_ANOTHER".to_owned()),
                        Type::new(Location::test(9, 28), TypeVariant::integer_unsigned(64)),
                        ExpressionTree::new(
                            Location::test(9, 34),
                            ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::test(9, 34),
                                    LexicalIntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                    )),
                    ContractLocalStatement::Fn(FnStatement::new(
                        Location::test(11, 9),
                        false,
                        false,
                        Identifier::new(Location::test(11, 12), "f1".to_owned()),
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
                    ContractLocalStatement::Fn(FnStatement::new(
                        Location::test(13, 9),
                        false,
                        false,
                        Identifier::new(Location::test(13, 12), "f2".to_owned()),
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
                    ContractLocalStatement::Fn(FnStatement::new(
                        Location::test(15, 9),
                        false,
                        false,
                        Identifier::new(Location::test(15, 12), "f3".to_owned()),
                        vec![Binding::new(
                            Location::test(15, 15),
                            BindingPattern::new(
                                Location::test(15, 15),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(15, 15), "a".to_owned()),
                                    false,
                                ),
                            ),
                            Some(Type::new(Location::test(15, 18), TypeVariant::field())),
                        )],
                        Some(Type::new(Location::test(15, 28), TypeVariant::field())),
                        BlockExpression::new(Location::test(15, 34), vec![], None),
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
        let input = r#"contract { a: u8 };"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 10),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_right() {
        let input = r#"contract Data { a: u8; );"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 24),
            Lexeme::Symbol(Symbol::ParenthesisRight),
            Some(crate::parser::statement::field::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
