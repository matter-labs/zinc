//!
//! The generator expression operator.
//!

use zinc_bytecode::builtins::BuiltinIdentifier;
use zinc_bytecode::Instruction;

use crate::generator::r#type::Type;
use crate::semantic::Type as SemanticType;

#[derive(Debug, Clone)]
pub enum Operator {
    // assignment
    Assignment,
    AssignmentBitwiseOr,
    AssignmentBitwiseXor,
    AssignmentBitwiseAnd,
    AssignmentBitwiseShiftLeft,
    AssignmentBitwiseShiftRight,
    AssignmentAddition,
    AssignmentSubtraction,
    AssignmentMultiplication,
    AssignmentDivision,
    AssignmentRemainder,

    // binary logical
    Or,
    Xor,
    And,

    // comparison
    Equals,
    NotEquals,
    GreaterEquals,
    LesserEquals,
    Greater,
    Lesser,

    // binary bitwise
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    BitwiseShiftLeft,
    BitwiseShiftRight,

    // binary arithmetic
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,

    // type casting
    Casting {
        r#type: Type,
    },

    // unary logical
    Not,

    // unary bitwise
    BitwiseNot,

    // unary arithmetic
    Negation,

    // runtime access
    ArrayIndex,
    Slice,

    // call
    Call {
        input_size: usize,
    },
    CallDebug {
        format: String,
        argument_types: Vec<Type>,
    },
    CallAssert {
        message: Option<String>,
    },
    CallStandardLibrary {
        identifier: BuiltinIdentifier,
    },
}

