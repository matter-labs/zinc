//!
//! The transpiler.
//!

use parser::ArrayExpression;
use parser::BlockExpression;
use parser::CircuitProgram;
use parser::ConditionalExpression;
use parser::Expression;
use parser::ExpressionObject;
use parser::ExpressionOperand;
use parser::ExpressionOperator;
use parser::Identifier;
use parser::InnerLiteral;
use parser::Literal;
use parser::Statement;
use parser::StructureExpression;
use parser::TupleExpression;
use parser::TypeVariant;

use crate::element::Descriptor;
use crate::element::Element;
use crate::element::PermanentElement;
use crate::element::TemporaryElement;
use crate::element::TypeElement;
use crate::error::Error;
use crate::output::AllocationBooleanOutput;
use crate::output::AllocationNumberConstantOutput;
use crate::output::AllocationNumberIndexOutput;
use crate::output::ArrayOutput;
use crate::output::AttributesOutput;
use crate::output::BlockOutput;
use crate::output::CircuitOutput;
use crate::output::ConditionalOutput;
use crate::output::DebugStatementOutput;
use crate::output::ImportsOutput;
use crate::output::LetStatementOutput;
use crate::output::LoopStatementForOutput;
use crate::output::LoopStatementWhileOutput;
use crate::output::OperatorAdditionOutput;
use crate::output::OperatorAndOutput;
use crate::output::OperatorAssignmentOutput;
use crate::output::OperatorCastingOutput;
use crate::output::OperatorDivisionOutput;
use crate::output::OperatorEqualsOutput;
use crate::output::OperatorGreaterEqualsOutput;
use crate::output::OperatorGreaterOutput;
use crate::output::OperatorLesserEqualsOutput;
use crate::output::OperatorLesserOutput;
use crate::output::OperatorMultiplicationOutput;
use crate::output::OperatorNegationOutput;
use crate::output::OperatorNotEqualsOutput;
use crate::output::OperatorNotOutput;
use crate::output::OperatorOrOutput;
use crate::output::OperatorRemainderOutput;
use crate::output::OperatorSubtractionOutput;
use crate::output::OperatorXorOutput;
use crate::output::RequireStatementOutput;
use crate::output::StructStatementOutput;
use crate::output::StructureOutput;
use crate::output::TupleOutput;
use crate::output::TypeStatementOutput;
use crate::scope::Scope;
use crate::writer::Writer;

pub struct Transpiler {
    writer: Writer,
    scope: Scope,
    rpn_stack: Vec<StackElement>,
    loop_stack: Vec<String>,
    id_sequence: usize,
}

pub enum StackElement {
    NonEvaluated(ExpressionOperand),
    Evaluated(Element),
}

pub enum EvaluationMode {
    Transpiling,
    Direct,
}

impl Default for Transpiler {
    fn default() -> Self {
        Self {
            writer: Default::default(),
            scope: Default::default(),
            rpn_stack: Default::default(),
            loop_stack: Vec::with_capacity(16),
            id_sequence: Default::default(),
        }
    }
}

