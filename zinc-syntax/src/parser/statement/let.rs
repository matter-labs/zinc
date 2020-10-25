//!
//! The `let` statement parser.
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
use crate::parser::binding::Parser as BindingParser;
use crate::parser::expression::Parser as ExpressionParser;
use crate::tree::statement::r#let::builder::Builder as LetStatementBuilder;
use crate::tree::statement::r#let::Statement as LetStatement;

/// The missing value error hint.
pub static HINT_EXPECTED_VALUE: &str = "variable must be initialized, e.g. `let value: u8 = 42;`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordLet,
    /// The `let` has been parsed so far.
    Binding,
    /// The `let {binding}` has been parsed so far.
    Equals,
    /// The `let {binding} =` has been parsed so far.
    Expression,
    /// The `let {binding} = {expression}` has been parsed so far.
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordLet
    }
}

///
/// The `let` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: LetStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'let' statement.
    ///
    /// 'let mut value: field = 42;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(LetStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordLet => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Let),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Binding;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["let"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Binding => {
                    let (binding, next) =
                        BindingParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_binding(binding);
                    self.next = next;
                    self.state = State::Equals;
                }
                State::Equals => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            ..
                        } => self.state = State::Expression,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_value(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_VALUE),
                            )));
                        }
                    }
                }
                State::Expression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_expression(expression);
                    self.next = next;
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    return match crate::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of_or_operator(
                                location,
                                vec![";"],
                                lexeme,
                                None,
                            ),
                        )),
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
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::expression::tuple::Expression as TupleExpression;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
    use crate::tree::pattern_binding::Pattern as BindingPattern;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;
    use crate::tree::statement::r#let::Statement as LetStatement;

    #[test]
    fn ok_binding() {
        let input = r#"let a = 42;"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 5), "a".to_owned()),
                            false,
                        ),
                    ),
                    None,
                ),
                ExpressionTree::new(
                    Location::test(1, 9),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 9),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding_mutable() {
        let input = r#"let mut a = 42;"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 9), "a".to_owned()),
                            true,
                        ),
                    ),
                    None,
                ),
                ExpressionTree::new(
                    Location::test(1, 13),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 13),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding_mutable_with_type() {
        let input = r#"let mut a: u232 = 42;"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_binding(
                            Identifier::new(Location::test(1, 9), "a".to_owned()),
                            true,
                        ),
                    ),
                    Some(Type::new(
                        Location::test(1, 12),
                        TypeVariant::integer_unsigned(232),
                    )),
                ),
                ExpressionTree::new(
                    Location::test(1, 19),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 19),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding_list() {
        let input = r#"let (mut a, b, mut c) = (1, 2, 3);"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_binding_list(vec![
                            BindingPattern::new(
                                Location::test(1, 6),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 10), "a".to_owned()),
                                    true,
                                ),
                            ),
                            BindingPattern::new(
                                Location::test(1, 13),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 13), "b".to_owned()),
                                    false,
                                ),
                            ),
                            BindingPattern::new(
                                Location::test(1, 16),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 20), "c".to_owned()),
                                    true,
                                ),
                            ),
                        ]),
                    ),
                    None,
                ),
                ExpressionTree::new(
                    Location::test(1, 25),
                    ExpressionTreeNode::operand(ExpressionOperand::Tuple(TupleExpression::new(
                        Location::test(1, 25),
                        vec![
                            ExpressionTree::new(
                                Location::test(1, 26),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 26),
                                        LexicalIntegerLiteral::new_decimal("1".to_owned()),
                                    ),
                                )),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 29),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 29),
                                        LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                    ),
                                )),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 32),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 32),
                                        LexicalIntegerLiteral::new_decimal("3".to_owned()),
                                    ),
                                )),
                            ),
                        ],
                    ))),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding_list_with_types() {
        let input = r#"let (mut a, b, mut c): (u8, u8, u8) = (1, 2, 3);"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_binding_list(vec![
                            BindingPattern::new(
                                Location::test(1, 6),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 10), "a".to_owned()),
                                    true,
                                ),
                            ),
                            BindingPattern::new(
                                Location::test(1, 13),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 13), "b".to_owned()),
                                    false,
                                ),
                            ),
                            BindingPattern::new(
                                Location::test(1, 16),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 20), "c".to_owned()),
                                    true,
                                ),
                            ),
                        ]),
                    ),
                    Some(Type::new(
                        Location::test(1, 24),
                        TypeVariant::tuple(vec![
                            Type::new(
                                Location::test(1, 25),
                                TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                            ),
                            Type::new(
                                Location::test(1, 29),
                                TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                            ),
                            Type::new(
                                Location::test(1, 33),
                                TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                            ),
                        ]),
                    )),
                ),
                ExpressionTree::new(
                    Location::test(1, 39),
                    ExpressionTreeNode::operand(ExpressionOperand::Tuple(TupleExpression::new(
                        Location::test(1, 39),
                        vec![
                            ExpressionTree::new(
                                Location::test(1, 40),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 40),
                                        LexicalIntegerLiteral::new_decimal("1".to_owned()),
                                    ),
                                )),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 43),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 43),
                                        LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                    ),
                                )),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 46),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 46),
                                        LexicalIntegerLiteral::new_decimal("3".to_owned()),
                                    ),
                                )),
                            ),
                        ],
                    ))),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding_list_nested() {
        let input = r#"let (mut a, b, (mut c, d, e)) = (1, 2, (3, 4, 5));"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_binding_list(vec![
                            BindingPattern::new(
                                Location::test(1, 6),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 10), "a".to_owned()),
                                    true,
                                ),
                            ),
                            BindingPattern::new(
                                Location::test(1, 13),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 13), "b".to_owned()),
                                    false,
                                ),
                            ),
                            BindingPattern::new(
                                Location::test(1, 16),
                                BindingPatternVariant::new_binding_list(vec![
                                    BindingPattern::new(
                                        Location::test(1, 17),
                                        BindingPatternVariant::new_binding(
                                            Identifier::new(Location::test(1, 21), "c".to_owned()),
                                            true,
                                        ),
                                    ),
                                    BindingPattern::new(
                                        Location::test(1, 24),
                                        BindingPatternVariant::new_binding(
                                            Identifier::new(Location::test(1, 24), "d".to_owned()),
                                            false,
                                        ),
                                    ),
                                    BindingPattern::new(
                                        Location::test(1, 27),
                                        BindingPatternVariant::new_binding(
                                            Identifier::new(Location::test(1, 27), "e".to_owned()),
                                            false,
                                        ),
                                    ),
                                ]),
                            ),
                        ]),
                    ),
                    None,
                ),
                ExpressionTree::new(
                    Location::test(1, 33),
                    ExpressionTreeNode::operand(ExpressionOperand::Tuple(TupleExpression::new(
                        Location::test(1, 33),
                        vec![
                            ExpressionTree::new(
                                Location::test(1, 34),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 34),
                                        LexicalIntegerLiteral::new_decimal("1".to_owned()),
                                    ),
                                )),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 37),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 37),
                                        LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                    ),
                                )),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 40),
                                ExpressionTreeNode::operand(ExpressionOperand::Tuple(
                                    TupleExpression::new(
                                        Location::test(1, 40),
                                        vec![
                                            ExpressionTree::new(
                                                Location::test(1, 41),
                                                ExpressionTreeNode::operand(
                                                    ExpressionOperand::LiteralInteger(
                                                        IntegerLiteral::new(
                                                            Location::test(1, 41),
                                                            LexicalIntegerLiteral::new_decimal(
                                                                "3".to_owned(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ExpressionTree::new(
                                                Location::test(1, 44),
                                                ExpressionTreeNode::operand(
                                                    ExpressionOperand::LiteralInteger(
                                                        IntegerLiteral::new(
                                                            Location::test(1, 44),
                                                            LexicalIntegerLiteral::new_decimal(
                                                                "4".to_owned(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ExpressionTree::new(
                                                Location::test(1, 47),
                                                ExpressionTreeNode::operand(
                                                    ExpressionOperand::LiteralInteger(
                                                        IntegerLiteral::new(
                                                            Location::test(1, 47),
                                                            LexicalIntegerLiteral::new_decimal(
                                                                "5".to_owned(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                )),
                            ),
                        ],
                    ))),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_binding_list_nested_with_types() {
        let input = r#"let (mut a, b, (mut c, d, e)): (u8, u8, (u8, u8, u8)) = (1, 2, (3, 4, 5));"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_binding_list(vec![
                            BindingPattern::new(
                                Location::test(1, 6),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 10), "a".to_owned()),
                                    true,
                                ),
                            ),
                            BindingPattern::new(
                                Location::test(1, 13),
                                BindingPatternVariant::new_binding(
                                    Identifier::new(Location::test(1, 13), "b".to_owned()),
                                    false,
                                ),
                            ),
                            BindingPattern::new(
                                Location::test(1, 16),
                                BindingPatternVariant::new_binding_list(vec![
                                    BindingPattern::new(
                                        Location::test(1, 17),
                                        BindingPatternVariant::new_binding(
                                            Identifier::new(Location::test(1, 21), "c".to_owned()),
                                            true,
                                        ),
                                    ),
                                    BindingPattern::new(
                                        Location::test(1, 24),
                                        BindingPatternVariant::new_binding(
                                            Identifier::new(Location::test(1, 24), "d".to_owned()),
                                            false,
                                        ),
                                    ),
                                    BindingPattern::new(
                                        Location::test(1, 27),
                                        BindingPatternVariant::new_binding(
                                            Identifier::new(Location::test(1, 27), "e".to_owned()),
                                            false,
                                        ),
                                    ),
                                ]),
                            ),
                        ]),
                    ),
                    Some(Type::new(
                        Location::test(1, 32),
                        TypeVariant::tuple(vec![
                            Type::new(
                                Location::test(1, 33),
                                TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                            ),
                            Type::new(
                                Location::test(1, 37),
                                TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                            ),
                            Type::new(
                                Location::test(1, 41),
                                TypeVariant::tuple(vec![
                                    Type::new(
                                        Location::test(1, 42),
                                        TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                                    ),
                                    Type::new(
                                        Location::test(1, 46),
                                        TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                                    ),
                                    Type::new(
                                        Location::test(1, 50),
                                        TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                                    ),
                                ]),
                            ),
                        ]),
                    )),
                ),
                ExpressionTree::new(
                    Location::test(1, 57),
                    ExpressionTreeNode::operand(ExpressionOperand::Tuple(TupleExpression::new(
                        Location::test(1, 57),
                        vec![
                            ExpressionTree::new(
                                Location::test(1, 58),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 58),
                                        LexicalIntegerLiteral::new_decimal("1".to_owned()),
                                    ),
                                )),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 61),
                                ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::test(1, 61),
                                        LexicalIntegerLiteral::new_decimal("2".to_owned()),
                                    ),
                                )),
                            ),
                            ExpressionTree::new(
                                Location::test(1, 64),
                                ExpressionTreeNode::operand(ExpressionOperand::Tuple(
                                    TupleExpression::new(
                                        Location::test(1, 64),
                                        vec![
                                            ExpressionTree::new(
                                                Location::test(1, 65),
                                                ExpressionTreeNode::operand(
                                                    ExpressionOperand::LiteralInteger(
                                                        IntegerLiteral::new(
                                                            Location::test(1, 65),
                                                            LexicalIntegerLiteral::new_decimal(
                                                                "3".to_owned(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ExpressionTree::new(
                                                Location::test(1, 68),
                                                ExpressionTreeNode::operand(
                                                    ExpressionOperand::LiteralInteger(
                                                        IntegerLiteral::new(
                                                            Location::test(1, 68),
                                                            LexicalIntegerLiteral::new_decimal(
                                                                "4".to_owned(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ExpressionTree::new(
                                                Location::test(1, 71),
                                                ExpressionTreeNode::operand(
                                                    ExpressionOperand::LiteralInteger(
                                                        IntegerLiteral::new(
                                                            Location::test(1, 71),
                                                            LexicalIntegerLiteral::new_decimal(
                                                                "5".to_owned(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                )),
                            ),
                        ],
                    ))),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_wildcard() {
        let input = r#"let _ = 42;"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_wildcard(),
                    ),
                    None,
                ),
                ExpressionTree::new(
                    Location::test(1, 9),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 9),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_wildcard_with_type() {
        let input = r#"let _: u8 = 42;"#;

        let expected = Ok((
            LetStatement::new(
                Location::test(1, 1),
                Binding::new(
                    Location::test(1, 5),
                    BindingPattern::new(
                        Location::test(1, 5),
                        BindingPatternVariant::new_wildcard(),
                    ),
                    Some(Type::new(
                        Location::test(1, 8),
                        TypeVariant::integer_unsigned(zinc_const::bitlength::BYTE),
                    )),
                ),
                ExpressionTree::new(
                    Location::test(1, 13),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 13),
                            LexicalIntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_value_without_type() {
        let input = r#"let a;"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_value(
            Location::test(1, 6),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_value_with_type() {
        let input = r#"let a: u64;"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_value(
            Location::test(1, 11),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"let a: u64 = 42"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 16),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
