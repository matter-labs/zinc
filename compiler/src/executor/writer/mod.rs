//!
//! The interpreter writer.
//!

mod generator;

pub use self::generator::Generator;

use std::cell::RefCell;
use std::rc::Rc;

use crate::executor::Error;
use crate::syntax::BlockExpression;
use crate::syntax::CircuitProgram;
use crate::lexical::Literal;
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
    id_sequence: usize,
}

impl Writer {
    pub fn new(generator: Rc<RefCell<Generator>>) -> Self {
        Self {
            stack: Default::default(),
            generator,
            id_sequence: 0,
        }
    }

    pub fn translate(&mut self, program: CircuitProgram) -> Result<(), Error> {
        self.generator.borrow_mut().write_imports();
        self.generator.borrow_mut().write_circuit_declaration();

        self.generator.borrow_mut().write_synthesize_header();
        for input in program.inputs.iter() {
            self.generator.borrow_mut().write_allocate_input(input);
        }
        self.generator.borrow_mut().write_empty_line();
        for witness in program.witnesses.iter() {
            self.generator
                .borrow_mut()
                .write_allocate_witness(witness);
        }
        self.generator.borrow_mut().write_empty_line();
        for statement in program.statements.into_iter() {
            self.execute(statement)?;
        }
        self.generator.borrow_mut().write_synthesize_trailer();

        Ok(())
    }

    pub fn execute(&mut self, statement: Statement) -> Result<(), Error> {
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
                self.generator.borrow_mut().write_require(&lvalue, &require.id);
            }
            Statement::Loop(r#loop) => {
                self.generator.borrow_mut().write_shifted_line(&format!("for {} in {}..{} {{", r#loop.index_identifier, r#loop.range_start, r#loop.range_end));
                self.generator.borrow_mut().increase_offset();
                let name = self.next_id();
                self
                    .generator
                    .borrow_mut()
                    .write_allocate_number_variable(r#loop.index_identifier.name(), r#loop.index_identifier.name(), &name);
                for statement in r#loop.block.statements {
                    self.execute(statement)?;
                }
                if let Some(expression) = r#loop.block.expression {
                    self.evaluate(*expression)?;
                }
                self.generator.borrow_mut().decrease_offset();
                self.generator.borrow_mut().write_shifted_line("}");
            }
            Statement::Expression(expression) => {
                let lvalue = self.next_id();
                let rvalue = self.evaluate(expression)?;
                self.generator.borrow_mut().write_expression(&lvalue, &rvalue);
            }
        }
        self.generator.borrow_mut().write_empty_line();

