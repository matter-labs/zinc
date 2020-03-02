//!
//! The expression semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::Zero;

use zinc_bytecode::data::types::DataType;
use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::scalar::ScalarType;
use zinc_bytecode::Instruction;

use crate::lexical::Location;
use crate::semantic::analyzer::error::Error;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::analyzer::translation_hint::TranslationHint;
use crate::semantic::bytecode::Bytecode;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::path::Path;
use crate::semantic::element::place::Place;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::builtin::Function as BuiltInFunctionType;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::stdlib::Function as StandardLibraryFunctionType;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::array::Array;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::element::value::structure::Structure;
use crate::semantic::element::value::tuple::Tuple;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::item::Variant as ScopeItem;
use crate::semantic::scope::Scope;
use crate::syntax;
use crate::syntax::ArrayExpression;
use crate::syntax::BlockExpression;
use crate::syntax::BooleanLiteral;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionAuxiliary;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchExpression;
use crate::syntax::MatchPatternVariant;
use crate::syntax::MemberInteger;
use crate::syntax::MemberString;
use crate::syntax::StringLiteral;
use crate::syntax::StructureExpression;
use crate::syntax::TupleExpression;

pub struct Analyzer {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    bytecode: Rc<RefCell<Bytecode>>,
    operands: Vec<StackElement>,

    // will be removed when IR is implemented
    is_next_call_builtin: bool,
    // will be removed when IR is implemented
    loads: usize,
    // will be removed when IR is implemented
    pushes: usize,
}

#[derive(Debug, Clone)]
enum StackElement {
    NotEvaluated(ExpressionOperand),
    Evaluated(Element),
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;
    const STACK_OPERAND_INITIAL_CAPACITY: usize = 16;

    pub fn new(scope: Rc<RefCell<Scope>>, bytecode: Rc<RefCell<Bytecode>>) -> Self {
        Self {
            scope_stack: {
                let mut scope_stack = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
                scope_stack.push(scope);
                scope_stack
            },
            bytecode,
            operands: Vec::with_capacity(Self::STACK_OPERAND_INITIAL_CAPACITY),

            is_next_call_builtin: false,
            loads: 0,
            pushes: 0,
        }
    }

    pub fn new_without_bytecode(scope: Rc<RefCell<Scope>>) -> Self {
        Self::new(scope, Rc::new(RefCell::new(Bytecode::default())))
    }

