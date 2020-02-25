//!
//! The conditional expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::block::Parser as BlockExpressionParser;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::expression::conditional::builder::Builder as ConditionalExpressionBuilder;
use crate::syntax::tree::expression::conditional::Expression as ConditionalExpression;
use crate::syntax::tree::expression::element::Element as ExpressionElement;
use crate::syntax::tree::expression::object::Object as ExpressionObject;
use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::Expression;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordIf,
    Condition,
    MainBlock,
    ElseKeywordOrEnd,
    KeywordIfOrElseBlock,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordIf
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ConditionalExpressionBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ConditionalExpression, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordIf => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::If),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Condition;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["if"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Condition => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_condition(expression);
                    self.state = State::MainBlock;
                }
                State::MainBlock => {
                    let block =
                        BlockExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_main_block(block);
                    self.state = State::ElseKeywordOrEnd;
                }
                State::ElseKeywordOrEnd => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Else),
                            ..
                        } => self.state = State::KeywordIfOrElseBlock,
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::KeywordIfOrElseBlock => {
                    return match crate::syntax::parser::take_or_next(
                        self.next.take(),
                        stream.clone(),
                    )? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::If),
                            ..
                        } => {
                            let (expression, next) = Self::default().parse(stream, Some(token))?;
                            let block = BlockExpression::new(
                                expression.location,
                                Vec::new(),
                                Some(Expression::new(
                                    expression.location,
                                    vec![ExpressionElement::new(
                                        expression.location,
                                        ExpressionObject::Operand(ExpressionOperand::Conditional(
                                            expression,
                                        )),
                                    )],
                                )),
                            );
                            self.builder.set_else_block(block);
                            Ok((self.builder.finish(), next))
                        }
                        token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            let block =
                                BlockExpressionParser::default().parse(stream, Some(token))?;
                            self.builder.set_else_block(block);
                            Ok((self.builder.finish(), None))
                        }
                        Token { lexeme, location } => Err(Error::Syntax(
                            SyntaxError::expected_one_of(location, vec!["if", "{"], lexeme, None),
                        )),
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Error;
    use super::Parser;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::block::Expression as BlockExpression;
    use crate::syntax::tree::expression::conditional::Expression as ConditionalExpression;
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_nested() {
        let input = r#"if true { 1 } else if false { 2 } else { 3 }"#;

        let expected = Ok((
            ConditionalExpression::new(
                Location::new(1, 1),
                Expression::new(
                    Location::new(1, 4),
                    vec![ExpressionElement::new(
                        Location::new(1, 4),
                        ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(Location::new(1, 4), lexical::BooleanLiteral::True),
                        )),
                    )],
                ),
                BlockExpression::new(
                    Location::new(1, 9),
                    vec![],
                    Some(Expression::new(
                        Location::new(1, 11),
                        vec![ExpressionElement::new(
                            Location::new(1, 11),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 11),
                                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        )],
                    )),
                ),
                Some(BlockExpression::new(
                    Location::new(1, 20),
                    vec![],
                    Some(Expression::new(
                        Location::new(1, 20),
                        vec![ExpressionElement::new(
                            Location::new(1, 20),
                            ExpressionObject::Operand(ExpressionOperand::Conditional(
                                ConditionalExpression::new(
                                    Location::new(1, 20),
                                    Expression::new(
                                        Location::new(1, 23),
                                        vec![ExpressionElement::new(
                                            Location::new(1, 23),
                                            ExpressionObject::Operand(
                                                ExpressionOperand::LiteralBoolean(
                                                    BooleanLiteral::new(
                                                        Location::new(1, 23),
                                                        lexical::BooleanLiteral::False,
                                                    ),
                                                ),
                                            ),
                                        )],
                                    ),
                                    BlockExpression::new(
                                        Location::new(1, 29),
                                        vec![],
                                        Some(Expression::new(
                                            Location::new(1, 31),
                                            vec![ExpressionElement::new(
                                                Location::new(1, 31),
                                                ExpressionObject::Operand(
                                                    ExpressionOperand::LiteralInteger(
                                                        IntegerLiteral::new(
                                                            Location::new(1, 31),
                                                            lexical::IntegerLiteral::new_decimal(
                                                                "2".to_owned(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            )],
                                        )),
                                    ),
                                    Some(BlockExpression::new(
                                        Location::new(1, 40),
                                        vec![],
                                        Some(Expression::new(
                                            Location::new(1, 42),
                                            vec![ExpressionElement::new(
                                                Location::new(1, 42),
                                                ExpressionObject::Operand(
                                                    ExpressionOperand::LiteralInteger(
                                                        IntegerLiteral::new(
                                                            Location::new(1, 42),
                                                            lexical::IntegerLiteral::new_decimal(
                                                                "3".to_owned(),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            )],
                                        )),
                                    )),
                                ),
                            )),
                        )],
                    )),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"if true { 42 } else ("#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 21),
            vec!["if", "{"],
            Lexeme::Symbol(Symbol::ParenthesisLeft),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