impl Transpiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transpile(mut self, program: CircuitProgram) -> Result<String, Error> {
        self.writer.write_lines(AttributesOutput::output());
        self.writer.write_lines(ImportsOutput::output());
        let circuit = CircuitOutput::output(program.inputs, program.witnesses);
        self.writer.write_lines(circuit.start);
        self.writer.shift_forward();
        self.writer.shift_forward();
        for statement in program.statements.into_iter() {
            self.transpile_statement(statement)?;
        }
        self.writer.shift_backward();
        self.writer.shift_backward();
        self.writer.write_lines(circuit.end);
        Ok(self.writer.take())
    }

    fn transpile_statement(&mut self, statement: Statement) -> Result<(), Error> {
        log::trace!("Statement              : {}", statement);

        match statement {
            Statement::Empty => {}
            Statement::Require(require) => {
                let expression = self.transpile_expression(require.expression)?;
                self.writer.write_line(RequireStatementOutput::output(
                    expression,
                    require.annotation,
                ));
            }
            Statement::Let(r#let) => {
                let expression = self.transpile_expression(r#let.expression)?;
                let mut type_variant = expression.type_variant();
                if let Some(ref r#type) = r#let.r#type {
                    type_variant = r#type.variant.clone();
                }
                self.scope
                    .declare_variable(
                        r#let.identifier.name.clone(),
                        type_variant,
                        r#let.is_mutable,
                    )
                    .map_err(Error::Scope)?; // TODO
                self.writer.write_line(LetStatementOutput::output(
                    r#let.is_mutable,
                    r#let.identifier,
                    r#let.r#type,
                    expression,
                ));
            }
            Statement::Loop(r#loop) => {
                let output = LoopStatementForOutput::output(
                    r#loop.index_identifier.name.clone(),
                    r#loop.range_start,
                    r#loop.range_end,
                    r#loop.is_range_inclusive,
                );
                self.writer.write_line(output.start);
                self.loop_stack.push(r#loop.index_identifier.name.clone());
                self.writer.shift_forward();
                let index_namespace = self.current_namespace();
                self.scope
                    .declare_variable(
                        r#loop.index_identifier.name.clone(),
                        TypeVariant::new_integer_unsigned(64),
                        false,
                    )
                    .map_err(Error::Scope)?;
                self.writer.write_line(AllocationNumberIndexOutput::output(
                    r#loop.index_identifier.name,
                    index_namespace,
                ));

                if let Some(r#while) = r#loop.while_condition {
                    let while_condition = self.transpile_expression(r#while)?;
                    let output = LoopStatementWhileOutput::output(while_condition);
                    self.writer.write_line(output.start);
                    self.writer.shift_forward();

                    for statement in r#loop.block.statements.into_iter() {
                        self.transpile_statement(statement)?;
                    }
                    if let Some(expression) = r#loop.block.expression {
                        self.transpile_expression(*expression)?;
                    }

                    self.writer.shift_backward();
                    self.writer.write_line(output.end);
                } else {
                    for statement in r#loop.block.statements.into_iter() {
                        self.transpile_statement(statement)?;
                    }
                    if let Some(expression) = r#loop.block.expression {
                        self.transpile_expression(*expression)?;
                    }
                }

                self.loop_stack.pop();
                self.writer.shift_backward();
                self.writer.write_line(output.end);
            }
            Statement::Type(r#type) => {
                self.scope
                    .declare_type(
                        r#type.identifier.name.clone(),
                        r#type.r#type.variant.clone(),
                    )
                    .map_err(Error::Scope)?;
                self.writer.write_line(TypeStatementOutput::output(
                    r#type.identifier,
                    r#type.r#type,
                ));
            }
            Statement::Struct(r#struct) => {
                self.scope
                    .declare_type(
                        r#struct.identifier.name.clone(),
                        TypeVariant::new_structure(
                            r#struct.identifier.name.clone(),
                            r#struct
                                .fields
                                .clone()
                                .into_iter()
                                .map(|(identifier, r#type)| (identifier.name, r#type.variant))
                                .collect(),
                        ),
                    )
                    .map_err(Error::Scope)?;
                self.writer.write_line(StructStatementOutput::output(
                    r#struct.identifier,
                    r#struct.fields,
                ));
            }
            Statement::Debug(debug) => {
                let expression = self.transpile_expression(debug.expression)?;
                self.writer
                    .write_line(DebugStatementOutput::output(expression));
            }
            Statement::Expression(expression) => {
                self.transpile_expression(expression)?;
            }
        }

        Ok(())
    }

    fn transpile_expression(&mut self, expression: Expression) -> Result<Element, Error> {
        log::trace!("Expression             : {}", expression);

        for expression_element in expression.into_iter() {
            match expression_element.object {
                ExpressionObject::Operand(operand) => {
                    self.rpn_stack.push(StackElement::NonEvaluated(operand));
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(EvaluationMode::Direct, EvaluationMode::Transpiling)?;
                    self.writer
                        .write_line(OperatorAssignmentOutput::output(operand_1, operand_2));
                    self.rpn_stack.push(StackElement::Evaluated(Element::Unit));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!("The range inclusive operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorOrOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorXorOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorAndOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorEqualsOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorNotEqualsOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorGreaterEqualsOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorLesserEqualsOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorGreaterOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorLesserOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorAdditionOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorSubtractionOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorMultiplicationOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorDivisionOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand_1.type_variant();
                    self.writer.write_line(OperatorRemainderOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (operand, r#type) = self.get_binary_operands(
                        EvaluationMode::Transpiling,
                        EvaluationMode::Transpiling,
                    )?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = r#type.type_variant();
                    self.writer.write_line(OperatorCastingOutput::output(
                        identifier.clone(),
                        namespace,
                        operand,
                        r#type,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand = self.get_unary_operand(EvaluationMode::Transpiling)?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand.type_variant();
                    self.writer.write_line(OperatorNegationOutput::output(
                        identifier.clone(),
                        namespace,
                        operand,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand = self.get_unary_operand(EvaluationMode::Transpiling)?;

                    let (identifier, namespace) = self.next_id_and_namespace();
                    let type_variant = operand.type_variant();
                    self.writer.write_line(OperatorNotOutput::output(
                        identifier.clone(),
                        namespace,
                        operand,
                    ));

                    self.rpn_stack
                        .push(StackElement::Evaluated(Element::Temporary(
                            TemporaryElement::new(identifier, type_variant),
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(EvaluationMode::Transpiling, EvaluationMode::Direct)?;

                    let operand_1 = match operand_1 {
                        Element::Permanent(mut element) => match operand_2 {
                            Element::ConstantNumeric(index) => {
                                element.push_descriptor(Descriptor::Array(index));
                                Element::Permanent(element)
                            }
                            _ => panic!("Always checked by some branches above"),
                        },
                        _ => panic!("Always checked by some branches above"),
                    };
                    self.rpn_stack.push(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (operand_1, operand_2) = self
                        .get_binary_operands(EvaluationMode::Transpiling, EvaluationMode::Direct)?;

                    let operand_1 = match operand_1 {
                        Element::Permanent(mut element) => match operand_2 {
                            Element::ConstantNumeric(field) => {
                                element.push_descriptor(Descriptor::Tuple(field));
                                Element::Permanent(element)
                            }
                            Element::ConstantString(field) => {
                                element.push_descriptor(Descriptor::Structure(field));
                                Element::Permanent(element)
                            }
                            _ => panic!("Always checked by some branches above"),
                        },
                        _ => panic!("Always checked by some branches above"),
                    };
                    self.rpn_stack.push(StackElement::Evaluated(operand_1));
                }
            }
        }

        self.get_operand(EvaluationMode::Transpiling)
    }

    fn transpile_identifier_expression(
        &mut self,
        identifier: Identifier,
        mode: EvaluationMode,
    ) -> Result<Element, Error> {
        Ok(match mode {
            EvaluationMode::Transpiling => {
                let variable = self
                    .scope
                    .get_variable(&identifier.name, vec![])
                    .map_err(Error::Scope)?;
                Element::Permanent(PermanentElement::new(
                    identifier.name,
                    variable.type_variant,
                    variable.is_mutable,
                ))
            }
            EvaluationMode::Direct => Element::ConstantString(identifier.name),
        })
    }

    fn transpile_literal_expression(
        &mut self,
        literal: Literal,
        mode: EvaluationMode,
    ) -> Result<Element, Error> {
        let element = match literal.data {
            InnerLiteral::Boolean(value) => match mode {
                EvaluationMode::Transpiling => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    self.writer.write_line(AllocationBooleanOutput::output(
                        identifier.clone(),
                        namespace,
                        value.to_string(),
                    ));
                    Element::Temporary(TemporaryElement::new(identifier, TypeVariant::Boolean))
                }
                EvaluationMode::Direct => Element::ConstantBoolean(value.into()),
            },
            InnerLiteral::Integer(value) => match mode {
                EvaluationMode::Transpiling => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (value, bitlength) = semantic::infer_integer_literal(&value).expect("TODO");
                    self.writer
                        .write_line(AllocationNumberConstantOutput::output(
                            identifier.clone(),
                            namespace,
                            value.to_string(),
                        ));
                    Element::Temporary(TemporaryElement::new(
                        identifier,
                        TypeVariant::new_integer_unsigned(bitlength),
                    ))
                }
                EvaluationMode::Direct => Element::ConstantNumeric(value.into()),
            },
            InnerLiteral::String(..) => panic!("String literals cannot be used in expressions"),
        };

        Ok(element)
    }

    fn transpile_block_expression(&mut self, block: BlockExpression) -> Result<Element, Error> {
        log::trace!("Block expression       : {}", block);

        let identifier = self.next_id();
        let output = BlockOutput::output(identifier.clone());

        self.writer.write_line(output.start);
        self.writer.shift_forward();
        for statement in block.statements.into_iter() {
            self.transpile_statement(statement)?;
        }
        let type_variant = if let Some(expression) = block.expression {
            let result = self.transpile_expression(*expression)?;
            self.writer.write_line(result.to_string());
            result.type_variant()
        } else {
            TypeVariant::Unit
        };
        self.writer.shift_backward();
        self.writer.write_line(output.end);

        Ok(Element::Temporary(TemporaryElement::new(
            identifier,
            type_variant,
        )))
    }

    fn transpile_conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<Element, Error> {
        log::trace!("Conditional expression : {}", conditional);

        let (identifier, namespace) = self.next_id_and_namespace();

        let main_result = self.transpile_block_expression(conditional.main_block)?;
        let else_result = if let Some(else_block) = conditional.else_block {
            self.transpile_block_expression(else_block)?
        } else if let Some(else_if_block) = conditional.else_if {
            self.transpile_conditional_expression(*else_if_block)?
        } else {
            Element::Unit
        };

        let type_variant = main_result.type_variant();
        if type_variant != else_result.type_variant() {
            panic!("Conditional type variants do not match");
        }

        let condition = self.transpile_expression(*conditional.condition)?;

        let element = if main_result.type_variant() != TypeVariant::Unit
            && else_result.type_variant() != TypeVariant::Unit
        {
            self.writer.write_line(ConditionalOutput::output(
                identifier.clone(),
                namespace,
                main_result,
                else_result,
                condition,
            ));
            Element::Temporary(TemporaryElement::new(identifier, type_variant))
        } else {
            Element::Unit
        };

        Ok(element)
    }

    fn transpile_array_expression(&mut self, array: ArrayExpression) -> Result<Element, Error> {
        log::trace!("Array expression       : {}", array);

        let mut elements = Vec::with_capacity(array.elements.len());
        let mut type_variant = TypeVariant::Unit;
        for expression in array.elements.into_iter() {
            let element = self.transpile_expression(expression)?;
            type_variant = element.type_variant();
            elements.push(element);
        }
        let identifier = self.next_id();
        let type_variant = TypeVariant::new_array(type_variant, elements.len());
        self.writer
            .write_line(ArrayOutput::output(identifier.clone(), elements));

        Ok(Element::Temporary(TemporaryElement::new(
            identifier,
            type_variant,
        )))
    }

    fn transpile_tuple_expression(&mut self, tuple: TupleExpression) -> Result<Element, Error> {
        log::trace!("Tuple expression       : {}", tuple);

        let mut elements = Vec::with_capacity(tuple.elements.len());
        let mut type_variants = Vec::with_capacity(tuple.elements.len());
        for expression in tuple.elements.into_iter() {
            let element = self.transpile_expression(expression)?;
            type_variants.push(element.type_variant());
            elements.push(element);
        }
        let identifier = self.next_id();
        let type_variant = TypeVariant::new_tuple(type_variants);
        self.writer
            .write_line(TupleOutput::output(identifier.clone(), elements));

        Ok(Element::Temporary(TemporaryElement::new(
            identifier,
            type_variant,
        )))
    }

    fn transpile_structure_expression(
        &mut self,
        structure: StructureExpression,
    ) -> Result<Element, Error> {
        log::trace!("Structure expression       : {}", structure);

        let mut fields = Vec::with_capacity(structure.fields.len());
        for (identifier, expression) in structure.fields.into_iter() {
            fields.push((identifier, self.transpile_expression(expression)?));
        }
        let identifier = self.next_id();
        self.writer.write_line(StructureOutput::output(
            identifier.clone(),
            structure.identifier.name.clone(),
            fields.as_slice(),
        ));

        Ok(Element::Temporary(TemporaryElement::new(
            identifier,
            TypeVariant::new_structure(
                structure.identifier.name,
                fields
                    .into_iter()
                    .map(|(identifier, element)| (identifier.name, element.type_variant()))
                    .collect::<Vec<(String, TypeVariant)>>(),
            ),
        )))
    }

    fn evaluate(
        &mut self,
        operand: ExpressionOperand,
        mode: EvaluationMode,
    ) -> Result<Element, Error> {
        Ok(match operand {
            ExpressionOperand::Unit => Element::Unit,
            ExpressionOperand::Identifier(identifier) => {
                self.transpile_identifier_expression(identifier, mode)?
            }
            ExpressionOperand::Literal(literal) => {
                self.transpile_literal_expression(literal, mode)?
            }
            ExpressionOperand::Block(expression) => self.transpile_block_expression(expression)?,
            ExpressionOperand::Conditional(expression) => {
                self.transpile_conditional_expression(expression)?
            }
            ExpressionOperand::Array(expression) => self.transpile_array_expression(expression)?,
            ExpressionOperand::Tuple(expression) => self.transpile_tuple_expression(expression)?,
            ExpressionOperand::Structure(expression) => {
                self.transpile_structure_expression(expression)?
            }
            ExpressionOperand::Type(r#type) => Element::Type(TypeElement::new(r#type.variant)),
        })
    }

    fn get_unary_operand(&mut self, mode: EvaluationMode) -> Result<Element, Error> {
        self.get_operand(mode)
    }

    fn get_binary_operands(
        &mut self,
        mode_1: EvaluationMode,
        mode_2: EvaluationMode,
    ) -> Result<(Element, Element), Error> {
        let operand_2 = self.get_operand(mode_2)?;
        let operand_1 = self.get_operand(mode_1)?;
        Ok((operand_1, operand_2))
    }

    fn get_operand(&mut self, mode: EvaluationMode) -> Result<Element, Error> {
        Ok(
            match self.rpn_stack.pop().expect("Always contains an element") {
                StackElement::NonEvaluated(operand) => self.evaluate(operand, mode)?,
                StackElement::Evaluated(element) => element,
            },
        )
    }

    fn next_id(&mut self) -> String {
        self.id_sequence += 1;
        format!(r#"temp_{0:06}"#, self.id_sequence)
    }

    fn current_namespace(&mut self) -> String {
        if self.loop_stack.is_empty() {
            format!(r#""temp_{0:06}""#, self.id_sequence)
        } else {
            let indexes = self
                .loop_stack
                .iter()
                .map(|index| format!("{0}_index", index))
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                r#"format!("temp_{0:06}{1}", {2})"#,
                self.id_sequence,
                "_{}".repeat(self.loop_stack.len()),
                indexes
            )
        }
    }

    fn next_id_and_namespace(&mut self) -> (String, String) {
        let id = self.next_id();
        let namespace = self.current_namespace();
        (id, namespace)
    }
}