    pub fn expression(
        &mut self,
        expression: Expression,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    self.push_operand(StackElement::NotEvaluated(operand))
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    let place = operand_1
                        .assign(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory(
                            element.location,
                            place.to_string(),
                            item_location,
                        ));
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType(
                            element.location,
                            r#type.to_string(),
                            place.r#type.to_string(),
                        ));
                    }

                    self.bytecode.borrow_mut().push_instruction_store(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentAddition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;

                    let place = operand_1
                        .clone()
                        .assign_add(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory(
                            element.location,
                            place.to_string(),
                            item_location,
                        ));
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType(
                            element.location,
                            r#type.to_string(),
                            place.r#type.to_string(),
                        ));
                    }

                    if place.is_indexed {
                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Swap(zinc_bytecode::Swap),
                            element.location,
                        );
                    }
                    self.bytecode.borrow_mut().push_instruction_load(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Swap(zinc_bytecode::Swap), element.location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Add(zinc_bytecode::Add), element.location);
                    self.bytecode.borrow_mut().push_instruction_store(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentSubtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;

                    let place = operand_1
                        .clone()
                        .assign_subtract(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory(
                            element.location,
                            place.to_string(),
                            item_location,
                        ));
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType(
                            element.location,
                            r#type.to_string(),
                            place.r#type.to_string(),
                        ));
                    }

                    if place.is_indexed {
                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Swap(zinc_bytecode::Swap),
                            element.location,
                        );
                    }
                    self.bytecode.borrow_mut().push_instruction_load(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Swap(zinc_bytecode::Swap), element.location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Sub(zinc_bytecode::Sub), element.location);
                    self.bytecode.borrow_mut().push_instruction_store(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentMultiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;

                    let place = operand_1
                        .clone()
                        .assign_multiply(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory(
                            element.location,
                            place.to_string(),
                            item_location,
                        ));
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType(
                            element.location,
                            r#type.to_string(),
                            place.r#type.to_string(),
                        ));
                    }

                    if place.is_indexed {
                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Swap(zinc_bytecode::Swap),
                            element.location,
                        );
                    }
                    self.bytecode.borrow_mut().push_instruction_load(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Swap(zinc_bytecode::Swap), element.location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Mul(zinc_bytecode::Mul), element.location);
                    self.bytecode.borrow_mut().push_instruction_store(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentDivision) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;

                    let place = operand_1
                        .clone()
                        .assign_divide(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory(
                            element.location,
                            place.to_string(),
                            item_location,
                        ));
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType(
                            element.location,
                            r#type.to_string(),
                            place.r#type.to_string(),
                        ));
                    }

                    if place.is_indexed {
                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Swap(zinc_bytecode::Swap),
                            element.location,
                        );
                    }
                    self.bytecode.borrow_mut().push_instruction_load(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Swap(zinc_bytecode::Swap), element.location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Div(zinc_bytecode::Div), element.location);
                    self.bytecode.borrow_mut().push_instruction_store(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::AssignmentRemainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;

                    let place = operand_1
                        .clone()
                        .assign_remainder(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;

                    if !place.is_mutable {
                        let item_location =
                            Scope::resolve_item(self.scope(), place.identifier.as_str())
                                .map_err(|error| Error::Scope(place.location, error))?
                                .location;
                        return Err(Error::MutatingImmutableMemory(
                            element.location,
                            place.to_string(),
                            item_location,
                        ));
                    }
                    if place.r#type != r#type {
                        return Err(Error::MutatingWithDifferentType(
                            element.location,
                            r#type.to_string(),
                            place.r#type.to_string(),
                        ));
                    }

                    if place.is_indexed {
                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Swap(zinc_bytecode::Swap),
                            element.location,
                        );
                    }
                    self.bytecode.borrow_mut().push_instruction_load(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Swap(zinc_bytecode::Swap), element.location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Rem(zinc_bytecode::Rem), element.location);
                    self.bytecode.borrow_mut().push_instruction_store(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode.borrow_mut().push_instruction(
                        Instruction::Pop(zinc_bytecode::Pop::new(2)),
                        element.location,
                    );

                    let result = operand_1
                        .range_inclusive(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode.borrow_mut().push_instruction(
                        Instruction::Pop(zinc_bytecode::Pop::new(2)),
                        element.location,
                    );

                    let result = operand_1
                        .range(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Or(zinc_bytecode::Or), element.location);

                    let result = operand_1
                        .or(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Xor(zinc_bytecode::Xor), element.location);

                    let result = operand_1
                        .xor(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::And(zinc_bytecode::And), element.location);

                    let result = operand_1
                        .and(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq), element.location);

                    let result = operand_1
                        .equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Ne(zinc_bytecode::Ne), element.location);

                    let result = operand_1
                        .not_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        true,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Ge(zinc_bytecode::Ge), element.location);

                    let result = operand_1
                        .greater_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        true,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Le(zinc_bytecode::Le), element.location);

                    let result = operand_1
                        .lesser_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        true,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Gt(zinc_bytecode::Gt), element.location);

                    let result = operand_1
                        .greater(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        true,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Lt(zinc_bytecode::Lt), element.location);

                    let result = operand_1
                        .lesser(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Add(zinc_bytecode::Add), element.location);

                    let result = operand_1
                        .add(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        true,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Sub(zinc_bytecode::Sub), element.location);

                    let result = operand_1
                        .subtract(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Mul(zinc_bytecode::Mul), element.location);

                    let result = operand_1
                        .multiply(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        true,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Div(zinc_bytecode::Div), element.location);

                    let result = operand_1
                        .divide(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                        true,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Rem(zinc_bytecode::Rem), element.location);

                    let result = operand_1
                        .remainder(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::TypeExpression,
                        false,
                    )?;

                    if let Some((is_signed, bitlength)) = operand_1
                        .cast(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?
                    {
                        let scalar_type = match (is_signed, bitlength) {
                            (false, crate::BITLENGTH_FIELD) => ScalarType::Field,
                            (signed, length) => IntegerType { signed, length }.into(),
                        };
                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Cast(zinc_bytecode::Cast::new(scalar_type)),
                            element.location,
                        );
                    }

                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Neg(zinc_bytecode::Neg), element.location);

                    let result = operand_1
                        .negate()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Not(zinc_bytecode::Not), element.location);

                    let result = operand_1
                        .not()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Index) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                        false,
                    )?;

                    let is_place_indexed = match operand_1 {
                        Element::Place(ref place) => place.is_indexed,
                        _ => false,
                    };

                    let result = operand_1
                        .index(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    match operand_1 {
                        operand @ Element::Place(_) => {
                            if let Element::Constant(Constant::Range(_))
                            | Element::Constant(Constant::RangeInclusive(_)) = operand_2
                            {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::PushConst(zinc_bytecode::PushConst::new(
                                        BigInt::from(result.offset),
                                        ScalarType::Field,
                                    )),
                                    element.location,
                                );
                            } else {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
                                    element.location,
                                );
                            }
                            if !is_place_indexed {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::PushConst(zinc_bytecode::PushConst::new(
                                        BigInt::zero(),
                                        ScalarType::Field,
                                    )),
                                    element.location,
                                );
                            }
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::PushConst(zinc_bytecode::PushConst::new(
                                    BigInt::from(result.element_size),
                                    ScalarType::Field,
                                )),
                                element.location,
                            );
                            if !is_place_indexed {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::Add(zinc_bytecode::Add),
                                    element.location,
                                );
                            }
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::Mul(zinc_bytecode::Mul),
                                element.location,
                            );
                            if is_place_indexed {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::Add(zinc_bytecode::Add),
                                    element.location,
                                );
                            }
                            self.push_operand(StackElement::Evaluated(operand));
                        }
                        Element::Value(_) | Element::Constant(_) => {
                            match operand_2 {
                                Element::Constant(Constant::Range(_))
                                | Element::Constant(Constant::RangeInclusive(_)) => {
                                    self.bytecode.borrow_mut().push_instruction(
                                        Instruction::PushConst(zinc_bytecode::PushConst::new(
                                            BigInt::from(result.offset),
                                            ScalarType::Field,
                                        )),
                                        element.location,
                                    );
                                }
                                _ => {
                                    self.bytecode.borrow_mut().push_instruction(
                                        Instruction::Cast(zinc_bytecode::Cast::new(
                                            ScalarType::Field,
                                        )),
                                        element.location,
                                    );
                                    self.bytecode.borrow_mut().push_instruction(
                                        Instruction::PushConst(zinc_bytecode::PushConst::new(
                                            BigInt::from(result.element_size),
                                            ScalarType::Field,
                                        )),
                                        element.location,
                                    );
                                    self.bytecode.borrow_mut().push_instruction(
                                        Instruction::Mul(zinc_bytecode::Mul),
                                        element.location,
                                    );
                                }
                            }
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::Slice(zinc_bytecode::Slice::new(
                                    result.total_size,
                                    result.element_size,
                                )),
                                element.location,
                            );

                            let value = Value::try_from(result.sliced_type)
                                .map_err(ElementError::Value)
                                .map_err(|error| Error::Element(element.location, error))?;
                            self.push_operand(StackElement::Evaluated(Element::Value(value)));
                        }
                        _ => {}
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::CompoundTypeMember,
                        false,
                    )?;

                    let is_place_indexed = match operand_1 {
                        Element::Place(ref place) => place.is_indexed,
                        _ => false,
                    };

                    let result = operand_1
                        .field(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    match operand_1 {
                        operand @ Element::Place(_) => {
                            if !is_place_indexed {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::PushConst(zinc_bytecode::PushConst::new(
                                        BigInt::zero(),
                                        ScalarType::Field,
                                    )),
                                    element.location,
                                );
                            }
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::PushConst(zinc_bytecode::PushConst::new(
                                    BigInt::from(result.offset),
                                    ScalarType::Field,
                                )),
                                element.location,
                            );
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::Add(zinc_bytecode::Add),
                                element.location,
                            );
                            self.push_operand(StackElement::Evaluated(operand));
                        }
                        Element::Value(_) | Element::Constant(_) => {
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::PushConst(zinc_bytecode::PushConst::new(
                                    BigInt::from(result.offset),
                                    ScalarType::Field,
                                )),
                                element.location,
                            );
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::Slice(zinc_bytecode::Slice::new(
                                    result.total_size,
                                    result.element_size,
                                )),
                                element.location,
                            );

                            let value = Value::try_from(result.sliced_type)
                                .map_err(ElementError::Value)
                                .map_err(|error| Error::Element(element.location, error))?;
                            self.push_operand(StackElement::Evaluated(Element::Value(value)));
                        }
                        _ => {}
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    self.operator_call(element)?
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PathExpression,
                        TranslationHint::CompoundTypeMember,
                        false,
                    )?;

                    operand_1
                        .path(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::CallBuiltIn) => {
                    self.is_next_call_builtin = true;
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceEnd) => {
                    let element = self
                        .evaluate_operand(TranslationHint::ValueExpression)
                        .map(StackElement::Evaluated)?;
                    self.push_operand(element);
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceCopy) => {
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Tee(zinc_bytecode::Tee), element.location);
                }
            }
        }

        self.evaluate_operand(translation_hint)
    }

    pub fn operator_call(&mut self, element: ExpressionElement) -> Result<(), Error> {
        let location = element.location;

        let (operand_1, operand_2) = self.evaluate_binary_operands(
            TranslationHint::TypeExpression,
            TranslationHint::ValueExpression,
            false,
        )?;

        let function = match operand_1 {
            Element::Type(Type::Function(function)) => function,
            Element::Path(path) => match Scope::resolve_path(self.scope(), &path)?.variant {
                ScopeItem::Type(Type::Function(function)) => function,
                item => {
                    return Err(Error::Function(
                        element.location,
                        FunctionError::NonCallable(item.to_string()),
                    ));
                }
            },
            operand => {
                return Err(Error::Function(
                    element.location,
                    FunctionError::NonCallable(operand.to_string()),
                ));
            }
        };

        let argument_elements = match operand_2 {
            Element::ArgumentList(values) => values,
            _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        };

        let return_type = match function {
            FunctionType::UserDefined(function) => {
                if self.is_next_call_builtin {
                    return Err(Error::Function(
                        element.location,
                        FunctionError::BuiltIn(BuiltInFunctionError::Unknown(
                            function.identifier().to_owned(),
                        )),
                    ));
                }

                let function_address = self
                    .bytecode
                    .borrow_mut()
                    .function_address(function.unique_id())
                    .expect(crate::semantic::PANIC_FUNCTION_ADDRESS_ALWAYS_EXISTS);
                let function_input_size = function.input_size();
                let return_type = function
                    .call(argument_elements)
                    .map_err(|error| Error::Function(element.location, error))?;

                self.bytecode.borrow_mut().push_instruction(
                    Instruction::Call(zinc_bytecode::Call::new(
                        function_address,
                        function_input_size,
                    )),
                    element.location,
                );

                return_type
            }
            FunctionType::BuiltInFunction(function) => {
                if !self.is_next_call_builtin {
                    return Err(Error::Function(
                        element.location,
                        FunctionError::BuiltIn(BuiltInFunctionError::SpecifierMissing(
                            function.identifier(),
                        )),
                    ));
                }

                match function {
                    BuiltInFunctionType::Debug(function) => {
                        let (return_type, format, argument_types) = function
                            .call(argument_elements)
                            .map_err(|error| Error::Function(element.location, error))?;

                        let bytecode_input_types: Vec<DataType> = argument_types
                            .into_iter()
                            .map(|r#type| (&r#type).into())
                            .collect();

                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Dbg(zinc_bytecode::Dbg::new(format, bytecode_input_types)),
                            element.location,
                        );

                        return_type
                    }
                    BuiltInFunctionType::Assert(function) => {
                        let (return_type, message) = function
                            .call(argument_elements)
                            .map_err(|error| Error::Function(element.location, error))?;

                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Assert(zinc_bytecode::Assert::new(message)),
                            element.location,
                        );

                        return_type
                    }
                }
            }
            FunctionType::StandardLibrary(function) => {
                if self.is_next_call_builtin {
                    return Err(Error::Function(
                        element.location,
                        FunctionError::BuiltIn(BuiltInFunctionError::Unknown(
                            function.identifier().to_owned(),
                        )),
                    ));
                }

                let builtin_identifier = function.builtin_identifier();

                let mut input_size = 0;
                for element in argument_elements.iter() {
                    input_size += Type::from_element(element, self.scope())?.size();
                }

                let return_type = match function {
                    StandardLibraryFunctionType::CryptoSha256(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::CryptoPedersen(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::CryptoSchnorrVerify(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::ConvertToBits(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::ConvertFromBitsUnsigned(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::ConvertFromBitsSigned(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::ConvertFromBitsField(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::ArrayReverse(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::ArrayTruncate(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                    StandardLibraryFunctionType::ArrayPad(function) => function
                        .call(argument_elements)
                        .map_err(|error| Error::Function(element.location, error))?,
                };

                self.bytecode.borrow_mut().push_instruction(
                    Instruction::CallBuiltin(zinc_bytecode::CallBuiltin::new(
                        builtin_identifier,
                        input_size,
                        return_type.size(),
                    )),
                    element.location,
                );

                return_type
            }
        };

        self.is_next_call_builtin = false;
        self.push_operand(StackElement::Evaluated(Element::Value(
            Value::try_from(return_type)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(location, error))?,
        )));
        Ok(())
    }

    pub fn block_expression(&mut self, block: BlockExpression) -> Result<Element, Error> {
        for statement in block.statements.into_iter() {
            StatementAnalyzer::new(self.scope(), self.bytecode.clone(), HashMap::new())
                .function_local_statement(statement)?;
        }
        match block.expression {
            Some(expression) => self.expression(*expression, TranslationHint::ValueExpression),
            None => Ok(Element::Value(Value::Unit)),
        }
    }

    fn boolean_literal(&mut self, literal: BooleanLiteral) -> Result<Element, Error> {
        let location = literal.location;

        let constant = Constant::from(literal);
        self.bytecode
            .borrow_mut()
            .push_instruction(constant.to_instruction(), location);
        self.pushes += 1;
        Ok(Element::Constant(constant))
    }

    fn integer_literal(&mut self, literal: IntegerLiteral) -> Result<Element, Error> {
        let location = literal.location;

        let integer = IntegerConstant::try_from(&literal).map_err(|error| {
            Error::Element(
                location,
                ElementError::Constant(ConstantError::Integer(error)),
            )
        })?;
        self.bytecode
            .borrow_mut()
            .push_instruction(integer.to_instruction(), location);
        self.pushes += 1;
        Ok(Element::Constant(Constant::Integer(integer)))
    }

    fn string_literal(&mut self, literal: StringLiteral) -> Result<Element, Error> {
        Ok(Element::Constant(Constant::String(literal.data.value)))
    }

    fn member_integer(&mut self, integer: MemberInteger) -> Result<Element, Error> {
        let location = integer.location;
        let integer = IntegerConstant::try_from(&integer.literal)
            .map_err(|error| {
                Error::Element(
                    location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?
            .to_usize()
            .map_err(|error| {
                Error::Element(
                    location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?;
        Ok(Element::MemberInteger(integer))
    }

    fn member_string(&mut self, member_string: MemberString) -> Result<Element, Error> {
        Ok(Element::MemberString(member_string))
    }

    fn identifier(
        &mut self,
        identifier: Identifier,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        let location = identifier.location;

        let path = Path::new(location, identifier.into());
        self.translate_path(&path, translation_hint)
    }

    fn r#type(&mut self, r#type: syntax::Type) -> Result<Element, Error> {
        Type::from_type_variant(&r#type.variant, self.scope()).map(Element::Type)
    }

    fn conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<Element, Error> {
        let location = conditional.location;
        let condition_location = conditional.condition.location;

        let main_expression_location = conditional
            .main_block
            .expression
            .as_ref()
            .map(|expression| expression.location)
            .unwrap_or(conditional.main_block.location);
        let else_expression_location = conditional
            .else_block
            .as_ref()
            .map(|block| {
                block
                    .expression
                    .as_ref()
                    .map(|expression| expression.location)
                    .unwrap_or(block.location)
            })
            .unwrap_or(conditional.location);

        // compile the condition and check if it is boolean
        let condition_result =
            self.expression(*conditional.condition, TranslationHint::ValueExpression)?;
        match Type::from_element(&condition_result, self.scope())? {
            Type::Boolean => {}
            r#type => {
                return Err(Error::ConditionalExpectedBooleanCondition(
                    condition_location,
                    r#type.to_string(),
                ))
            }
        }

        self.bytecode
            .borrow_mut()
            .push_instruction(Instruction::If(zinc_bytecode::If), location);

        self.push_scope();
        let main_result = self.block_expression(conditional.main_block)?;
        let main_type = Type::from_element(&main_result, self.scope())?;
        self.pop_scope();

        let else_type = if let Some(else_block) = conditional.else_block {
            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::Else(zinc_bytecode::Else), else_block.location);

            self.push_scope();
            let else_result = self.block_expression(else_block)?;
            let else_type = Type::from_element(&else_result, self.scope())?;
            self.pop_scope();

            else_type
        } else {
            Type::Unit
        };

        self.bytecode
            .borrow_mut()
            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);

        // check if the two branches return equals types
        if main_type != else_type {
            return Err(Error::ConditionalBranchTypesMismatch(
                main_expression_location,
                main_type.to_string(),
                else_type.to_string(),
                else_expression_location,
            ));
        }

        Ok(main_result)
    }

    fn match_expression(&mut self, r#match: MatchExpression) -> Result<Element, Error> {
        let location = r#match.location;

        let scrutinee_location = r#match.scrutinee.location;
        let scrutinee_result =
            self.expression(r#match.scrutinee, TranslationHint::ValueExpression)?;
        let scrutinee_type = Type::from_element(&scrutinee_result, self.scope())?;
        let scrutinee_size = scrutinee_type.size();
        let scrutinee_address = self
            .bytecode
            .borrow_mut()
            .allocate_data_stack_space(scrutinee_size);
        self.bytecode.borrow_mut().push_instruction_store(
            scrutinee_address,
            scrutinee_size,
            None,
            false,
            scrutinee_location,
        );

        let first_branch_expression_location = r#match.branches[0].1.location;

        let mut is_exhausted = false;
        let mut branch_results = Vec::with_capacity(r#match.branches.len());
        let mut endifs = 0;
        for (index, (pattern, expression)) in r#match.branches.into_iter().enumerate() {
            let pattern_location = pattern.location;
            let expression_location = expression.location;

            if is_exhausted {
                return Err(Error::MatchBranchUnreachable(pattern.location));
            }
            let result = match pattern.variant {
                MatchPatternVariant::BooleanLiteral(boolean) => {
                    let location = boolean.location;

                    let constant = Constant::from(boolean);
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType(
                            pattern_location,
                            scrutinee_type.to_string(),
                            pattern_type.to_string(),
                            scrutinee_location,
                        ));
                    }

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                        endifs += 1;
                    }

                    self.bytecode.borrow_mut().push_instruction_load(
                        scrutinee_address,
                        scrutinee_size,
                        None,
                        false,
                        scrutinee_location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(constant.to_instruction(), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If), location);

                    self.expression(expression, TranslationHint::ValueExpression)?
                }
                MatchPatternVariant::IntegerLiteral(integer) => {
                    let location = integer.location;

                    let constant = IntegerConstant::try_from(&integer).map_err(|error| {
                        Error::Element(
                            location,
                            ElementError::Constant(ConstantError::Integer(error)),
                        )
                    })?;
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType(
                            pattern_location,
                            scrutinee_type.to_string(),
                            pattern_type.to_string(),
                            scrutinee_location,
                        ));
                    }

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                        endifs += 1;
                    }

                    self.bytecode.borrow_mut().push_instruction_load(
                        scrutinee_address,
                        scrutinee_size,
                        None,
                        false,
                        scrutinee_location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(constant.to_instruction(), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If), location);

                    self.expression(expression, TranslationHint::ValueExpression)?
                }
                MatchPatternVariant::Binding(identifier) => {
                    let location = identifier.location;
                    is_exhausted = true;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                    }
                    self.push_scope();
                    Scope::declare_variable(
                        self.scope(),
                        identifier,
                        ScopeVariableItem::new(scrutinee_type.clone(), false, scrutinee_address),
                    )
                    .map_err(|error| Error::Scope(location, error))?;
                    let result = self.expression(expression, TranslationHint::ValueExpression)?;
                    self.pop_scope();

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
                    }

                    result
                }
                MatchPatternVariant::Path(path) => {
                    let location = path.location;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                        endifs += 1;
                    }

                    let path = match self.expression(path, TranslationHint::PathExpression)? {
                        Element::Path(path) => path,
                        element => {
                            return Err(Error::MatchBranchPatternPathExpectedEvaluable(
                                location,
                                element.to_string(),
                            ))
                        }
                    };

                    self.bytecode.borrow_mut().push_instruction_load(
                        scrutinee_address,
                        scrutinee_size,
                        None,
                        false,
                        scrutinee_location,
                    );
                    match Scope::resolve_path(self.scope(), &path)?.variant {
                        ScopeItem::Variable(variable) => {
                            self.bytecode.borrow_mut().push_instruction_load(
                                variable.address,
                                variable.r#type.size(),
                                None,
                                false,
                                location,
                            )
                        }
                        ScopeItem::Static(r#static) => {
                            self.bytecode.borrow_mut().push_instruction_load(
                                r#static.address,
                                r#static.data.r#type().size(),
                                None,
                                true,
                                location,
                            )
                        }
                        ScopeItem::Constant(constant) => self
                            .bytecode
                            .borrow_mut()
                            .push_instruction(constant.to_instruction(), location),
                        item => {
                            return Err(Error::MatchBranchPatternPathExpectedEvaluable(
                                path.location,
                                item.to_string(),
                            ))
                        }
                    }
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If), location);

                    self.expression(expression, TranslationHint::ValueExpression)?
                }
                MatchPatternVariant::Wildcard => {
                    let location = expression.location;
                    is_exhausted = true;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                    }

                    let result = self.expression(expression, TranslationHint::ValueExpression)?;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
                    }

                    result
                }
            };

            let result_type = Type::from_element(&result, self.scope())?;
            if let Some(first_branch_result) = branch_results.get(0) {
                let first_branch_result_type =
                    Type::from_element(first_branch_result, self.scope())?;
                if result_type != first_branch_result_type {
                    return Err(Error::MatchBranchExpressionInvalidType(
                        expression_location,
                        first_branch_result_type.to_string(),
                        result_type.to_string(),
                        first_branch_expression_location,
                    ));
                }
            }

            branch_results.push(result);
        }

        for _ in 0..endifs {
            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        }

        if !is_exhausted {
            return Err(Error::MatchNotExhausted(location));
        }

        Ok(match branch_results.pop() {
            Some(result) => result,
            None => Element::Constant(Constant::Unit),
        })
    }

    fn array_expression(&mut self, array: ArrayExpression) -> Result<Element, Error> {
        let mut result = Array::default();

        for expression in array.elements.into_iter() {
            let location = expression.location;
            match array.size_expression {
                Some(ref size_expression) => {
                    let size_location = size_expression.location;
                    let size = match Self::new_without_bytecode(self.scope())
                        .expression(size_expression.to_owned(), TranslationHint::ValueExpression)?
                    {
                        Element::Constant(Constant::Integer(integer)) => {
                            integer.to_usize().map_err(|error| {
                                Error::Element(
                                    size_location,
                                    ElementError::Constant(ConstantError::Integer(error)),
                                )
                            })?
                        }
                        element => {
                            return Err(Error::ConstantExpressionHasNonConstantElement(
                                size_location,
                                element.to_string(),
                            ))
                        }
                    };

                    for _ in 0..size {
                        let element =
                            self.expression(expression.clone(), TranslationHint::ValueExpression)?;
                        let element_type = Type::from_element(&element, self.scope())?;
                        result.push(element_type).map_err(|error| {
                            Error::Element(location, ElementError::Value(ValueError::Array(error)))
                        })?;
                    }
                    break;
                }
                None => {
                    let element = self.expression(expression, TranslationHint::ValueExpression)?;
                    let element_type = Type::from_element(&element, self.scope())?;
                    result.push(element_type).map_err(|error| {
                        Error::Element(location, ElementError::Value(ValueError::Array(error)))
                    })?;
                }
            }
        }

        Ok(Element::Value(Value::Array(result)))
    }

    fn tuple_expression(&mut self, tuple: TupleExpression) -> Result<Element, Error> {
        let mut result = Tuple::default();
        for expression in tuple.elements.into_iter() {
            let element = self.expression(expression, TranslationHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;
            result.push(element_type);
        }

        Ok(Element::Value(Value::Tuple(result)))
    }

    fn structure_expression(&mut self, structure: StructureExpression) -> Result<Element, Error> {
        let identifier_location = structure.identifier.location;
        let (structure_identifier, type_unique_id, expected_fields) =
            match Scope::resolve_item(self.scope(), &structure.identifier.name)
                .map_err(|error| Error::Scope(identifier_location, error))?
                .variant
            {
                ScopeItem::Type(Type::Structure(structure)) => {
                    (structure.identifier, structure.unique_id, structure.fields)
                }
                item => {
                    return Err(Error::TypeAliasDoesNotPointToStructure(
                        identifier_location,
                        item.to_string(),
                    ))
                }
            };

        let mut result = Structure::new(
            structure_identifier.clone(),
            type_unique_id,
            Vec::with_capacity(structure.fields.len()),
        );
        for (index, (identifier, expression)) in structure.fields.into_iter().enumerate() {
            let identifier_location = identifier.location;
            let expression_location = expression.location;
            let element = self.expression(expression, TranslationHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;

            if result.contains_key(&identifier.name) {
                return Err(Error::Element(
                    identifier_location,
                    ElementError::Value(ValueError::Structure(
                        StructureValueError::FieldAlreadyExists(
                            identifier.name,
                            structure_identifier,
                        ),
                    )),
                ));
            }

            match expected_fields.get(index) {
                Some((field_name, field_type)) => {
                    if field_type != &element_type {
                        return Err(Error::Element(
                            expression_location,
                            ElementError::Value(ValueError::Structure(
                                StructureValueError::FieldInvalidType(
                                    identifier.name,
                                    structure_identifier,
                                    field_type.to_string(),
                                    element_type.to_string(),
                                ),
                            )),
                        ));
                    }
                    if field_name != &identifier.name {
                        return Err(Error::Element(
                            identifier_location,
                            ElementError::Value(ValueError::Structure(
                                StructureValueError::FieldDoesNotExist(
                                    identifier.name,
                                    structure_identifier,
                                ),
                            )),
                        ));
                    }
                }
                None => {
                    return Err(Error::Element(
                        identifier_location,
                        ElementError::Value(ValueError::Structure(
                            StructureValueError::FieldDoesNotExist(
                                identifier.name,
                                structure_identifier,
                            ),
                        )),
                    ));
                }
            }

            result
                .push(identifier.name.clone(), element_type)
                .map_err(|error| {
                    Error::Element(
                        identifier_location,
                        ElementError::Value(ValueError::Structure(error)),
                    )
                })?;
        }

        Ok(Element::Value(Value::Structure(result)))
    }

    fn list_expression(&mut self, list: Vec<Expression>) -> Result<Element, Error> {
        let mut elements = Vec::with_capacity(list.len());
        for expression in list.into_iter() {
            let element = self.expression(expression, TranslationHint::ValueExpression)?;
            elements.push(element);
        }
        Ok(Element::ArgumentList(elements))
    }

    fn translate_path(
        &mut self,
        path: &Path,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        let location = path.location;
        let path_last = path.last();

        match translation_hint {
            TranslationHint::PlaceExpression => {
                let identifier = path.last().name.to_owned();
                match Scope::resolve_path(self.scope(), path)?.variant {
                    ScopeItem::Variable(variable) => Ok(Element::Place(Place::new(
                        location,
                        identifier,
                        variable.r#type,
                        variable.address,
                        variable.is_mutable,
                        false,
                    ))),
                    ScopeItem::Static(r#static) => Ok(Element::Place(Place::new(
                        location,
                        identifier,
                        r#static.data.r#type(),
                        r#static.address,
                        false,
                        true,
                    ))),
                    ScopeItem::Constant(constant) => Ok(Element::Constant(constant)),
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    ScopeItem::Module(_) => Ok(Element::Module(path_last.name.to_owned())),
                }
            }
            TranslationHint::ValueExpression => {
                match Scope::resolve_path(self.scope(), path)?.variant {
                    ScopeItem::Variable(variable) => {
                        let size = variable.r#type.size();
                        self.bytecode.borrow_mut().push_instruction_load(
                            variable.address,
                            size,
                            None,
                            false,
                            location,
                        );
                        self.loads += 1;
                        Value::try_from(variable.r#type)
                            .map(Element::Value)
                            .map_err(ElementError::Value)
                            .map_err(|error| Error::Element(location, error))
                    }
                    ScopeItem::Constant(constant) => {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(constant.to_instruction(), location);
                        self.pushes += 1;
                        Ok(Element::Constant(constant))
                    }
                    ScopeItem::Static(r#static) => {
                        let r#type = r#static.data.r#type();
                        let size = r#type.size();
                        self.bytecode.borrow_mut().push_instruction_load(
                            r#static.address,
                            size,
                            None,
                            true,
                            location,
                        );
                        self.loads += 1;
                        Ok(Element::Constant(r#static.data))
                    }
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    ScopeItem::Module(_) => Ok(Element::Module(path_last.name.to_owned())),
                }
            }

            TranslationHint::TypeExpression => {
                match Scope::resolve_path(self.scope(), path)?.variant {
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    _ => Ok(Element::Path(path.to_owned())),
                }
            }
            TranslationHint::PathExpression => Ok(Element::Path(path.to_owned())),
            TranslationHint::CompoundTypeMember => Ok(Element::MemberString(MemberString::new(
                location,
                path_last.name.to_owned(),
            ))),
        }
    }

    fn translate_place(
        &mut self,
        place: &Place,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        match translation_hint {
            TranslationHint::ValueExpression => {
                self.bytecode.borrow_mut().push_instruction_load(
                    place.address,
                    place.r#type.size(),
                    if place.is_indexed {
                        Some(place.total_size)
                    } else {
                        None
                    },
                    place.is_global,
                    place.location,
                );
                self.loads += 1;
                Value::try_from(&place.r#type)
                    .map(Element::Value)
                    .map_err(ElementError::Value)
                    .map_err(|error| Error::Element(place.location, error))
            }
            _ => Ok(Element::Place(place.to_owned())),
        }
    }

    fn evaluate_operand(&mut self, translation_hint: TranslationHint) -> Result<Element, Error> {
        match self.pop_operand() {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::Unit => Ok(Element::Constant(Constant::Unit)),
                ExpressionOperand::LiteralBoolean(literal) => self.boolean_literal(literal),
                ExpressionOperand::LiteralInteger(literal) => self.integer_literal(literal),
                ExpressionOperand::LiteralString(literal) => self.string_literal(literal),
                ExpressionOperand::MemberInteger(integer) => self.member_integer(integer),
                ExpressionOperand::MemberString(identifier) => self.member_string(identifier),
                ExpressionOperand::Identifier(identifier) => {
                    self.identifier(identifier, translation_hint)
                }
                ExpressionOperand::ExpressionList(expressions) => self.list_expression(expressions),
                ExpressionOperand::Type(r#type) => self.r#type(r#type),
                ExpressionOperand::Block(expression) => {
                    self.push_scope();
                    let result = self.block_expression(expression)?;
                    self.pop_scope();
                    Ok(result)
                }
                ExpressionOperand::Conditional(expression) => {
                    self.conditional_expression(expression)
                }
                ExpressionOperand::Match(expression) => self.match_expression(expression),
                ExpressionOperand::Array(expression) => self.array_expression(expression),
                ExpressionOperand::Tuple(expression) => self.tuple_expression(expression),
                ExpressionOperand::Structure(expression) => self.structure_expression(expression),
            },
            StackElement::Evaluated(element) => match element {
                Element::Path(path) => self.translate_path(&path, translation_hint),
                Element::Place(place) => self.translate_place(&place, translation_hint),
                element => Ok(element),
            },
        }
    }

    fn evaluate_unary_operand(
        &mut self,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        self.evaluate_operand(translation_hint)
    }

    fn evaluate_binary_operands(
        &mut self,
        translation_hint_1: TranslationHint,
        translation_hint_2: TranslationHint,
        swap_on_single_load: bool,
    ) -> Result<(Element, Element), Error> {
        self.swap_top();

        let loads_before = self.loads;
        let pushes_before = self.pushes;
        let operand_1 = self.evaluate_operand(translation_hint_1)?;
        let added_first = self.pushes - pushes_before == 1 || self.loads - loads_before == 1;

        let loads_before = self.loads;
        let pushes_before = self.pushes;
        let operand_2 = self.evaluate_operand(translation_hint_2)?;
        let added_second = self.pushes - pushes_before == 1 || self.loads - loads_before == 1;

        if swap_on_single_load && added_first && !added_second {
            self.bytecode.borrow_mut().push_instruction(
                Instruction::Swap(zinc_bytecode::Swap::default()),
                Location::default(),
            );
        }

        Ok((operand_1, operand_2))
    }

    fn push_operand(&mut self, operand: StackElement) {
        self.operands.push(operand);
    }

    fn pop_operand(&mut self) -> StackElement {
        self.operands
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND)
    }

    fn swap_top(&mut self) {
        let last_index = self.operands.len() - 1;
        let last_but_one_index = self.operands.len() - 2;
        self.operands.swap(last_index, last_but_one_index)
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }

    fn push_scope(&mut self) {
        self.scope_stack.push(Scope::new_child(self.scope()));
    }

    fn pop_scope(&mut self) {
        self.scope_stack
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE);
    }
}
