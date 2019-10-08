//!
//! The syntax parser tests.
//!

#![cfg(test)]

use crate::lexical;
use crate::lexical::IntegerLiteral;
use crate::lexical::Location;
use crate::lexical::TokenStream;
use crate::syntax::CircuitProgram;
use crate::syntax::Error;
use crate::syntax::Expression;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::Input;
use crate::syntax::LetStatement;
use crate::syntax::Literal;
use crate::syntax::Parser;
use crate::syntax::Statement;
use crate::syntax::Type;
use crate::syntax::TypeVariant;
use crate::syntax::Witness;

#[test]
fn ok() {
    let input = r#"
inputs {
    a: u8,
}

witness {
    b: i248,
}

let mut c: u232 = 2 + 2;
"#;

    let expected = Ok(CircuitProgram {
        inputs: vec![Input::new(
            Location::new(3, 5),
            Identifier::new(Location::new(3, 5), "a".to_owned()),
            Type::new(Location::new(3, 8), TypeVariant::new_integer_unsigned(8)),
        )],
        witnesses: vec![Witness::new(
            Location::new(7, 5),
            Identifier::new(Location::new(7, 5), "b".to_owned()),
            Type::new(Location::new(7, 8), TypeVariant::new_integer_signed(248)),
        )],
        statements: vec![Statement::Let(LetStatement {
            location: Location::new(10, 1),
            identifier: Identifier::new(Location::new(10, 9), "c".to_owned()),
            r#type: Some(Type::new(
                Location::new(10, 12),
                TypeVariant::new_integer_unsigned(232),
            )),
            expression: Expression::new(
                Location::new(10, 19),
                vec![
                    ExpressionElement::new(
                        Location::new(10, 19),
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                            Location::new(10, 19),
                            lexical::Literal::Integer(IntegerLiteral::new_decimal("2".to_owned())),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(10, 23),
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                            Location::new(10, 23),
                            lexical::Literal::Integer(IntegerLiteral::new_decimal("2".to_owned())),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(10, 21),
                        ExpressionObject::Operator(ExpressionOperator::Addition),
                    ),
                ],
            ),
            is_mutable: true,
        })],
    });

    let result = Parser::parse(TokenStream::new(input.to_owned()));

    assert_eq!(expected, result);
}

#[test]
fn err_unexpected_end() {
    use crate::Error as MainError;

    let input = "inputs";

    let result: Result<CircuitProgram, MainError> =
        Parser::parse(TokenStream::new(input.to_owned()));

    let expected: Result<CircuitProgram, MainError> =
        Err(MainError::Syntax(Error::UnexpectedEnd(Location::new(1, 7))));

    assert_eq!(expected, result);
}

#[test]
fn err_expected() {
    use crate::lexical::Lexeme;
    use crate::lexical::Symbol;
    use crate::Error as MainError;

    let input = "inputs }";

    let result: Result<CircuitProgram, MainError> =
        Parser::parse(TokenStream::new(input.to_owned()));

    let expected: Result<CircuitProgram, MainError> = Err(MainError::Syntax(Error::Expected(
        Location::new(1, 8),
        vec!["{"],
        Lexeme::Symbol(Symbol::BracketCurlyRight),
    )));

    assert_eq!(expected, result);
}

#[test]
fn err_unexpected_expression_at_root() {
    use crate::Error as MainError;

    let input = r#"
    inputs {}

    2 + 2
"#;

    let result: Result<CircuitProgram, MainError> =
        Parser::parse(TokenStream::new(input.to_owned()));

    let expected: Result<CircuitProgram, MainError> = Err(MainError::Syntax(
        Error::UnterminatedExpressionAtRoot(Location::new(4, 5)),
    ));

    assert_eq!(expected, result);
}
