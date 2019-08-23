//!
//! The syntax tests.
//!

#![cfg(test)]

use super::*;
use crate::lexical::*;

#[test]
fn success() {
    let code = br#"/*
    This is the mega ultra inputs input!
*/
inputs {
    a: uint8; // input 1
    b: field; // input 2
    c: bool; // input 3
} /* This is the end of the mega ultra inputs input */

/*
    This is the mega ultra witness input!
*/
witness {
    d: int126; // witness 1
    e: field; // witness 2
    f: bool; // witness 3
} /* This is the end of the mega ultra witness input */

let mut x: uint228 = 2 + 2;"#;

    let result: CircuitProgram = parse(TokenStream::new(code.to_vec())).unwrap();

    let expected: CircuitProgram = CircuitProgram {
        inputs: vec![
            Input::new(Identifier::new("a"), Type::Uint { bitlength: 8 }),
            Input::new(Identifier::new("b"), Type::Field),
            Input::new(Identifier::new("c"), Type::Bool),
        ],
        witnesses: vec![
            Witness::new(Identifier::new("d"), Type::Int { bitlength: 126 }),
            Witness::new(Identifier::new("e"), Type::Field),
            Witness::new(Identifier::new("f"), Type::Bool),
        ],
        statements: vec![Statement::Let(Let {
            identifier: Identifier::new("x"),
            r#type: Some(Type::Uint { bitlength: 228 }),
            expression: Expression {
                elements: vec![
                    ExpressionElement::new(
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Integer(
                            IntegerLiteral::Decimal {
                                value: "2".to_string(),
                            },
                        ))),
                        Token::new(
                            Lexeme::Literal(Literal::Integer(IntegerLiteral::Decimal {
                                value: "2".to_string(),
                            })),
                            Location::new(19, 22),
                        ),
                    ),
                    ExpressionElement::new(
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Integer(
                            IntegerLiteral::Decimal {
                                value: "2".to_string(),
                            },
                        ))),
                        Token::new(
                            Lexeme::Literal(Literal::Integer(IntegerLiteral::Decimal {
                                value: "2".to_string(),
                            })),
                            Location::new(19, 26),
                        ),
                    ),
                    ExpressionElement::new(
                        ExpressionObject::Operator(ExpressionOperator::Addition),
                        Token::new(Lexeme::Symbol(Symbol::Plus), Location::new(19, 24)),
                    ),
                ],
            },
            is_mutable: true,
        })],
    };

    assert_eq!(result, expected);
}
