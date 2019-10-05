//!
//! The generator.
//!

mod error;
mod writer;

pub use self::error::Error;
pub use self::writer::Error as WriterError;
pub use self::writer::Writer;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::lexical;
use crate::syntax::BlockExpression;
use crate::syntax::CircuitProgram;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::Literal;
use crate::syntax::Statement;

pub struct Generator {
    stack: Vec<ExpressionOperand>,
    writer: Rc<RefCell<Writer>>,
}

impl Generator {
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            stack: Default::default(),
            writer: Rc::new(RefCell::new(Writer::new(path)?)),
        })
    }

    pub fn generate(&mut self, program: CircuitProgram) -> Result<(), Error> {
        self.writer.borrow_mut().write_attributes()?;
        self.writer.borrow_mut().write_imports()?;
        self.writer
            .borrow_mut()
            .write_circuit_declaration(program.inputs.as_slice(), program.witnesses.as_slice())?;

        self.writer.borrow_mut().write_circuit_header()?;
        for input in program.inputs.into_iter() {
            self.writer.borrow_mut().write_allocate_input(input)?;
        }
        for witness in program.witnesses.into_iter() {
            self.writer.borrow_mut().write_allocate_witness(witness)?;
        }
        for statement in program.statements.into_iter() {
            self.statement(statement)?;
        }
        self.writer.borrow_mut().write_circuit_trailer()?;

        Ok(())
    }

    fn statement(&mut self, statement: Statement) -> Result<(), Error> {
        log::trace!("Statement              : {}", statement);

        match statement {
            Statement::Empty => {}
            Statement::Require(require) => {
                let result = self.evaluate(require.expression)?;
                self.writer
                    .borrow_mut()
                    .write_require(result.as_str(), require.id.as_str())?;
            }
            Statement::Let(r#let) => {
                let result = self.evaluate(r#let.expression)?;
                self.writer.borrow_mut().write_let(
                    r#let.is_mutable,
                    r#let.identifier.name.as_str(),
                    result.as_str(),
                )?;
            }
            Statement::Loop(r#loop) => {
                self.writer.borrow_mut().enter_loop(
                    r#loop.index_identifier.name.as_str(),
                    r#loop.range_start.to_string().as_str(),
                    r#loop.range_end.to_string().as_str(),
                )?;
                self.writer
                    .borrow_mut()
                    .write_allocate_number_loop_index(r#loop.index_identifier.name.as_str())?;
                for statement in r#loop.block.statements.into_iter() {
                    self.statement(statement)?;
                }
                if let Some(expression) = r#loop.block.expression {
                    self.evaluate(*expression)?;
                }
                self.writer.borrow_mut().exit_loop()?;
            }
            Statement::Type { .. } => unimplemented!(),
            Statement::Struct { .. } => unimplemented!(),
            Statement::Debug(debug) => {
                let result = self.evaluate(debug.expression)?;
                self.writer.borrow_mut().write_debug(result.as_str())?;
            }
            Statement::Expression(expression) => {
                self.evaluate(expression)?;
            }
        }

        Ok(())
    }

    fn evaluate(&mut self, expression: Expression) -> Result<String, Error> {
        log::trace!("Expression    : {}", expression);

        for expression_element in expression.into_iter() {
            match expression_element.object {
                ExpressionObject::Operand(operand) => self.stack.push(operand),
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.writer
                        .borrow_mut()
                        .write_assignment(operand_1.as_str(), operand_2.as_str())?;

                    self.stack.push(ExpressionOperand::Literal(Literal::new(
                        expression_element.location,
                        lexical::Literal::Unit,
                    )));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!("The range inclusive operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_or(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_xor(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_and(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Equal) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_equals_number(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_not_equals_number(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_greater_equals(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_lesser_equals(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_greater(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_lesser(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_addition(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_subtraction(operand_1.as_str(), operand_2.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_multiplication(operand_1.as_str(), operand_2.as_str())?;

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
                    let (operand_1, _type) = self.get_binary_operands()?;
                    let id = self.writer.borrow_mut().write_casting(operand_1.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.get_unary_operand()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_negation(operand_1.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 = self.get_unary_operand()?;
                    let id = self.writer.borrow_mut().write_not(operand_1.as_str())?;

                    self.stack
                        .push(ExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {
                    panic!("The indexing operator cannot be used in expressions")
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    panic!("The field operator cannot be used in expressions")
                }
            }
        }

        Ok(
            match self.stack.pop().expect("Always contains an element") {
                ExpressionOperand::Identifier(identifier) => identifier.name,
                ExpressionOperand::Literal(literal) => self.literal(literal)?,
                _ => panic!("Always is either an identifier or literal"),
            },
        )
    }

    fn block(&mut self, block: BlockExpression) -> Result<String, Error> {
        log::trace!("Block expression       : {}", block);

        let id = self.writer.borrow_mut().enter_block()?;
        for statement in block.statements.into_iter() {
            self.statement(statement)?;
        }
        let result = if let Some(expression) = block.expression {
            let result = self.evaluate(*expression)?;
            self.writer.borrow_mut().write_identifier(result.as_str())?;
            id
        } else {
            "()".to_owned()
        };
        self.writer.borrow_mut().exit_block()?;
        Ok(result)
    }

    fn conditional(&mut self, conditional: ConditionalExpression) -> Result<String, Error> {
        log::trace!("Conditional expression : {}", conditional);

        let condition_result = self.evaluate(*conditional.condition)?;

        let id = self
            .writer
            .borrow_mut()
            .enter_conditional(&condition_result, true)?;
        for statement in conditional.main_block.statements.into_iter() {
            self.statement(statement)?;
        }
        let main_result = if let Some(expression) = conditional.main_block.expression {
            let result = self.evaluate(*expression)?;
            self.writer.borrow_mut().write_identifier(result.as_str())?;
            id
        } else {
            "()".to_owned()
        };
        self.writer.borrow_mut().exit_conditional()?;

        let else_result = if let Some(else_block) = conditional.else_block {
            let id = self
                .writer
                .borrow_mut()
                .enter_conditional(&condition_result, false)?;
            for statement in else_block.statements.into_iter() {
                self.statement(statement)?;
            }
            let else_result = if let Some(expression) = else_block.expression {
                let result = self.evaluate(*expression)?;
                self.writer.borrow_mut().write_identifier(result.as_str())?;
                id
            } else {
                "()".to_owned()
            };
            self.writer.borrow_mut().exit_conditional()?;
            else_result
        } else if let Some(else_if_block) = conditional.else_if {
            self.conditional(*else_if_block)?
        } else {
            "()".to_owned()
        };

        if main_result.as_str() == "()" || else_result.as_str() == "()" {
            return Ok("()".to_owned());
        }

        let result = self.writer.borrow_mut().write_conditional(
            main_result.as_str(),
            else_result.as_str(),
            condition_result.as_str(),
        )?;
        Ok(result)
    }

    fn literal(&mut self, literal: Literal) -> Result<String, Error> {
        Ok(match literal.data {
            lexical::Literal::Unit => "()".to_owned(),
            lexical::Literal::Boolean(value) => self
                .writer
                .borrow_mut()
                .write_allocate_boolean(value.to_string().as_str())?,
            lexical::Literal::Integer(value) => self
                .writer
                .borrow_mut()
                .write_allocate_number_constant(value.to_string().as_str())?,
            lexical::Literal::String(..) => panic!("String literals cannot be used in expressions"),
        })
    }

    fn get_unary_operand(&mut self) -> Result<String, Error> {
        let operand_1 = match self.stack.pop().expect("Always contains an element") {
            ExpressionOperand::Identifier(identifier) => identifier.name,
            ExpressionOperand::Literal(literal) => self.literal(literal)?,
            ExpressionOperand::Block(block) => self.block(block)?,
            ExpressionOperand::Conditional(conditional) => self.conditional(conditional)?,
            ExpressionOperand::Type(r#type) => r#type.to_string(),
            ExpressionOperand::Array(array) => array.to_string(),
            ExpressionOperand::Tuple(tuple) => tuple.to_string(),
            ExpressionOperand::Structure(structure) => structure.to_string(),
        };

        Ok(operand_1)
    }

    fn get_binary_operands(&mut self) -> Result<(String, String), Error> {
        let operand_2 = self.stack.pop().expect("Always contains an element");
        let operand_1 = self.stack.pop().expect("Always contains an element");

        let operand_1 = match operand_1 {
            ExpressionOperand::Identifier(identifier) => identifier.name,
            ExpressionOperand::Literal(literal) => self.literal(literal)?,
            ExpressionOperand::Block(block) => self.block(block)?,
            ExpressionOperand::Conditional(conditional) => self.conditional(conditional)?,
            ExpressionOperand::Type(r#type) => r#type.to_string(),
            ExpressionOperand::Array(array) => array.to_string(),
            ExpressionOperand::Tuple(tuple) => tuple.to_string(),
            ExpressionOperand::Structure(structure) => structure.to_string(),
        };

        let operand_2 = match operand_2 {
            ExpressionOperand::Identifier(identifier) => identifier.name,
            ExpressionOperand::Literal(literal) => self.literal(literal)?,
            ExpressionOperand::Block(block) => self.block(block)?,
            ExpressionOperand::Conditional(conditional) => self.conditional(conditional)?,
            ExpressionOperand::Type(r#type) => r#type.to_string(),
            ExpressionOperand::Array(array) => array.to_string(),
            ExpressionOperand::Tuple(tuple) => tuple.to_string(),
            ExpressionOperand::Structure(structure) => structure.to_string(),
        };

        Ok((operand_1, operand_2))
    }
}
