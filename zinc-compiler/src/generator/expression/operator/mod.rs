//!
//! The generator expression operator.
//!

use zinc_types::LibraryFunctionIdentifier;

use crate::generator::expression::operand::place::Place;
use crate::generator::expression::Expression;
use crate::generator::r#type::contract_field::ContractField;
use crate::generator::r#type::Type;
use crate::semantic::element::access::dot::stack_field::StackField as StackFieldAccess;
use crate::semantic::element::access::index::Index as IndexAccess;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The expression operator which is translated to some specific instructions.
///
#[derive(Debug, Clone)]
pub enum Operator {
    /// No operator, which is useful to indicate that nothing must be written to the bytecode.
    None,

    /// The ordinar `==` assignment operator.
    Assignment {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
    },
    /// The shortcut `|=` assignment operator.
    AssignmentBitwiseOr {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `|`.
        operator: Box<Self>,
    },
    /// The shortcut `^=` assignment operator.
    AssignmentBitwiseXor {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `^`.
        operator: Box<Self>,
    },
    /// The shortcut `&=` assignment operator.
    AssignmentBitwiseAnd {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `&`.
        operator: Box<Self>,
    },
    /// The shortcut `<<=` assignment operator.
    AssignmentBitwiseShiftLeft {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `<<`.
        operator: Box<Self>,
    },
    /// The shortcut `>>=` assignment operator.
    AssignmentBitwiseShiftRight {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `>>`.
        operator: Box<Self>,
    },
    /// The shortcut `+=` assignment operator.
    AssignmentAddition {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `+`.
        operator: Box<Self>,
    },
    /// The shortcut `-=` assignment operator.
    AssignmentSubtraction {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `-`.
        operator: Box<Self>,
    },
    /// The shortcut `*=` assignment operator.
    AssignmentMultiplication {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `*`.
        operator: Box<Self>,
    },
    /// The shortcut `/=` assignment operator.
    AssignmentDivision {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `/`.
        operator: Box<Self>,
    },
    /// The shortcut `%=` assignment operator.
    AssignmentRemainder {
        /// The memory place being assigned to.
        place: Place,
        /// The expression assigned to the memory place.
        expression: Expression,
        /// The inner assignment operator, that is, `%`.
        operator: Box<Self>,
    },

    /// The binary `||` OR operator.
    Or,
    /// The binary `||` OR operator short-circuit evaluation start marker.
    OrShortCircuitStart,
    /// The binary `||` OR operator short-circuit evaluation end marker.
    OrShortCircuitEnd,
    /// The binary `^^` XOR operator.
    Xor,
    /// The binary `&&` AND operator.
    And,
    /// The binary `&&` AND operator short-circuit evaluation start marker.
    AndShortCircuitStart,
    /// The binary `&&` AND operator short-circuit evaluation end marker.
    AndShortCircuitEnd,

