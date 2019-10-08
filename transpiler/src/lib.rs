//!
//! The transpiler library.
//!

mod converter;
mod error;
mod writer;

pub use self::converter::Converter;
pub use self::error::Error;
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
}

impl Default for Transpiler {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            writer: Writer::new(),
        }
    }
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
                let result = self.evaluate(require.expression);
                self.writer
                    .write_require(result.as_str(), require.id.as_str());
            }
            Statement::Let(r#let) => {
                let result = self.evaluate(r#let.expression);
                self.writer.write_let(
                    r#let.is_mutable,
                    r#let.identifier.name.as_str(),
                    result.as_str(),
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
            Statement::Type(r#type) => self.writer.write_type(r#type),
            Statement::Struct(r#struct) => self.writer.write_struct(r#struct),
            Statement::Debug(debug) => {
                let result = self.evaluate(debug.expression);
                self.writer.write_debug(result.as_str());
            }
            Statement::Expression(expression) => {
                self.evaluate(expression);
            }
        }
    }

    fn evaluate(&mut self, expression: Expression) -> String {
        log::trace!("Expression    : {}", expression);

        for expression_element in expression.into_iter() {
            match expression_element.object {
                ExpressionObject::Operand(operand) => self.stack.push(operand),
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    self.writer
                        .write_assignment(operand_1.as_str(), operand_2.as_str());

                    self.stack.push(ExpressionOperand::Unit);
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!("The range inclusive operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self.writer.write_or(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_xor(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_and(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Equal) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_equals_number(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_not_equals_number(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_greater_equals(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_lesser_equals(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_greater(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_lesser(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_addition(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_subtraction(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands();
                    let id = self
                        .writer
                        .write_multiplication(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    unimplemented!();
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (operand_1, _type) = self.get_binary_operands();
                    let id = self.writer.write_casting(operand_1.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.get_unary_operand();
                    let id = self.writer.write_negation(operand_1.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 = self.get_unary_operand();
                    let id = self.writer.write_not(operand_1.as_str());

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
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
            ExpressionOperand::Unit => "()".to_owned(),
            ExpressionOperand::Identifier(identifier) => identifier.name,
            ExpressionOperand::Literal(literal) => self.literal(literal),
            ExpressionOperand::Block(expression) => self.block(expression),
            ExpressionOperand::Conditional(expression) => self.conditional(expression),
            ExpressionOperand::Array(expression) => self.array(expression),
            ExpressionOperand::Tuple(expression) => self.tuple(expression),
            ExpressionOperand::Structure(expression) => self.structure(expression),
            ExpressionOperand::Type { .. } => panic!("Type expression cannot be evaluated"),
        }
    }

    fn literal(&mut self, literal: Literal) -> String {
        match literal.data {
            InnerLiteral::Boolean(value) => self
                .writer
                .write_allocate_boolean(value.to_string().as_str()),
            InnerLiteral::Integer(value) => self
                .writer
                .write_allocate_number_constant(value.to_string().as_str()),
            InnerLiteral::String(..) => panic!("String literals cannot be used in expressions"),
        }
    }

    fn block(&mut self, block: BlockExpression) -> String {
        log::trace!("Block expression       : {}", block);

        let id = self.writer.enter_block();
        for statement in block.statements.into_iter() {
            self.statement(statement);
        }
        let result = if let Some(expression) = block.expression {
            let result = self.evaluate(*expression);
            self.writer.write_identifier(result.as_str());
            id
        } else {
            "()".to_owned()
        };
        self.writer.exit_block();
        result
    }

    fn conditional(&mut self, conditional: ConditionalExpression) -> String {
        log::trace!("Conditional expression : {}", conditional);

        let condition_result = self.evaluate(*conditional.condition);

        let id = self.writer.enter_conditional(&condition_result, true);
        for statement in conditional.main_block.statements.into_iter() {
            self.statement(statement);
        }
        let main_result = if let Some(expression) = conditional.main_block.expression {
            let result = self.evaluate(*expression);
            self.writer.write_identifier(result.as_str());
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
                self.writer.write_identifier(result.as_str());
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
            return "()".to_owned();
        }

        self.writer.write_conditional(
            main_result.as_str(),
            else_result.as_str(),
            condition_result.as_str(),
        )
    }

    fn array(&mut self, array: ArrayExpression) -> String {
        log::trace!("Array expression       : {}", array);

        format!(
            "[{0}]",
            array
                .elements
                .into_iter()
                .map(|element| self.evaluate(element))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn tuple(&mut self, tuple: TupleExpression) -> String {
        log::trace!("Tuple expression       : {}", tuple);

        format!(
            "({0})",
            tuple
                .elements
                .into_iter()
                .map(|element| self.evaluate(element))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn structure(&mut self, structure: StructureExpression) -> String {
        log::trace!("Structure expression   : {}", structure);

        format!(
            "{0} {{ {1} }}",
            structure.identifier,
            structure
                .fields
                .into_iter()
                .map(|(key, expression)| format!("{0}: {1}", key, self.evaluate(expression)))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn get_unary_operand(&mut self) -> String {
        match self.stack.pop().expect("Always contains an element") {
            ExpressionOperand::Unit => "()".to_owned(),
            ExpressionOperand::Identifier(identifier) => identifier.name,
            ExpressionOperand::Literal(literal) => self.literal(literal),
            ExpressionOperand::Block(block) => self.block(block),
            ExpressionOperand::Conditional(conditional) => self.conditional(conditional),
            ExpressionOperand::Type(r#type) => r#type.to_string(),
            ExpressionOperand::Array(array) => array.to_string(),
            ExpressionOperand::Tuple(tuple) => tuple.to_string(),
            ExpressionOperand::Structure(structure) => structure.to_string(),
        }
    }

    fn get_binary_operands(&mut self) -> (String, String) {
        let operand_2 = self.stack.pop().expect("Always contains an element");
        let operand_1 = self.stack.pop().expect("Always contains an element");

        let operand_1 = match operand_1 {
            ExpressionOperand::Unit => "()".to_owned(),
            ExpressionOperand::Identifier(identifier) => identifier.name,
            ExpressionOperand::Literal(literal) => self.literal(literal),
            ExpressionOperand::Block(block) => self.block(block),
            ExpressionOperand::Conditional(conditional) => self.conditional(conditional),
            ExpressionOperand::Type(r#type) => r#type.to_string(),
            ExpressionOperand::Array(array) => array.to_string(),
            ExpressionOperand::Tuple(tuple) => tuple.to_string(),
            ExpressionOperand::Structure(structure) => structure.to_string(),
        };

        let operand_2 = match operand_2 {
            ExpressionOperand::Unit => "()".to_owned(),
            ExpressionOperand::Identifier(identifier) => identifier.name,
            ExpressionOperand::Literal(literal) => self.literal(literal),
            ExpressionOperand::Block(block) => self.block(block),
            ExpressionOperand::Conditional(conditional) => self.conditional(conditional),
            ExpressionOperand::Type(r#type) => r#type.to_string(),
            ExpressionOperand::Array(array) => array.to_string(),
            ExpressionOperand::Tuple(tuple) => tuple.to_string(),
            ExpressionOperand::Structure(structure) => structure.to_string(),
        };

        (operand_1, operand_2)
    }
}
