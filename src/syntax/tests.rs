//!
//! The syntax tests.
//!

#![cfg(test)]

use super::*;
use crate::lexical::IntegerLiteral;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;

#[test]
fn success() {
    let code = br#"
inputs {
    a: uint1;
}

witness {
    b: int253;
}

let mut c: uint228 = 2 + 2;
"#;

    let expected: CircuitProgram = CircuitProgram {
        inputs: vec![Input::new(
            Identifier::new(Location::new(3, 5), b"a".to_vec()),
            Type::new(Location::new(3, 8), TypeVariant::Uint { bitlength: 1 }),
        )],
        witnesses: vec![Witness::new(
            Identifier::new(Location::new(7, 5), b"b".to_vec()),
            Type::new(Location::new(7, 8), TypeVariant::Int { bitlength: 253 }),
        )],
        statements: vec![Statement::Let(Let {
            location: Location::new(10, 1),
            identifier: Identifier::new(Location::new(10, 9), b"c".to_vec()),
            r#type: Some(Type::new(
                Location::new(10, 12),
                TypeVariant::Uint { bitlength: 228 },
            )),
            expression: Expression {
                elements: vec![
                    ExpressionElement::new(
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Integer(
                            IntegerLiteral::Decimal {
                                value: b"2".to_vec(),
                            },
                        ))),
                        Token::new(
                            Lexeme::Literal(Literal::Integer(IntegerLiteral::Decimal {
                                value: b"2".to_vec(),
                            })),
                            Location::new(10, 22),
                        ),
                    ),
                    ExpressionElement::new(
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Integer(
                            IntegerLiteral::Decimal {
                                value: b"2".to_vec(),
                            },
                        ))),
                        Token::new(
                            Lexeme::Literal(Literal::Integer(IntegerLiteral::Decimal {
                                value: b"2".to_vec(),
                            })),
                            Location::new(10, 26),
                        ),
                    ),
                    ExpressionElement::new(
                        ExpressionObject::Operator(ExpressionOperator::Addition),
                        Token::new(Lexeme::Symbol(Symbol::Plus), Location::new(10, 24)),
                    ),
                ],
            },
            is_mutable: true,
        })],
    };

    let result: CircuitProgram = parse(TokenStream::new(code.to_vec())).expect("Syntax error");

    assert_eq!(expected, result);
}
