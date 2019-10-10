//!
//! The transpiler library.
//!

mod error;
mod output;
mod writer;

pub use self::error::Error;
pub use self::output::ArrayOutput;
pub use self::output::DebugStatementOutput;
pub use self::output::IntermediateAdditionOutput;
pub use self::output::IntermediateAndOutput;
pub use self::output::IntermediateAssignmentOutput;
pub use self::output::IntermediateCastingOutput;
pub use self::output::IntermediateDivisionOutput;
pub use self::output::IntermediateEqualsOutput;
pub use self::output::IntermediateGreaterEqualsOutput;
pub use self::output::IntermediateGreaterOutput;
pub use self::output::IntermediateLesserEqualsOutput;
pub use self::output::IntermediateLesserOutput;
pub use self::output::IntermediateMultiplicationOutput;
pub use self::output::IntermediateNegationOutput;
pub use self::output::IntermediateNotEqualsOutput;
pub use self::output::IntermediateNotOutput;
pub use self::output::IntermediateOrOutput;
pub use self::output::IntermediateRemainderOutput;
pub use self::output::IntermediateSubtractionOutput;
pub use self::output::IntermediateXorOutput;
pub use self::output::LetStatementOutput;
pub use self::output::RequireStatementOutput;
pub use self::output::StructStatementOutput;
pub use self::output::StructureOutput;
pub use self::output::TupleOutput;
pub use self::output::TypeOutput;
pub use self::output::TypeStatementOutput;
pub use self::output::VariableOutput;
pub use self::writer::Writer;

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

pub const SIZE_FIELD: usize = 254;

pub struct Transpiler {
    stack: Vec<ExpressionOperand>,
    writer: Writer,

    id_sequence: usize,
    offset: usize,
    loop_stack: Vec<String>,
    conditional_stack: Vec<(String, bool)>,
}

impl Default for Transpiler {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            writer: Writer::new(),

            id_sequence: 0,
            offset: 0,
            loop_stack: Vec::with_capacity(16),
            conditional_stack: Vec::with_capacity(16),
        }
    }
}

pub struct Variable {
    pub name: String,
    pub is_temporary: bool,
}

impl Transpiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transpile(&mut self, program: CircuitProgram) -> Result<String, Error> {
        self.writer.write_attributes();
        self.writer.write_imports();
        self.writer
            .write_circuit_declaration(program.inputs.as_slice(), program.witnesses.as_slice());

        self.writer.write_circuit_header();
        for input in program.inputs.into_iter() {
            self.writer.write_allocate_input(&input.identifier.name);
        }
        for witness in program.witnesses.into_iter() {
            self.writer.write_allocate_witness(&witness.identifier.name);
        }
        for statement in program.statements.into_iter() {
            self.statement(statement);
        }
        self.writer.write_circuit_trailer();