impl Operator {
    pub fn casting(r#type: &SemanticType) -> Option<Self> {
        Type::try_from_semantic(r#type).map(|r#type| Self::Casting { r#type })
    }

    pub fn call(input_size: usize) -> Self {
        Self::Call { input_size }
    }

    pub fn call_debug(format: String, argument_types: Vec<SemanticType>) -> Self {
        Self::CallDebug {
            format,
            argument_types: argument_types
                .as_slice()
                .iter()
                .filter_map(Type::try_from_semantic)
                .collect(),
        }
    }

    pub fn call_assert(message: Option<String>) -> Self {
        Self::CallAssert { message }
    }

    pub fn call_std(identifier: BuiltinIdentifier) -> Self {
        Self::CallStandardLibrary { identifier }
    }

    pub fn into_instruction(self) -> Instruction {
        match self {
            Self::Assignment => todo!(),
            Self::AssignmentBitwiseOr => todo!(),
            Self::AssignmentBitwiseXor => todo!(),
            Self::AssignmentBitwiseAnd => todo!(),
            Self::AssignmentBitwiseShiftLeft => todo!(),
            Self::AssignmentBitwiseShiftRight => todo!(),
            Self::AssignmentAddition => todo!(),
            Self::AssignmentSubtraction => todo!(),
            Self::AssignmentMultiplication => todo!(),
            Self::AssignmentDivision => todo!(),
            Self::AssignmentRemainder => todo!(),

            Self::Or => Instruction::Or(zinc_bytecode::Or),
            Self::Xor => Instruction::Xor(zinc_bytecode::Xor),
            Self::And => Instruction::And(zinc_bytecode::And),

            Self::Equals => Instruction::Eq(zinc_bytecode::Eq),
            Self::NotEquals => Instruction::Ne(zinc_bytecode::Ne),
            Self::GreaterEquals => Instruction::Ge(zinc_bytecode::Ge),
            Self::LesserEquals => Instruction::Le(zinc_bytecode::Le),
            Self::Greater => Instruction::Gt(zinc_bytecode::Gt),
            Self::Lesser => Instruction::Lt(zinc_bytecode::Lt),

            Self::BitwiseOr => Instruction::BitOr(zinc_bytecode::BitOr),
            Self::BitwiseXor => Instruction::BitXor(zinc_bytecode::BitXor),
            Self::BitwiseAnd => Instruction::BitAnd(zinc_bytecode::BitAnd),
            Self::BitwiseShiftLeft => Instruction::BitShiftLeft(zinc_bytecode::BitShiftLeft),
            Self::BitwiseShiftRight => Instruction::BitShiftRight(zinc_bytecode::BitShiftRight),

            Self::Addition => Instruction::Add(zinc_bytecode::Add),
            Self::Subtraction => Instruction::Sub(zinc_bytecode::Sub),
            Self::Multiplication => Instruction::Mul(zinc_bytecode::Mul),
            Self::Division => Instruction::Div(zinc_bytecode::Div),
            Self::Remainder => Instruction::Rem(zinc_bytecode::Rem),

            Self::Casting { r#type } => Instruction::Cast(zinc_bytecode::Cast::new(r#type.into())),

            Self::Not => Instruction::Not(zinc_bytecode::Not),

            Self::BitwiseNot => Instruction::BitNot(zinc_bytecode::BitNot),

            Self::Negation => Instruction::Neg(zinc_bytecode::Neg),

            Self::ArrayIndex => todo!(),
            Self::Slice => todo!(),

            Self::Call { .. } => todo!(),
            Self::CallDebug { .. } => todo!(),
            Self::CallAssert { .. } => todo!(),
            Self::CallStandardLibrary { .. } => todo!(),
        }
    }
}

fn _temp() {
    // if let Element::Constant(Constant::Range(_))
    // | Element::Constant(Constant::RangeInclusive(_)) = operand_2
    // {
    //     self.bytecode.borrow_mut().push_instruction(
    //         Instruction::PushConst(zinc_bytecode::PushConst::new(
    //             BigInt::from(result.offset),
    //             ScalarType::Field,
    //         )),
    //         element.location,
    //     );
    // } else {
    //     self.bytecode.borrow_mut().push_instruction(
    //         Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
    //         element.location,
    //     );
    // }
    // if !is_place_indexed {
    //     self.bytecode.borrow_mut().push_instruction(
    //         Instruction::PushConst(zinc_bytecode::PushConst::new(
    //             BigInt::zero(),
    //             ScalarType::Field,
    //         )),
    //         element.location,
    //     );
    // }
    // self.bytecode.borrow_mut().push_instruction(
    //     Instruction::PushConst(zinc_bytecode::PushConst::new(
    //         BigInt::from(result.element_size),
    //         ScalarType::Field,
    //     )),
    //     element.location,
    // );
    // if !is_place_indexed {
    //     self.bytecode.borrow_mut().push_instruction(
    //         Instruction::Add(zinc_bytecode::Add),
    //         element.location,
    //     );
    // }
    // self.bytecode.borrow_mut().push_instruction(
    //     Instruction::Mul(zinc_bytecode::Mul),
    //     element.location,
    // );
    // if is_place_indexed {
    //     self.bytecode.borrow_mut().push_instruction(
    //         Instruction::Add(zinc_bytecode::Add),
    //         element.location,
    //     );
    // }

    // match operand_2 {
    //     Element::Constant(Constant::Range(_))
    //     | Element::Constant(Constant::RangeInclusive(_)) => {
    //         self.bytecode.borrow_mut().push_instruction(
    //             Instruction::PushConst(zinc_bytecode::PushConst::new(
    //                 BigInt::from(result.offset),
    //                 ScalarType::Field,
    //             )),
    //             element.location,
    //         );
    //     }
    //     _ => {
    //         self.bytecode.borrow_mut().push_instruction(
    //             Instruction::Cast(zinc_bytecode::Cast::new(
    //                 ScalarType::Field,
    //             )),
    //             element.location,
    //         );
    //         self.bytecode.borrow_mut().push_instruction(
    //             Instruction::PushConst(zinc_bytecode::PushConst::new(
    //                 BigInt::from(result.element_size),
    //                 ScalarType::Field,
    //             )),
    //             element.location,
    //         );
    //         self.bytecode.borrow_mut().push_instruction(
    //             Instruction::Mul(zinc_bytecode::Mul),
    //             element.location,
    //         );
    //     }
    // }
    // self.bytecode.borrow_mut().push_instruction(
    //     Instruction::Slice(zinc_bytecode::Slice::new(
    //         result.total_size,
    //         result.element_size,
    //     )),
    //     element.location,
    // );

    // if !is_place_indexed {
    //     self.bytecode.borrow_mut().push_instruction(
    //         Instruction::PushConst(zinc_bytecode::PushConst::new(
    //             BigInt::zero(),
    //             ScalarType::Field,
    //         )),
    //         element.location,
    //     );
    // }
    // self.bytecode.borrow_mut().push_instruction(
    //     Instruction::PushConst(zinc_bytecode::PushConst::new(
    //         BigInt::from(result.offset),
    //         ScalarType::Field,
    //     )),
    //     element.location,
    // );
    // self.bytecode.borrow_mut().push_instruction(
    //     Instruction::Add(zinc_bytecode::Add),
    //     element.location,
    // );

    // self.bytecode.borrow_mut().push_instruction(
    //     Instruction::PushConst(zinc_bytecode::PushConst::new(
    //         BigInt::from(result.offset),
    //         ScalarType::Field,
    //     )),
    //     element.location,
    // );
    // self.bytecode.borrow_mut().push_instruction(
    //     Instruction::Slice(zinc_bytecode::Slice::new(
    //         result.total_size,
    //         result.element_size,
    //     )),
    //     element.location,
    // );
}