        Ok(())
    }

    fn evaluate(&mut self, expression: Expression) -> Result<String, Error> {
        match expression {
            Expression::Operator(expression) => self.evaluate_operator(expression),
            Expression::Block(expression) => self.evaluate_block(expression, "UNUSED".to_owned()),
            Expression::Conditional(expression) => self.evaluate_conditional(expression),
        }
    }

    fn evaluate_operator(&mut self, expression: OperatorExpression) -> Result<String, Error> {
        log::trace!("Operator expression    : {}", expression);

        for expression_element in expression.into_iter() {
            match expression_element.object {
                OperatorExpressionObject::Operand(operand) => self.stack.push(operand),
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_assignment(
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions (yet)")
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_or(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_xor(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_and(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_equals(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::NotEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_not_equals(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::GreaterEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_greater_equals(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::LesserEqual) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_lesser_equals(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_greater(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_lesser(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_addition(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_subtraction(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.get_binary_operands()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_multiplication(
                        &result_lvalue,
                        &operand_1,
                        &operand_2,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Division) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Remainder) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Casting) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Negation) => {
                    let operand_1 = self.get_unary_operand()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_negation(
                        &result_lvalue,
                        &operand_1,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
                    let operand_1 = self.get_unary_operand()?;
                    let result_lvalue = self.next_id();
                    self.generator.borrow_mut().write_not(
                        &result_lvalue,
                        &operand_1,
                    );
                    self.generator.borrow_mut().write_empty_line();

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
            }
        }

        Ok(match self.stack.pop().expect("Must contain an element") {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name().to_owned(),
            OperatorExpressionOperand::Literal(literal) => {
                let operand_1_lvalue = self.next_id();
                match literal {
                    Literal::Boolean(value) => self
                        .generator
                        .borrow_mut()
                        .write_allocate_boolean(&operand_1_lvalue, &value.to_string()),
                    Literal::Integer(value) => self
                        .generator
                        .borrow_mut()
                        .write_allocate_number_constant(&operand_1_lvalue, &value.to_string()),
                    _ => panic!("Invalid literal type"),
                }
                operand_1_lvalue
            },
            _ => panic!("The expression result must be representable"),
        })
    }

    fn evaluate_block(&mut self, block: BlockExpression, name: String) -> Result<String, Error> {
        log::trace!("Block expression       : {}", block);

        self.generator.borrow_mut().increase_offset();
        for statement in block.statements {
            self.execute(statement)?;
        }
        let result = if let Some(expression) = block.expression {
            let result = self.evaluate(*expression)?;
            self.generator.borrow_mut().write_shifted_line(&result);
            name
        } else {
            "VOID".to_owned()
        };
        self.generator.borrow_mut().decrease_offset();
        Ok(result)
    }

    fn evaluate_conditional(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<String, Error> {
        log::trace!("Conditional expression : {}", conditional);

        panic!("Conditionals are not implemented yet");
    }

    fn get_binary_operands(&mut self) -> Result<(String, String), Error> {
        let operand_2 = match self.stack.pop().unwrap() {
            OperatorExpressionOperand::Identifier(identifier) => {
                identifier.name().to_owned()
            },
            OperatorExpressionOperand::Literal(literal) => {
                let operand_1_lvalue = self.next_id();
                match literal {
                    Literal::Boolean(value) => self
                        .generator
                        .borrow_mut()
                        .write_allocate_boolean(&operand_1_lvalue, &value.to_string()),
                    Literal::Integer(value) => self
                        .generator
                        .borrow_mut()
                        .write_allocate_number_constant(&operand_1_lvalue, &value.to_string()),
                    _ => panic!("Invalid literal type"),
                }
                operand_1_lvalue
            },
            OperatorExpressionOperand::Block(expression) => {
                let lvalue = self.next_id();
                self.generator.borrow_mut().write_shifted_line(&format!("let {} = {{", lvalue));
                let result = self.evaluate_block(expression, lvalue)?;
                self.generator.borrow_mut().write_shifted_line("};");
                result
            },
            _ => panic!("Conditionals are not implemented yet"),
        };

        let operand_1 = match self.stack.pop().unwrap() {
            OperatorExpressionOperand::Identifier(identifier) => {
                identifier.name().to_owned()
            },
            OperatorExpressionOperand::Literal(literal) => {
                let operand_1_lvalue = self.next_id();
                match literal {
                    Literal::Boolean(value) => self
                        .generator
                        .borrow_mut()
                        .write_allocate_boolean(&operand_1_lvalue, &value.to_string()),
                    Literal::Integer(value) => self
                        .generator
                        .borrow_mut()
                        .write_allocate_number_constant(&operand_1_lvalue, &value.to_string()),
                    _ => panic!("Invalid literal type"),
                }
                operand_1_lvalue
            },
            OperatorExpressionOperand::Block(expression) => {
                let lvalue = self.next_id();
                self.generator.borrow_mut().write_shifted_line(&format!("let {} = {{", lvalue));
                let result = self.evaluate_block(expression, lvalue)?;
                self.generator.borrow_mut().write_shifted_line("};");
                result
            },
            _ => panic!("Conditionals are not implemented yet"),
        };

        Ok((operand_1, operand_2))
    }

    fn get_unary_operand(&mut self) -> Result<String, Error> {
        let operand_1 = match self.stack.pop().unwrap() {
            OperatorExpressionOperand::Identifier(ref identifier) => {
                identifier.name().to_owned()
            },
            OperatorExpressionOperand::Literal(literal) => {
                let operand_1_lvalue = self.next_id();
                match literal {
                    Literal::Boolean(value) => self
                        .generator
                        .borrow_mut()
                        .write_allocate_boolean(&operand_1_lvalue, &value.to_string()),
                    Literal::Integer(value) => self
                        .generator
                        .borrow_mut()
                        .write_allocate_number_constant(&operand_1_lvalue, &value.to_string()),
                    _ => panic!("Invalid literal type"),
                }
                operand_1_lvalue
            },
            OperatorExpressionOperand::Block(expression) => {
                let lvalue = self.next_id();
                self.generator.borrow_mut().write_shifted_line(&format!("let {} = {{", lvalue));
                let result = self.evaluate_block(expression, lvalue)?;
                self.generator.borrow_mut().write_shifted_line("};");
                result
            },
            _ => panic!("Conditionals are not implemented yet"),
        };

        Ok(operand_1)
    }

    fn next_id(&mut self) -> String {
        self.id_sequence += 1;
        format!("id_{:06}", self.id_sequence)
    }
}
