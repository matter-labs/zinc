//!
//! The transpiler library.
//!

mod element;
mod error;
mod output;
mod writer;

pub use self::element::Descriptor;
pub use self::element::Element;
pub use self::element::PermanentElement;
pub use self::element::TemporaryElement;
pub use self::element::TypeElement;
pub use self::error::Error;
pub use self::output::AllocationBooleanOutput;
pub use self::output::AllocationNumberConstantOutput;
pub use self::output::AllocationNumberIndexOutput;
pub use self::output::ArrayOutput;
pub use self::output::AttributesOutput;
pub use self::output::BlockOutput;
pub use self::output::CircuitOutput;
pub use self::output::ConditionalOutput;
pub use self::output::DebugStatementOutput;
pub use self::output::ImportsOutput;
pub use self::output::InputOutput;
pub use self::output::LetStatementOutput;
pub use self::output::LoopStatementOutput;
pub use self::output::OperatorAdditionOutput;
pub use self::output::OperatorAndOutput;
pub use self::output::OperatorAssignmentOutput;
pub use self::output::OperatorCastingOutput;
pub use self::output::OperatorDivisionOutput;
pub use self::output::OperatorEqualsOutput;
pub use self::output::OperatorGreaterEqualsOutput;
pub use self::output::OperatorGreaterOutput;
pub use self::output::OperatorLesserEqualsOutput;
pub use self::output::OperatorLesserOutput;
pub use self::output::OperatorMultiplicationOutput;
pub use self::output::OperatorNegationOutput;
pub use self::output::OperatorNotEqualsOutput;
pub use self::output::OperatorNotOutput;
pub use self::output::OperatorOrOutput;
pub use self::output::OperatorRemainderOutput;
pub use self::output::OperatorSubtractionOutput;
pub use self::output::OperatorXorOutput;
pub use self::output::RequireStatementOutput;
pub use self::output::StructStatementOutput;
pub use self::output::StructureOutput;
pub use self::output::TupleOutput;
pub use self::output::TypeOutput;
pub use self::output::TypeStatementOutput;
pub use self::output::WitnessOutput;
pub use self::writer::Writer;

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

pub struct Transpiler {
    stack: Vec<Element>,
    writer: Writer,

    id_sequence: usize,
    loop_stack: Vec<String>,
}

impl Default for Transpiler {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            writer: Writer::new(),

            id_sequence: 0,
            loop_stack: Vec::with_capacity(16),
        }
    }
}

impl Transpiler {
    pub fn new() -> Self {
        Default::default()
    }

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
                let while_condition = match r#loop.while_condition {
                    Some(expression) => Some(self.evaluate(expression)),
                    None => None,
                };
                let output = LoopStatementOutput::output(
                    r#loop.index_identifier.name.clone(),
                    r#loop.range_start,
                    r#loop.range_end,
                    r#loop.is_range_inclusive,
                    while_condition,
                );
                self.writer.write_line(output.r#for);
                self.loop_stack.push(r#loop.index_identifier.name.clone());
                self.writer.shift_forward();
                let index_namespace = self.current_namespace();
                self.writer.write_line(AllocationNumberIndexOutput::output(
                    r#loop.index_identifier.name,
                    index_namespace,
                ));
                let has_while = output.r#while.is_some();
                if let Some(r#while) = output.r#while {
                    self.writer.write_line(r#while);
                    self.writer.shift_forward();
                }
                for statement in r#loop.block.statements.into_iter() {
                    self.statement(statement);
                }
                if let Some(expression) = r#loop.block.expression {
                    self.evaluate(*expression);
                }
                if has_while {
                    self.writer.shift_backward();
                    self.writer.write_line(output.end.clone());
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
                    self.stack.push(Element::Operand(operand));
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands(false, true);
                    self.writer
                        .write_line(OperatorAssignmentOutput::output(operand_1, operand_2));
                    self.stack.push(Element::Unit);
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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

                    self.stack
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
                    self.stack.push(operand_1);
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
                    self.stack.push(operand_1);
                }
            }
        }

        match self.stack.pop().expect("Always contains an element") {
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
        match self.stack.pop().expect("Always contains an element") {
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
        let operand_2 = self.stack.pop().expect("Always contains an element");
        let operand_1 = self.stack.pop().expect("Always contains an element");

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
                r#"format!("temp_{0:06}_{{}}", {1})"#,
                self.id_sequence, indexes
            )
        }
    }

    fn next_id_and_namespace(&mut self) -> (String, String) {
        let id = self.next_id();
        let namespace = self.current_namespace();
        (id, namespace)
    }
}
