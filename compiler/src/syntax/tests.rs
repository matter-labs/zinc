//!
//! The syntax parser tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical;
use crate::lexical::IntegerLiteral;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::syntax::CircuitProgram;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Field;
use crate::syntax::Identifier;
use crate::syntax::LetStatement;
use crate::syntax::Literal;
use crate::syntax::Parser;
use crate::syntax::Statement;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

#[test]
fn ok() {
    let input = r#"
input {
    a: u8,
}

witness {
    b: i248,
}

output {
    c: field,
}

let mut d: u232 = 2 + 2;
"#;

    let expected = Ok(CircuitProgram {
        inputs: vec![Field::new(
            Location::new(3, 5),
            Identifier::new(Location::new(3, 5), "a".to_owned()),
            Type::new(Location::new(3, 8), TypeVariant::new_integer_unsigned(8)),
        )],
        witnesses: vec![Field::new(
            Location::new(7, 5),
            Identifier::new(Location::new(7, 5), "b".to_owned()),
            Type::new(Location::new(7, 8), TypeVariant::new_integer_signed(248)),
        )],
        outputs: vec![Field::new(
            Location::new(11, 5),
            Identifier::new(Location::new(11, 5), "c".to_owned()),
            Type::new(Location::new(11, 8), TypeVariant::new_field()),
        )],
        statements: vec![Statement::Let(LetStatement {
            location: Location::new(14, 1),
            identifier: Identifier::new(Location::new(14, 9), "d".to_owned()),
            r#type: Some(Type::new(
                Location::new(14, 12),
                TypeVariant::new_integer_unsigned(232),
            )),
            expression: Expression::new(
                Location::new(14, 19),
                vec![
                    ExpressionElement::new(
                        Location::new(14, 19),
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                            Location::new(14, 19),
                            lexical::Literal::Integer(IntegerLiteral::new_decimal("2".to_owned())),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(14, 23),
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                            Location::new(14, 23),
                            lexical::Literal::Integer(IntegerLiteral::new_decimal("2".to_owned())),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(14, 21),
                        ExpressionObject::Operator(ExpressionOperator::Addition),
                    ),
                ],
            ),
            is_mutable: true,
        })],
    });

    let result = Parser::default().parse(input.to_owned());

    assert_eq!(expected, result);
}

#[test]
fn error_expected() {
    use crate::lexical::Lexeme;
    use crate::lexical::Symbol;

    let input = "input }";

    let result: Result<CircuitProgram, Error> = Parser::default().parse(input.to_owned());

    let expected: Result<CircuitProgram, Error> = Err(Error::Syntax(SyntaxError::Expected(
        Location::new(1, 7),
        vec!["{"],
        Lexeme::Symbol(Symbol::BracketCurlyRight),
    )));

    assert_eq!(expected, result);
}

#[test]
fn error_unexpected_eof() {
    let input = "input";

    let result: Result<CircuitProgram, Error> = Parser::default().parse(input.to_owned());

    let expected: Result<CircuitProgram, Error> = Err(Error::Syntax(SyntaxError::Expected(
        Location::new(1, 6),
        vec!["{"],
        Lexeme::Eof,
    )));

    assert_eq!(expected, result);
}

#[test]
fn error_expression_statement_at_root() {
    use crate::error::Error;

    let input = r#"
    input {}
    witness {}
    output {}

    2 + 2
"#;

    let result: Result<CircuitProgram, Error> = Parser::default().parse(input.to_owned());

    let expected: Result<CircuitProgram, Error> = Err(Error::Syntax(
        SyntaxError::ExpressionStatementAtRoot(Location::new(6, 5)),
    ));

    assert_eq!(expected, result);
}
