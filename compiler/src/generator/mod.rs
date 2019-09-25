//!
//! The generator.
//!

mod error;
mod writer;

pub use self::error::Error;
pub use self::writer::Writer;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::lexical;
use crate::syntax::BlockExpression;
use crate::syntax::CircuitProgram;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::Literal;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionObject;
use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;
use crate::syntax::Statement;

pub struct Generator {
    stack: Vec<OperatorExpressionOperand>,
    writer: Rc<RefCell<Writer>>,
}

impl Generator {
    pub fn new(path: PathBuf) -> Self {
        Self {
            stack: Default::default(),
            writer: Rc::new(RefCell::new(Writer::new(path))),
        }
    }

    pub fn generate(&mut self, program: CircuitProgram) -> Result<(), Error> {
        self.writer.borrow_mut().write_imports();
        self.writer
            .borrow_mut()
            .write_circuit_declaration(program.inputs.as_slice(), program.witnesses.as_slice());

        self.writer.borrow_mut().write_circuit_header();
        for input in program.inputs.into_iter() {
            self.writer.borrow_mut().write_allocate_input(input);
        }
        for witness in program.witnesses.into_iter() {
            self.writer.borrow_mut().write_allocate_witness(witness);
        }
        for statement in program.statements.into_iter() {
            self.statement(statement)?;
        }
        self.writer.borrow_mut().write_circuit_trailer();

        Ok(())
    }

    fn statement(&mut self, statement: Statement) -> Result<(), Error> {
        log::trace!("Statement              : {}", statement);

        match statement {
            Statement::Debug(debug) => {
                let result = self.evaluate(debug.expression)?;
                self.writer.borrow_mut().write_debug(result.as_str());
            }
            Statement::Let(r#let) => {
                let result = self.evaluate(r#let.expression)?;
                self.writer.borrow_mut().write_let(
                    r#let.is_mutable,
                    r#let.identifier.name.as_str(),
                    result.as_str(),
                );
            }
            Statement::Require(require) => {
                let result = self.evaluate(require.expression)?;
                self.writer
                    .borrow_mut()
                    .write_require(result.as_str(), require.id.as_str());
            }
            Statement::Loop(r#loop) => {
                self.writer.borrow_mut().enter_loop(
                    r#loop.index_identifier.name.as_str(),
                    r#loop.range_start.to_string().as_str(),
                    r#loop.range_end.to_string().as_str(),
                );
                self.writer
                    .borrow_mut()
                    .write_allocate_number_loop_index(r#loop.index_identifier.name.as_str());
                for statement in r#loop.block.statements.into_iter() {
                    self.statement(statement)?;
                }
                if let Some(expression) = r#loop.block.expression {
                    self.evaluate(*expression)?;
                }
                self.writer.borrow_mut().exit_loop();
            }
            Statement::Expression(expression) => {
                self.evaluate(expression)?;
            }
        }

        Ok(())
    }

    fn evaluate(&mut self, expression: Expression) -> Result<String, Error> {
        match expression {
            Expression::Operator(expression) => self.operator(expression),
            Expression::Block(expression) => self.block(expression),
            Expression::Conditional(expression) => self.conditional(expression),
        }
    }

