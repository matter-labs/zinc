//!
//! The match expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::MatchExpression;
use crate::syntax::MatchExpressionBuilder;
use crate::syntax::MatchPatternParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordMatch,
    ScrutineeExpression,
    BracketCurlyLeft,
    BracketCurlyRightOrBranchPattern,
    Select,
    BranchExpression,
    CommaOrBracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordMatch
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: MatchExpressionBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<MatchExpression, Error> {
        loop {
            match self.state {
                State::KeywordMatch => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Match),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::ScrutineeExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["match"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::ScrutineeExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_scrutinee_expression(expression);
                    self.state = State::BracketCurlyLeft;
                }
                State::BracketCurlyLeft => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => self.state = State::BracketCurlyRightOrBranchPattern,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::BracketCurlyRightOrBranchPattern => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        token => {
                            let (pattern, next) =
                                MatchPatternParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_branch_pattern(pattern);
                            self.state = State::Select;
                        }
                    }
                }
                State::Select => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::EqualsGreater),
                            ..
                        } => self.state = State::BranchExpression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["=>"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::BranchExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_branch_expression(expression);
                    self.state = State::CommaOrBracketCurlyRight;
                }
                State::CommaOrBracketCurlyRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::BracketCurlyRightOrBranchPattern,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", "}"],
                                lexeme,
                            )));
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
    use crate::lexical;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BooleanLiteral;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::MatchExpression;
    use crate::syntax::MatchPattern;
    use crate::syntax::MatchPatternVariant;

    #[test]
    fn ok_single() {
        let input = r#"
    match test {
        false => true,
    }
"#;

        let expected = Ok(MatchExpression::new(
            Location::new(2, 5),
            Expression::new(
                Location::new(2, 11),
                vec![ExpressionElement::new(
                    Location::new(2, 11),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(2, 11),
                        "test".to_owned(),
                    ))),
                )],
            ),
            vec![(
                MatchPattern::new(
                    Location::new(3, 9),
                    MatchPatternVariant::new_boolean_literal(BooleanLiteral::new(
                        Location::new(3, 9),
                        lexical::BooleanLiteral::False,
                    )),
                ),
                Expression::new(
                    Location::new(3, 18),
                    vec![ExpressionElement::new(
                        Location::new(3, 18),
                        ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(3, 18),
                                lexical::BooleanLiteral::True,
                            ),
                        )),
                    )],
                ),
            )],
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"
    match test {
        1 => 1,
        2 => 2,
        _ => 3,
    }
"#;
        let expected = Ok(MatchExpression::new(
            Location::new(2, 5),
            Expression::new(
                Location::new(2, 11),
                vec![ExpressionElement::new(
                    Location::new(2, 11),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(2, 11),
                        "test".to_owned(),
                    ))),
                )],
            ),
            vec![
                (
                    MatchPattern::new(
                        Location::new(3, 9),
                        MatchPatternVariant::new_integer_literal(IntegerLiteral::new(
                            Location::new(3, 9),
                            lexical::IntegerLiteral::new_decimal("1".to_owned()),
                        )),
                    ),
                    Expression::new(
                        Location::new(3, 14),
                        vec![ExpressionElement::new(
                            Location::new(3, 14),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(3, 14),
                                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        )],
                    ),
                ),
                (
                    MatchPattern::new(
                        Location::new(4, 9),
                        MatchPatternVariant::new_integer_literal(IntegerLiteral::new(
                            Location::new(4, 9),
                            lexical::IntegerLiteral::new_decimal("2".to_owned()),
                        )),
                    ),
                    Expression::new(
                        Location::new(4, 14),
                        vec![ExpressionElement::new(
                            Location::new(4, 14),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(4, 14),
                                    lexical::IntegerLiteral::new_decimal("2".to_owned()),
                                ),
                            )),
                        )],
                    ),
                ),
                (
                    MatchPattern::new(Location::new(5, 9), MatchPatternVariant::new_wildcard()),
                    Expression::new(
                        Location::new(5, 14),
                        vec![ExpressionElement::new(
                            Location::new(5, 14),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(5, 14),
                                    lexical::IntegerLiteral::new_decimal("3".to_owned()),
                                ),
                            )),
                        )],
                    ),
                ),
            ],
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_empty() {
        let input = r#"
    match test {}
"#;

        let expected = Ok(MatchExpression::new(
            Location::new(2, 5),
            Expression::new(
                Location::new(2, 11),
                vec![ExpressionElement::new(
                    Location::new(2, 11),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(2, 11),
                        "test".to_owned(),
                    ))),
                )],
            ),
            vec![],
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
