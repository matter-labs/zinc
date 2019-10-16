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
use parser::InnerLiteral;
use parser::Literal;
use parser::Statement;
use parser::StructureExpression;
use parser::TupleExpression;

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
use crate::writer::Writer;

pub struct Transpiler {
    writer: Writer,
    rpn_stack: Vec<Element>,
    loop_stack: Vec<String>,
    id_sequence: usize,
}

impl Default for Transpiler {
    fn default() -> Self {
        Self {
            writer: Default::default(),
            rpn_stack: Default::default(),
            loop_stack: Vec::with_capacity(16),
            id_sequence: Default::default(),
        }
    }
}

impl Transpiler {
    pub fn transpile(&mut self, program: CircuitProgram) -> Result<String, Error> {
        self.writer.write_lines(AttributesOutput::output());
        self.writer.write_lines(ImportsOutput::output());
        let circuit = CircuitOutput::output(program.inputs, program.witnesses);
        self.writer.write_lines(circuit.start);
        self.writer.shift_forward();
        self.writer.shift_forward();
        for statement in program.statements.into_iter() {
            self.statement(statement);
        }
        self.writer.shift_backward();
        self.writer.shift_backward();
        self.writer.write_lines(circuit.end);
        Ok(self.writer.get())
    }

    fn statement(&mut self, statement: Statement) {
        log::trace!("Statement              : {}", statement);

        match statement {
            Statement::Empty => {}
            Statement::Require(require) => {
                let expression = self.evaluate(require.expression);
                self.writer.write_line(RequireStatementOutput::output(
                    expression,
                    require.annotation,
                ));
            }
            Statement::Let(r#let) => {
                let expression = self.evaluate(r#let.expression);
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
                self.writer.write_line(AllocationNumberIndexOutput::output(
                    r#loop.index_identifier.name,
                    index_namespace,
                ));

                if let Some(r#while) = r#loop.while_condition {
                    let while_condition = self.evaluate(r#while);
                    let output = LoopStatementWhileOutput::output(while_condition);
                    self.writer.write_line(output.start);
                    self.writer.shift_forward();

                    for statement in r#loop.block.statements.into_iter() {
                        self.statement(statement);
                    }
                    if let Some(expression) = r#loop.block.expression {
                        self.evaluate(*expression);
                    }

                    self.writer.shift_backward();
                    self.writer.write_line(output.end);
                } else {
                    for statement in r#loop.block.statements.into_iter() {
                        self.statement(statement);
                    }
                    if let Some(expression) = r#loop.block.expression {
                        self.evaluate(*expression);
                    }
                }

                self.loop_stack.pop();
                self.writer.shift_backward();
                self.writer.write_line(output.end);
            }
            Statement::Type(r#type) => self.writer.write_line(TypeStatementOutput::output(
                r#type.identifier,
                r#type.r#type,
            )),
            Statement::Struct(r#struct) => self.writer.write_line(StructStatementOutput::output(
                r#struct.identifier,
                r#struct.fields,
            )),
            Statement::Debug(debug) => {
                let expression = self.evaluate(debug.expression);
                self.writer
                    .write_line(DebugStatementOutput::output(expression));
            }
            Statement::Expression(expression) => {
                self.evaluate(expression);
            }
        }
    }

    fn evaluate(&mut self, expression: Expression) -> Element {
        for expression_element in expression.into_iter() {
            match expression_element.object {
                ExpressionObject::Operand(operand) => {
                    self.rpn_stack.push(Element::Operand(operand));
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands(false, true);
                    self.writer
                        .write_line(OperatorAssignmentOutput::output(operand_1, operand_2));
                    self.rpn_stack.push(Element::Unit);
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!("The range inclusive operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorOrOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorXorOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorAndOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorEqualsOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorNotEqualsOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorGreaterEqualsOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorLesserEqualsOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorGreaterOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorLesserOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorAdditionOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorSubtractionOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorMultiplicationOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorDivisionOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands(true, true);
                    self.writer.write_line(OperatorRemainderOutput::output(
                        identifier.clone(),
                        namespace,
                        operand_1,
                        operand_2,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand, r#type) = self.get_binary_operands(true, false);
                    self.writer.write_line(OperatorCastingOutput::output(
                        identifier.clone(),
                        namespace,
                        operand,
                        r#type,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let operand = self.get_unary_operand(true);
                    self.writer.write_line(OperatorNegationOutput::output(
                        identifier.clone(),
                        namespace,
                        operand,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let operand = self.get_unary_operand(true);
                    self.writer.write_line(OperatorNotOutput::output(
                        identifier.clone(),
                        namespace,
                        operand,
                    ));

                    self.rpn_stack
                        .push(Element::Temporary(TemporaryElement::new(identifier)));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {
                    let (operand_1, operand_2) = self.get_binary_operands(false, false);
                    let operand_1 = match operand_1 {
                        Element::Permanent(mut element) => {
                            element.push_descriptor(Descriptor::Index(operand_2.to_string()));
                            Element::Permanent(element)
                        }
                        Element::Temporary(mut element) => {
                            element.push_descriptor(Descriptor::Index(operand_2.to_string()));
                            Element::Temporary(element)
                        }
                        _ => panic!("Always checked by some branches above"),
                    };
                    self.rpn_stack.push(operand_1);
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (operand_1, operand_2) = self.get_binary_operands(false, false);
                    let operand_1 = match operand_1 {
                        Element::Permanent(mut element) => {
                            element.push_descriptor(Descriptor::Field(operand_2.to_string()));
                            Element::Permanent(element)
                        }
                        Element::Temporary(mut element) => {
                            element.push_descriptor(Descriptor::Field(operand_2.to_string()));
                            Element::Temporary(element)
                        }
                        _ => panic!("Always checked by some branches above"),
                    };
                    self.rpn_stack.push(operand_1);
                }
            }
        }

        match self.rpn_stack.pop().expect("Always contains an element") {
            Element::Operand(operand) => match operand {
                ExpressionOperand::Unit => Element::Unit,
                ExpressionOperand::Identifier(identifier) => {
                    Element::Permanent(PermanentElement::new(identifier.name))
                }
                ExpressionOperand::Literal(literal) => self.literal(literal, true),
                ExpressionOperand::Block(expression) => self.block(expression),
                ExpressionOperand::Conditional(expression) => self.conditional(expression),
                ExpressionOperand::Array(expression) => self.array(expression),
                ExpressionOperand::Tuple(expression) => self.tuple(expression),
                ExpressionOperand::Structure(expression) => self.structure(expression),
                ExpressionOperand::Type(r#type) => {
                    Element::Type(TypeElement::new(r#type.variant.to_string()))
                }
            },
            element => element,
        }
    }

    fn literal(&mut self, literal: Literal, allocate: bool) -> Element {
        match literal.data {
            InnerLiteral::Boolean(value) => {
                if allocate {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    self.writer.write_line(AllocationBooleanOutput::output(
                        identifier.clone(),
                        namespace,
                        value.to_string(),
                    ));
                    Element::Temporary(TemporaryElement::new(identifier))
                } else {
                    Element::Constant(value.to_string())
                }
            }
            InnerLiteral::Integer(value) => {
                if allocate {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    self.writer
                        .write_line(AllocationNumberConstantOutput::output(
                            identifier.clone(),
                            namespace,
                            value.to_string(),
                        ));
                    Element::Temporary(TemporaryElement::new(identifier))
                } else {
                    Element::Constant(value.to_string())
                }
            }
            InnerLiteral::String(..) => panic!("String literals cannot be used in expressions"),
        }
    }

    fn block(&mut self, block: BlockExpression) -> Element {
        let identifier = self.next_id();
        let output = BlockOutput::output(identifier.clone());

        self.writer.write_line(output.start);
        self.writer.shift_forward();
        for statement in block.statements.into_iter() {
            self.statement(statement);
        }
        if let Some(expression) = block.expression {
            let result = self.evaluate(*expression);
            self.writer.write_line(result.to_string());
        }
        self.writer.shift_backward();
        self.writer.write_line(output.end);

        Element::Temporary(TemporaryElement::new(identifier))
    }

    fn conditional(&mut self, conditional: ConditionalExpression) -> Element {
        let (identifier, namespace) = self.next_id_and_namespace();

        let main_result = self.block(conditional.main_block);
        let else_result = if let Some(else_block) = conditional.else_block {
            self.block(else_block)
        } else if let Some(else_if_block) = conditional.else_if {
            self.conditional(*else_if_block)
        } else {
            Element::Unit
        };
        let condition = self.evaluate(*conditional.condition);

        if !main_result.is_unit() && !else_result.is_unit() {
            self.writer.write_line(ConditionalOutput::output(
                identifier.clone(),
                namespace,
                main_result,
                else_result,
                condition,
            ));
            Element::Temporary(TemporaryElement::new(identifier))
        } else {
            Element::Unit
        }
    }

    fn array(&mut self, array: ArrayExpression) -> Element {
        let mut elements = Vec::with_capacity(array.elements.len());
        for expression in array.elements.into_iter() {
            elements.push(self.evaluate(expression));
        }
        let identifier = self.next_id();
        self.writer
            .write_line(ArrayOutput::output(identifier.clone(), elements));

        Element::Temporary(TemporaryElement::new(identifier))
    }

    fn tuple(&mut self, tuple: TupleExpression) -> Element {
        let mut elements = Vec::with_capacity(tuple.elements.len());
        for expression in tuple.elements.into_iter() {
            elements.push(self.evaluate(expression));
        }
        let identifier = self.next_id();
        self.writer
            .write_line(TupleOutput::output(identifier.clone(), elements));

        Element::Temporary(TemporaryElement::new(identifier))
    }

    fn structure(&mut self, structure: StructureExpression) -> Element {
        let mut fields = Vec::with_capacity(structure.fields.len());
        for (identifier, expression) in structure.fields.into_iter() {
            fields.push((identifier, self.evaluate(expression)));
        }
        let identifier = self.next_id();
        self.writer.write_line(StructureOutput::output(
            identifier.clone(),
            structure.identifier.name,
            fields,
        ));

        Element::Temporary(TemporaryElement::new(identifier))
    }

    fn get_unary_operand(&mut self, allocate: bool) -> Element {
        match self.rpn_stack.pop().expect("Always contains an element") {
            Element::Operand(operand) => match operand {
                ExpressionOperand::Unit => Element::Unit,
                ExpressionOperand::Identifier(identifier) => {
                    Element::Permanent(PermanentElement::new(identifier.name))
                }
                ExpressionOperand::Literal(literal) => self.literal(literal, allocate),
                ExpressionOperand::Block(expression) => self.block(expression),
                ExpressionOperand::Conditional(expression) => self.conditional(expression),
                ExpressionOperand::Array(expression) => self.array(expression),
                ExpressionOperand::Tuple(expression) => self.tuple(expression),
                ExpressionOperand::Structure(expression) => self.structure(expression),
                ExpressionOperand::Type(r#type) => {
                    Element::Type(TypeElement::new(r#type.variant.to_string()))
                }
            },
            element => element,
        }
    }

    fn get_binary_operands(&mut self, allocate_1: bool, allocate_2: bool) -> (Element, Element) {
        let operand_2 = self.rpn_stack.pop().expect("Always contains an element");
        let operand_1 = self.rpn_stack.pop().expect("Always contains an element");

        let operand_1 = match operand_1 {
            Element::Operand(operand) => match operand {
                ExpressionOperand::Unit => Element::Unit,
                ExpressionOperand::Identifier(identifier) => {
                    Element::Permanent(PermanentElement::new(identifier.name))
                }
                ExpressionOperand::Literal(literal) => self.literal(literal, allocate_1),
                ExpressionOperand::Block(expression) => self.block(expression),
                ExpressionOperand::Conditional(expression) => self.conditional(expression),
                ExpressionOperand::Array(expression) => self.array(expression),
                ExpressionOperand::Tuple(expression) => self.tuple(expression),
                ExpressionOperand::Structure(expression) => self.structure(expression),
                ExpressionOperand::Type(r#type) => {
                    Element::Type(TypeElement::new(r#type.variant.to_string()))
                }
            },
            element => element,
        };

        let operand_2 = match operand_2 {
            Element::Operand(operand) => match operand {
                ExpressionOperand::Unit => Element::Unit,
                ExpressionOperand::Identifier(identifier) => {
                    Element::Permanent(PermanentElement::new(identifier.name))
                }
                ExpressionOperand::Literal(literal) => self.literal(literal, allocate_2),
                ExpressionOperand::Block(expression) => self.block(expression),
                ExpressionOperand::Conditional(expression) => self.conditional(expression),
                ExpressionOperand::Array(expression) => self.array(expression),
                ExpressionOperand::Tuple(expression) => self.tuple(expression),
                ExpressionOperand::Structure(expression) => self.structure(expression),
                ExpressionOperand::Type(r#type) => {
                    Element::Type(TypeElement::new(r#type.variant.to_string()))
                }
            },
            element => element,
        };

        (operand_1, operand_2)
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