        Ok(self.writer.get())
    }

    fn statement(&mut self, statement: Statement) {
        log::trace!("Statement              : {}", statement);

        match statement {
            Statement::Empty => {}
            Statement::Require(require) => {
                let expression = self.evaluate(require.expression);
                self.writer.write_line(
                    RequireStatementOutput::new(expression.into(), require.annotation).into(),
                );
            }
            Statement::Let(r#let) => {
                let expression = self.evaluate(r#let.expression);
                self.writer.write_line(
                    LetStatementOutput::new(
                        r#let.is_mutable,
                        r#let.identifier,
                        r#let.r#type,
                        expression.into(),
                    )
                    .into(),
                );
            }
            Statement::Loop(r#loop) => {
                self.writer.enter_loop(
                    r#loop.index_identifier.name.as_str(),
                    r#loop.range_start.to_string().as_str(),
                    r#loop.range_end.to_string().as_str(),
                );
                self.writer
                    .write_allocate_number_loop_index(r#loop.index_identifier.name.as_str());
                for statement in r#loop.block.statements.into_iter() {
                    self.statement(statement);
                }
                if let Some(expression) = r#loop.block.expression {
                    self.evaluate(*expression);
                }
                self.writer.exit_loop();
            }
            Statement::Type(r#type) => self
                .writer
                .write_line(TypeStatementOutput::new(r#type.identifier, r#type.r#type).into()),
            Statement::Struct(r#struct) => self.writer.write_line(
                StructStatementOutput::new(r#struct.identifier, r#struct.fields).into(),
            ),
            Statement::Debug(debug) => {
                let expression = self.evaluate(debug.expression);
                self.writer
                    .write_line(DebugStatementOutput::new(expression.into()).into());
            }
            Statement::Expression(expression) => {
                self.evaluate(expression);
            }
        }
    }

    fn evaluate(&mut self, expression: Expression) -> VariableOutput {
        log::trace!("Expression    : {}", expression);

        for expression_element in expression.into_iter() {
            match expression_element.object {
                ExpressionObject::Operand(operand) => self.stack.push(operand),
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer
                        .write_line(IntermediateAssignmentOutput::new(operand_1, operand_2).into());
                    self.stack.push(ExpressionOperand::Unit);
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!("The range inclusive operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateOrOutput::new(identifier, namespace, operand_1, operand_2)
                            .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateXorOutput::new(identifier, namespace, operand_1, operand_2)
                            .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateAndOutput::new(identifier, namespace, operand_1, operand_2)
                            .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateEqualsOutput::new(identifier, namespace, operand_1, operand_2)
                            .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateNotEqualsOutput::new(
                            identifier, namespace, operand_1, operand_2,
                        )
                        .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateGreaterEqualsOutput::new(
                            identifier, namespace, operand_1, operand_2,
                        )
                        .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateLesserEqualsOutput::new(
                            identifier, namespace, operand_1, operand_2,
                        )
                        .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateGreaterOutput::new(identifier, namespace, operand_1, operand_2)
                            .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateLesserOutput::new(identifier, namespace, operand_1, operand_2)
                            .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateAdditionOutput::new(
                            identifier, namespace, operand_1, operand_2,
                        )
                        .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateSubtractionOutput::new(
                            identifier, namespace, operand_1, operand_2,
                        )
                        .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateMultiplicationOutput::new(
                            identifier, namespace, operand_1, operand_2,
                        )
                        .into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let (operand, _type) = self.get_binary_operands();
                    self.writer.write_line(
                        IntermediateCastingOutput::new(identifier, namespace, operand).into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let operand = self.get_unary_operand();
                    self.writer.write_line(
                        IntermediateNegationOutput::new(identifier, namespace, operand).into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let (identifier, namespace) = self.next_id_and_namespace();
                    let operand = self.get_unary_operand();
                    self.writer.write_line(
                        IntermediateNotOutput::new(identifier, namespace, operand).into(),
                    );

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            identifier,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    unimplemented!();
                }
            }
        }

        match self.stack.pop().expect("Always contains an element") {
            ExpressionOperand::Unit => VariableOutput::new("()".to_owned(), true),
            ExpressionOperand::Identifier(identifier) => {
                VariableOutput::new(identifier.name, false)
            }
            ExpressionOperand::Literal(literal) => self.literal(literal),
            ExpressionOperand::Block(expression) => self.block(expression),
            ExpressionOperand::Conditional(expression) => self.conditional(expression),
            ExpressionOperand::Array(expression) => self.array(expression),
            ExpressionOperand::Tuple(expression) => self.tuple(expression),
            ExpressionOperand::Structure(expression) => self.structure(expression),
            ExpressionOperand::Type { .. } => panic!("Type expression cannot be evaluated"),
        }
    }

    fn literal(&mut self, literal: Literal) -> VariableOutput {
        let identifier = match literal.data {
            InnerLiteral::Boolean(value) => self
                .writer
                .write_allocate_boolean(value.to_string().as_str()),
            InnerLiteral::Integer(value) => self
                .writer
                .write_allocate_number_constant(value.to_string().as_str()),
            InnerLiteral::String(..) => panic!("String literals cannot be used in expressions"),
        };
        VariableOutput::new(identifier, true)
    }

    fn block(&mut self, block: BlockExpression) -> VariableOutput {
        log::trace!("Block expression       : {}", block);

        let id = self.writer.enter_block();
        for statement in block.statements.into_iter() {
            self.statement(statement);
        }
        let result = if let Some(expression) = block.expression {
            let result = self.evaluate(*expression);
            self.writer.write_line(result.into());
            id
        } else {
            "()".to_owned()
        };
        self.writer.exit_block();

        VariableOutput::new(result, true)
    }

    fn conditional(&mut self, conditional: ConditionalExpression) -> VariableOutput {
        log::trace!("Conditional expression : {}", conditional);

        let condition_result = self.evaluate(*conditional.condition);

        let id = self.writer.enter_conditional(&condition_result, true);
        for statement in conditional.main_block.statements.into_iter() {
            self.statement(statement);
        }
        let main_result = if let Some(expression) = conditional.main_block.expression {
            let result = self.evaluate(*expression);
            self.writer.write_line(result.into());
            id
        } else {
            "()".to_owned()
        };
        self.writer.exit_conditional();

        let else_result = if let Some(else_block) = conditional.else_block {
            let id = self.writer.enter_conditional(&condition_result, false);
            for statement in else_block.statements.into_iter() {
                self.statement(statement);
            }
            let else_result = if let Some(expression) = else_block.expression {
                let result = self.evaluate(*expression);
                self.writer.write_line(result.into());
                id
            } else {
                "()".to_owned()
            };
            self.writer.exit_conditional();
            else_result
        } else if let Some(else_if_block) = conditional.else_if {
            self.conditional(*else_if_block)
        } else {
            "()".to_owned()
        };

        if main_result.as_str() == "()" || else_result.as_str() == "()" {
            return VariableOutput::new("()".to_owned(), true);
        }

        let result = self.writer.write_conditional(
            main_result.as_str(),
            else_result.as_str(),
            condition_result.into().as_str(),
        );

        VariableOutput::new(result, true)
    }

    fn array(&mut self, array: ArrayExpression) -> VariableOutput {
        let mut elements = Vec::with_capacity(array.elements.len());
        for expression in array.elements.into_iter() {
            elements.push(self.evaluate(expression));
        }
        let (identifier, _namespace) = self.next_id_and_namespace();
        self.writer
            .write_line(ArrayOutput::new(identifier.clone(), elements).into());
        VariableOutput::new(identifier, true)
    }

    fn tuple(&mut self, tuple: TupleExpression) -> VariableOutput {
        let mut elements = Vec::with_capacity(tuple.elements.len());
        for expression in tuple.elements.into_iter() {
            elements.push(self.evaluate(expression));
        }
        let (identifier, _namespace) = self.next_id_and_namespace();
        self.writer
            .write_line(TupleOutput::new(identifier.clone(), elements).into());
        VariableOutput::new(identifier, true)
    }

    fn structure(&mut self, structure: StructureExpression) -> VariableOutput {
        let mut fields = Vec::with_capacity(structure.fields.len());
        for (identifier, expression) in structure.fields.into_iter() {
            fields.push((identifier, self.evaluate(expression)));
        }
        let (identifier, _namespace) = self.next_id_and_namespace();
        self.writer.write_line(
            StructureOutput::new(identifier.clone(), structure.identifier.name, fields).into(),
        );
        VariableOutput::new(identifier, true)
    }

    fn get_unary_operand(&mut self) -> VariableOutput {
        match self.stack.pop().expect("Always contains an element") {
            ExpressionOperand::Unit => VariableOutput::new("()".to_owned(), true),
            ExpressionOperand::Identifier(identifier) => {
                VariableOutput::new(identifier.name, false)
            }
            ExpressionOperand::Literal(literal) => self.literal(literal),
            ExpressionOperand::Block(expression) => self.block(expression),
            ExpressionOperand::Conditional(expression) => self.conditional(expression),
            ExpressionOperand::Array(expression) => self.array(expression),
            ExpressionOperand::Tuple(expression) => self.tuple(expression),
            ExpressionOperand::Structure(expression) => self.structure(expression),
            ExpressionOperand::Type { .. } => panic!("Type expression cannot be evaluated"),
        }
    }

    fn get_binary_operands(&mut self) -> (VariableOutput, VariableOutput) {
        let operand_2 = self.stack.pop().expect("Always contains an element");
        let operand_1 = self.stack.pop().expect("Always contains an element");

        let operand_1 = match operand_1 {
            ExpressionOperand::Unit => VariableOutput::new("()".to_owned(), true),
            ExpressionOperand::Identifier(identifier) => {
                VariableOutput::new(identifier.name, false)
            }
            ExpressionOperand::Literal(literal) => self.literal(literal),
            ExpressionOperand::Block(expression) => self.block(expression),
            ExpressionOperand::Conditional(expression) => self.conditional(expression),
            ExpressionOperand::Array(expression) => self.array(expression),
            ExpressionOperand::Tuple(expression) => self.tuple(expression),
            ExpressionOperand::Structure(expression) => self.structure(expression),
            ExpressionOperand::Type { .. } => panic!("Type expression cannot be evaluated"),
        };

        let operand_2 = match operand_2 {
            ExpressionOperand::Unit => VariableOutput::new("()".to_owned(), true),
            ExpressionOperand::Identifier(identifier) => {
                VariableOutput::new(identifier.name, false)
            }
            ExpressionOperand::Literal(literal) => self.literal(literal),
            ExpressionOperand::Block(expression) => self.block(expression),
            ExpressionOperand::Conditional(expression) => self.conditional(expression),
            ExpressionOperand::Array(expression) => self.array(expression),
            ExpressionOperand::Tuple(expression) => self.tuple(expression),
            ExpressionOperand::Structure(expression) => self.structure(expression),
            ExpressionOperand::Type { .. } => panic!("Type expression cannot be evaluated"),
        };

        (operand_1, operand_2)
    }

    fn next_id_and_namespace(&mut self) -> (String, String) {
        self.id_sequence += 1;
        let id = format!(r#"temp_{0:06}"#, self.id_sequence);
        let namespace = if self.loop_stack.is_empty() {
            format!(r#""temp_{0:06}""#, self.id_sequence)
        } else {
            let indexes = self
                .loop_stack
                .iter()
                .map(|index| format!("{}_index", index))
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                r#"format!("temp_{0:06}_{{}}", {1})"#,
                self.id_sequence, indexes
            )
        };
        (id, namespace)
    }

    fn shift_forward(&mut self) {
        self.offset += 4;
    }

    fn shift_backward(&mut self) {
        self.offset -= 4;
    }
}
