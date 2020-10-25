//!
//! The function-local statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::Parser as ExpressionParser;
use crate::parser::statement::r#const::Parser as ConstStatementParser;
use crate::parser::statement::r#for::Parser as ForStatementParser;
use crate::parser::statement::r#let::Parser as LetStatementParser;
use crate::tree::statement::local_fn::Statement as FunctionLocalStatement;

///
/// The function-local statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a statement allowed in functions.
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(FunctionLocalStatement, Option<Token>, bool), ParsingError> {
        self.next = initial;

        let statement = match crate::parser::take_or_next(self.next.take(), stream.clone())? {
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Let),
                ..
            } => {
                let (statement, next) =
                    LetStatementParser::default().parse(stream.clone(), Some(token))?;
                self.next = next;
                FunctionLocalStatement::Let(statement)
            }
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Const),
                ..
            } => {
                let (statement, next) =
                    ConstStatementParser::default().parse(stream.clone(), Some(token))?;
                self.next = next;
                FunctionLocalStatement::Const(statement)
            }
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::For),
                ..
            } => {
                let (statement, next) =
                    ForStatementParser::default().parse(stream.clone(), Some(token))?;
                self.next = next;
                FunctionLocalStatement::For(statement)
            }
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                location,
            } => return Ok((FunctionLocalStatement::Empty(location), None, false)),
            token => {
                let (expression, next) =
                    ExpressionParser::default().parse(stream.clone(), Some(token))?;
                self.next = next;
                FunctionLocalStatement::Expression(expression)
            }
        };
        match statement {
            statement @ FunctionLocalStatement::Expression { .. } => {
                match crate::parser::take_or_next(self.next.take(), stream)? {
                    Token {
                        lexeme: Lexeme::Symbol(Symbol::Semicolon),
                        ..
                    } => Ok((statement, None, false)),
                    token => Ok((statement, Some(token), true)),
                }
            }
            statement => Ok((statement, None, false)),
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
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
    use crate::tree::statement::local_fn::Statement as FunctionLocalStatement;
    use crate::tree::statement::r#let::Statement as LetStatement;

    #[test]
    fn ok_semicolon_terminated() {
        let input = r#"let mut a: u232 = 42;"#;

        let expected = Ok((
            FunctionLocalStatement::Let(LetStatement::new(
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
            )),
            None,
            false,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_semicolon_unterminated() {
        let input = r#"{ 42 }"#;

        let expected = Ok((
            FunctionLocalStatement::Expression(ExpressionTree::new(
                Location::test(1, 1),
                ExpressionTreeNode::operand(ExpressionOperand::Block(BlockExpression::new(
                    Location::test(1, 1),
                    vec![],
                    Some(ExpressionTree::new(
                        Location::test(1, 3),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 3),
                                LexicalIntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    )),
                ))),
            )),
            Some(Token::new(Lexeme::Eof, Location::test(1, 7))),
            true,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