    fn operator(&mut self, expression: OperatorExpression) -> Result<String, Error> {
        log::trace!("Operator expression    : {}", expression);

        for expression_element in expression.into_iter() {
            match expression_element.object {
                OperatorExpressionObject::Operand(operand) => self.stack.push(operand),
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    self.writer
                        .borrow_mut()
                        .write_assignment(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Literal(Literal::new(
                            expression_element.location,
                            lexical::Literal::Void,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_or(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_xor(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_and(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_equals(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::NotEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_not_equals(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::GreaterEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_greater_equals(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::LesserEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_lesser_equals(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_greater(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_lesser(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_addition(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_subtraction(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .writer
                        .borrow_mut()
                        .write_multiplication(operand_1.as_str(), operand_2.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Division) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Remainder) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Casting) => {
                    let (operand_1, _type) = self.get_binary_operands()?;
                    let id = self.writer.borrow_mut().write_casting(operand_1.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Negation) => {
                    let operand_1 = self.get_unary_operand()?;
                    let id = self.writer.borrow_mut().write_negation(operand_1.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
                    let operand_1 = self.get_unary_operand()?;
                    let id = self.writer.borrow_mut().write_not(operand_1.as_str());

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.location,
                            id,
                        )));
                }
            }
        }

        Ok(match self.stack.pop().expect("Must contain an element") {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name,
            OperatorExpressionOperand::Literal(literal) => self.literal(literal),
            _ => panic!("The expression result must be either an identifier or a literal"),
        })
    }

    fn block(&mut self, block: BlockExpression) -> Result<String, Error> {
        log::trace!("Block expression       : {}", block);

        let id = self.writer.borrow_mut().enter_block();
        for statement in block.statements.into_iter() {
            self.statement(statement)?;
        }
        let result = if let Some(expression) = block.expression {
            let result = self.evaluate(*expression)?;
            self.writer.borrow_mut().write_identifier(result.as_str());
            id
        } else {
            "()".to_owned()
        };
        self.writer.borrow_mut().exit_block();
        Ok(result)
    }

    fn conditional(&mut self, conditional: ConditionalExpression) -> Result<String, Error> {
        log::trace!("Conditional expression : {}", conditional);

        let condition_result = self.evaluate(*conditional.condition)?;

        let id = self
            .writer
            .borrow_mut()
            .enter_conditional(&condition_result, true);
        for statement in conditional.main_block.statements.into_iter() {
            self.statement(statement)?;
        }
        let main_result = if let Some(expression) = conditional.main_block.expression {
            let result = self.evaluate(*expression)?;
            self.writer.borrow_mut().write_identifier(result.as_str());
            id
        } else {
            "()".to_owned()
        };
        self.writer.borrow_mut().exit_conditional();

        let else_result = if let Some(else_block) = conditional.else_block {
            let id = self
                .writer
                .borrow_mut()
                .enter_conditional(&condition_result, false);
            for statement in else_block.statements.into_iter() {
                self.statement(statement)?;
            }
            let else_result = if let Some(expression) = else_block.expression {
                let result = self.evaluate(*expression)?;
                self.writer.borrow_mut().write_identifier(result.as_str());
                id
            } else {
                "()".to_owned()
            };
            self.writer.borrow_mut().exit_conditional();
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
        );
        Ok(result)
    }

    fn literal(&mut self, literal: Literal) -> String {
        match literal.data {
            lexical::Literal::Void => "()".to_owned(),
            lexical::Literal::Boolean(value) => self
                .writer
                .borrow_mut()
                .write_allocate_boolean(value.to_string().as_str()),
            lexical::Literal::Integer(value) => self
                .writer
                .borrow_mut()
                .write_allocate_number_constant(value.to_string().as_str()),
            lexical::Literal::String(..) => panic!("String literals cannot be used in expressions"),
        }
    }

    fn get_binary_operands(&mut self) -> Result<(String, String), Error> {
        let operand_2 = self.stack.pop().unwrap();
        let operand_1 = self.stack.pop().unwrap();

        let operand_1 = match operand_1 {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name,
            OperatorExpressionOperand::Literal(literal) => self.literal(literal),
            OperatorExpressionOperand::Block(block) => self.block(block)?,
            OperatorExpressionOperand::Conditional(conditional) => self.conditional(conditional)?,
            OperatorExpressionOperand::Type(r#type) => r#type.to_string(),
        };

        let operand_2 = match operand_2 {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name,
            OperatorExpressionOperand::Literal(literal) => self.literal(literal),
            OperatorExpressionOperand::Block(block) => self.block(block)?,
            OperatorExpressionOperand::Conditional(conditional) => self.conditional(conditional)?,
            OperatorExpressionOperand::Type(r#type) => r#type.to_string(),
        };

        Ok((operand_1, operand_2))
    }

    fn get_unary_operand(&mut self) -> Result<String, Error> {
        let operand_1 = match self.stack.pop().unwrap() {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name,
            OperatorExpressionOperand::Literal(literal) => self.literal(literal),
            OperatorExpressionOperand::Block(block) => self.block(block)?,
            OperatorExpressionOperand::Conditional(conditional) => self.conditional(conditional)?,
            OperatorExpressionOperand::Type(r#type) => r#type.to_string(),
        };

        Ok(operand_1)
    }
}