    /// The binary `==` comparison operator.
    Equals {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `!=` comparison operator.
    NotEquals {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `>=` comparison operator.
    GreaterEquals {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `<=` comparison operator.
    LesserEquals {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `>` comparison operator.
    Greater {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `<` comparison operator.
    Lesser {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },

    /// The binary `|` bitwise OR operator.
    BitwiseOr {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `^` bitwise XOR operator.
    BitwiseXor {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `&` bitwise AND operator.
    BitwiseAnd {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `<<` bitwise shift left operator.
    BitwiseShiftLeft,
    /// The binary `>>` bitwise shift right operator.
    BitwiseShiftRight,

    /// The binary `+` arithmetic addition operator.
    Addition {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `-` arithmetic subtraction operator.
    Subtraction {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `*` arithmetic multiplication operator.
    Multiplication {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `/` arithmetic division operator.
    Division {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },
    /// The binary `%` arithmetic remainder operator.
    Remainder {
        /// The type to cast the first operand into. Present only for integer literals.
        operand_1_inferred_type: Option<Type>,
        /// The type to cast the second operand into. Present only for integer literals.
        operand_2_inferred_type: Option<Type>,
    },

    /// The type casting operator.
    Casting {
        /// The type to cast into.
        r#type: Type,
    },

    /// The unary logical `!` NOT operator.
    Not,

    /// The unary bitwise `~` NOT operator.
    BitwiseNot,

    /// The unary arithmetic `-` negation operator.
    Negation,

    /// The array index operator.
    Index {
        /// The array index expression inside the `[...]` square brackets.
        expression: Expression,
        /// The access data with type sizes and offsets to simplify bytecode generation.
        access: IndexAccess,
    },

    /// The tuple or structure slice operator, which is represented by `.` in the source code.
    Slice {
        /// The access data with type sizes and offsets to simplify bytecode generation.
        access: StackFieldAccess,
    },

    /// The ordinar function call operator.
    Call {
        /// The function unique ID assigned during semantic analysis.
        type_id: usize,
        /// The function arguments size.
        input_size: usize,
    },
    /// The `dbg!(...)` function call operator.
    CallDebug {
        /// The format string with `{}` placeholders.
        format: String,
        /// The debugged argument types.
        argument_types: Vec<Type>,
    },
    /// The `require(...)` function call operator.
    CallRequire {
        /// The optional error description message.
        message: Option<String>,
    },
    /// The `<Contract>::fetch(...)` function call operator.
    CallContractFetch {
        /// The contract storage fields.
        fields: Vec<ContractField>,
    },
    /// The standard library function call.
    CallLibrary {
        /// The unique standard library function identifier.
        identifier: LibraryFunctionIdentifier,
        /// The function arguments size.
        input_size: usize,
        /// The function result type size.
        output_size: usize,
    },
}

impl Operator {
    ///
    /// A shortcut constructor.
    ///
    pub fn equals() -> Self {
        Self::Equals {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn equals_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::Equals {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn not_equals() -> Self {
        Self::NotEquals {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn not_equals_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::NotEquals {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn greater_equals() -> Self {
        Self::GreaterEquals {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn greater_equals_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::GreaterEquals {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn lesser_equals() -> Self {
        Self::LesserEquals {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn lesser_equals_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::LesserEquals {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn greater() -> Self {
        Self::Greater {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn greater_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::Greater {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn lesser() -> Self {
        Self::Lesser {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn lesser_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::Lesser {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn bitwise_or() -> Self {
        Self::BitwiseOr {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn bitwise_or_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::BitwiseOr {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn bitwise_xor() -> Self {
        Self::BitwiseXor {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn bitwise_xor_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::BitwiseXor {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn bitwise_and() -> Self {
        Self::BitwiseAnd {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn bitwise_and_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::BitwiseAnd {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn addition() -> Self {
        Self::Addition {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn addition_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::Addition {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn subtraction() -> Self {
        Self::Subtraction {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn subtraction_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::Subtraction {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn multiplication() -> Self {
        Self::Multiplication {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn multiplication_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::Multiplication {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn division() -> Self {
        Self::Division {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn division_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::Division {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn remainder() -> Self {
        Self::Remainder {
            operand_1_inferred_type: None,
            operand_2_inferred_type: None,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// Sets the integer literal inferred types, if some casting needed to be done in the target code.
    ///
    pub fn remainder_inferred(
        operand_1_inferred_type: Option<SemanticType>,
        operand_2_inferred_type: Option<SemanticType>,
    ) -> Self {
        Self::Remainder {
            operand_1_inferred_type: operand_1_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
            operand_2_inferred_type: operand_2_inferred_type
                .as_ref()
                .and_then(Type::try_from_semantic),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    /// If the `r#type` is not a valid type to cast to, `None` is returned.
    ///
    pub fn try_casting(r#type: &SemanticType) -> Option<Self> {
        Type::try_from_semantic(r#type).map(|r#type| Self::Casting { r#type })
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn casting(r#type: Type) -> Self {
        Self::Casting { r#type }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn index(expression: Expression, access: IndexAccess) -> Self {
        Self::Index { expression, access }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn slice(access: StackFieldAccess) -> Self {
        Self::Slice { access }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn call(type_id: usize, input_size: usize) -> Self {
        Self::Call {
            type_id,
            input_size,
        }
    }

    ///
    /// A shortcut constructor.
    ///
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

    ///
    /// A shortcut constructor.
    ///
    pub fn call_require(message: Option<String>) -> Self {
        Self::CallRequire { message }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn call_contract_fetch(fields: Vec<ContractField>) -> Self {
        Self::CallContractFetch { fields }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn call_library(
        identifier: LibraryFunctionIdentifier,
        input_size: usize,
        output_size: usize,
    ) -> Self {
        Self::CallLibrary {
            identifier,
            input_size,
            output_size,
        }
    }

    ///
    /// Returns the first operand's integer inferred type, if the type was inferred for the literal
    /// to adopt the other operand's fixed type.
    ///
    pub fn operand_1_inferred_type(&self) -> Option<Type> {
        match self.to_owned() {
            Self::Equals {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::NotEquals {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::GreaterEquals {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::LesserEquals {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::Greater {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::Lesser {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,

            Self::BitwiseOr {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::BitwiseXor {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::BitwiseAnd {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,

            Self::Addition {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::Subtraction {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::Multiplication {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::Division {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,
            Self::Remainder {
                operand_1_inferred_type,
                ..
            } => operand_1_inferred_type,

            _ => None,
        }
    }

    ///
    /// Returns the second operand's integer inferred type, if the type was inferred for the literal
    /// to adopt the other operand's fixed type.
    ///
    pub fn operand_2_inferred_type(&self) -> Option<Type> {
        match self.to_owned() {
            Self::Equals {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::NotEquals {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::GreaterEquals {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::LesserEquals {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::Greater {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::Lesser {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,

            Self::BitwiseOr {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::BitwiseXor {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::BitwiseAnd {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,

            Self::Addition {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::Subtraction {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::Multiplication {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::Division {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,
            Self::Remainder {
                operand_2_inferred_type,
                ..
            } => operand_2_inferred_type,

            _ => None,
        }
    }
}
