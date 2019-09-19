//!
//! The interpreter writer.
//!

mod generator;

pub use self::generator::Generator;

use std::cell::RefCell;
use std::rc::Rc;

use crate::executor::Error;
use crate::lexical::Literal;
use crate::syntax::BlockExpression;
use crate::syntax::CircuitProgram;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionObject;
use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;
use crate::syntax::Statement;

pub struct Writer {
    stack: Vec<OperatorExpressionOperand>,
    generator: Rc<RefCell<Generator>>,
}

impl Writer {
    pub fn new(generator: Rc<RefCell<Generator>>) -> Self {
        Self {
            stack: Default::default(),
            generator,
        }
    }

    pub fn translate(&mut self, program: CircuitProgram) -> Result<(), Error> {
        self.generator.borrow_mut().write_imports();
        self.generator
            .borrow_mut()
            .write_circuit_declaration(program.inputs.as_slice(), program.witnesses.as_slice());

        self.generator.borrow_mut().write_circuit_header();
        for input in program.inputs.iter() {
            self.generator.borrow_mut().write_allocate_input(input);
        }
        for witness in program.witnesses.iter() {
            self.generator.borrow_mut().write_allocate_witness(witness);
        }
        for statement in program.statements.into_iter() {
            self.statement(statement)?;
        }
        self.generator.borrow_mut().write_circuit_trailer();

        Ok(())
    }

    fn statement(&mut self, statement: Statement) -> Result<(), Error> {
        log::trace!("Statement              : {}", statement);

        match statement {
            Statement::Debug(debug) => {
                let rvalue = self.evaluate(debug.expression)?;
                self.generator.borrow_mut().write_debug(&rvalue);
            }
            Statement::Let(r#let) => {
                let rvalue = self.evaluate(r#let.expression)?;
                self.generator.borrow_mut().write_let(
                    r#let.is_mutable,
                    r#let.identifier.name(),
                    &rvalue,
                );
            }
            Statement::Require(require) => {
                let lvalue = self.evaluate(require.expression)?;
                self.generator
                    .borrow_mut()
                    .write_require(&lvalue, &require.id);
            }
            Statement::Loop(r#loop) => {
                self.generator.borrow_mut().enter_loop(
                    r#loop.index_identifier.name(),
                    r#loop.range_start.to_string().as_str(),
                    r#loop.range_end.to_string().as_str(),
                );
                self.generator
                    .borrow_mut()
                    .write_allocate_number_loop_index(r#loop.index_identifier.name());
                for statement in r#loop.block.statements {
                    self.statement(statement)?;
                }
                if let Some(expression) = r#loop.block.expression {
                    self.evaluate(*expression)?;
                }
                self.generator.borrow_mut().exit_loop();
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
                    self.generator
                        .borrow_mut()
                        .write_assignment(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Literal(Literal::Void));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions")
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self.generator.borrow_mut().write_or(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_xor(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_and(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_equals(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::NotEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_not_equals(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::GreaterEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_greater_equals(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::LesserEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_lesser_equals(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_greater(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_lesser(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_addition(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_subtraction(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let id = self
                        .generator
                        .borrow_mut()
                        .write_multiplication(&operand_1, &operand_2);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
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
                    let id = self.generator.borrow_mut().write_casting(&operand_1);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Negation) => {
                    let operand_1 = self.get_unary_operand()?;
                    let id = self.generator.borrow_mut().write_negation(&operand_1);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
                    let operand_1 = self.get_unary_operand()?;
                    let id = self.generator.borrow_mut().write_not(&operand_1);

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            id,
                        )));
                }
            }
        }

        Ok(match self.stack.pop().expect("Must contain an element") {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name().to_owned(),
            OperatorExpressionOperand::Literal(literal) => self.literal(literal),
            _ => panic!("The expression result must contain a mere value"),
        })
    }

    fn block(&mut self, block: BlockExpression) -> Result<String, Error> {
        log::trace!("Block expression       : {}", block);

        let id = self.generator.borrow_mut().enter_block();
        for statement in block.statements {
            self.statement(statement)?;
        }
        let result = if let Some(expression) = block.expression {
            let result = self.evaluate(*expression)?;
            self.generator.borrow_mut().write_identifier(&result);
            id
        } else {
            panic!("Voids are not implemented yet");
        };
        self.generator.borrow_mut().exit_block();
        Ok(result)
    }

    fn conditional(&mut self, conditional: ConditionalExpression) -> Result<String, Error> {
        log::trace!("Conditional expression : {}", conditional);

        panic!("Conditionals are not implemented yet");
    }

    fn literal(&mut self, literal: Literal) -> String {
        match literal {
            Literal::Boolean(value) => self
                .generator
                .borrow_mut()
                .write_allocate_boolean(&value.to_string()),
            Literal::Integer(value) => self
                .generator
                .borrow_mut()
                .write_allocate_number_constant(&value.to_string()),
            Literal::Void => "()".to_owned(),
            Literal::String(..) => panic!("String literals cannot be used in expressions"),
        }
    }

    fn get_binary_operands(&mut self) -> Result<(String, String), Error> {
        let operand_2 = self.stack.pop().unwrap();
        let operand_1 = self.stack.pop().unwrap();

        let operand_1 = match operand_1 {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name().to_owned(),
            OperatorExpressionOperand::Literal(literal) => self.literal(literal),
            OperatorExpressionOperand::Block(block) => self.block(block)?,
            OperatorExpressionOperand::Type(r#type) => r#type.to_string(),
            _ => panic!("Conditionals are not implemented yet"),
        };

        let operand_2 = match operand_2 {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name().to_owned(),
            OperatorExpressionOperand::Literal(literal) => self.literal(literal),
            OperatorExpressionOperand::Block(block) => self.block(block)?,
            OperatorExpressionOperand::Type(r#type) => r#type.to_string(),
            _ => panic!("Conditionals are not implemented yet"),
        };

        Ok((operand_1, operand_2))
    }

    fn get_unary_operand(&mut self) -> Result<String, Error> {
        let operand_1 = match self.stack.pop().unwrap() {
            OperatorExpressionOperand::Identifier(ref identifier) => identifier.name().to_owned(),
            OperatorExpressionOperand::Literal(literal) => self.literal(literal),
            OperatorExpressionOperand::Block(block) => self.block(block)?,
            _ => panic!("Conditionals are not implemented yet"),
        };

        Ok(operand_1)
    }
}
