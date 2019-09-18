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
    sequence: usize,
}

impl Writer {
    pub fn new(generator: Rc<RefCell<Generator>>) -> Self {
        Self {
            stack: Default::default(),
            generator,
            sequence: 0,
        }
    }

    pub fn translate(&mut self, program: CircuitProgram) -> Result<(), Error> {
        self.generator.borrow_mut().write_imports();
        self.generator.borrow_mut().write_circuit_declaration();

        self.generator.borrow_mut().write_synthesize_header();
        for input in program.inputs.iter() {
            self.generator.borrow_mut().write_synthesize_input(input);
        }
        self.generator.borrow_mut().write_empty_line();
        for witness in program.witnesses.iter() {
            self.generator
                .borrow_mut()
                .write_synthesize_witness(witness);
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
                let lvalue = format!("expression_{}_result", self.next_sequence());
                let rvalue = self.evaluate(require.expression)?;
                self.generator.borrow_mut().write_expression_result(&lvalue, &rvalue);
                self.generator.borrow_mut().write_require(&lvalue, &require.id);
            }
            Statement::Loop(r#loop) => {
                unimplemented!();
            }
            Statement::Expression(expression) => {
                let lvalue = format!("expression_{}_result", self.next_sequence());
                let rvalue = self.evaluate(expression)?;
                self.generator.borrow_mut().write_expression_result(&lvalue, &rvalue);
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expression: Expression) -> Result<String, Error> {
        match expression {
            Expression::Operator(expression) => self.evaluate_operator(expression),
            Expression::Block(expression) => self.evaluate_block(expression),
            Expression::Conditional(expression) => self.evaluate_conditional(expression),
        }
    }

    fn evaluate_operator(&mut self, expression: OperatorExpression) -> Result<String, Error> {
        log::trace!("Operator expression    : {}", expression);

        for expression_element in expression.into_iter() {
            match expression_element.object {
                OperatorExpressionObject::Operand(operand) => self.stack.push(operand),
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Assignment) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Range) => {
                    panic!("The range operator cannot be used in expressions (yet)")
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Or) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Xor) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::And) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Equal) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::NotEqual) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::GreaterEqual) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::LesserEqual) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Greater) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Lesser) => {
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition) => {
                    let operand_2 = self.stack.pop().unwrap();
                    let operand_2_lvalue = format!(
                        "line_{}_{}_addition_operand_2",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    let operand_2_rvalue = Self::stringify_operand(&operand_2);
                    match &operand_2 {
                        OperatorExpressionOperand::Literal(..) => self
                            .generator
                            .borrow_mut()
                            .write_alloc_literal(&operand_2_lvalue, &operand_2_rvalue),
                        OperatorExpressionOperand::Identifier(..) => self
                            .generator
                            .borrow_mut()
                            .write_reassign_identifier(&operand_2_lvalue, &operand_2_rvalue),
                        _ => unimplemented!(),
                    }

                    let operand_1 = self.stack.pop().unwrap();
                    let operand_1_lvalue = format!(
                        "line_{}_{}_addition_operand_1",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    let operand_1_rvalue = Self::stringify_operand(&operand_1);
                    match &operand_1 {
                        OperatorExpressionOperand::Literal(..) => self
                            .generator
                            .borrow_mut()
                            .write_alloc_literal(&operand_1_lvalue, &operand_1_rvalue),
                        OperatorExpressionOperand::Identifier(..) => self
                            .generator
                            .borrow_mut()
                            .write_reassign_identifier(&operand_1_lvalue, &operand_1_rvalue),
                        _ => unimplemented!(),
                    }

                    let result_lvalue = format!(
                        "line_{}_{}_addition_result",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    self.generator.borrow_mut().write_sum_result(
                        &result_lvalue,
                        &operand_1_lvalue,
                        &operand_2_lvalue,
                    );

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Subtraction) => {
                    let operand_2 = self.stack.pop().unwrap();
                    let operand_2_lvalue = format!(
                        "line_{}_{}_subtraction_operand_2",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    let operand_2_rvalue = Self::stringify_operand(&operand_2);
                    match &operand_2 {
                        OperatorExpressionOperand::Literal(..) => self
                            .generator
                            .borrow_mut()
                            .write_alloc_literal(&operand_2_lvalue, &operand_2_rvalue),
                        OperatorExpressionOperand::Identifier(..) => self
                            .generator
                            .borrow_mut()
                            .write_reassign_identifier(&operand_2_lvalue, &operand_2_rvalue),
                        _ => unimplemented!(),
                    }

                    let operand_1 = self.stack.pop().unwrap();
                    let operand_1_lvalue = format!(
                        "line_{}_{}_subtraction_operand_1",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    let operand_1_rvalue = Self::stringify_operand(&operand_1);
                    match &operand_1 {
                        OperatorExpressionOperand::Literal(..) => self
                            .generator
                            .borrow_mut()
                            .write_alloc_literal(&operand_1_lvalue, &operand_1_rvalue),
                        OperatorExpressionOperand::Identifier(..) => self
                            .generator
                            .borrow_mut()
                            .write_reassign_identifier(&operand_1_lvalue, &operand_1_rvalue),
                        _ => unimplemented!(),
                    }

                    let result_lvalue = format!(
                        "line_{}_{}_subtraction_result",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    self.generator.borrow_mut().write_diff_result(
                        &result_lvalue,
                        &operand_1_lvalue,
                        &operand_2_lvalue,
                    );

                    self.stack
                        .push(OperatorExpressionOperand::Identifier(Identifier::new(
                            expression_element.token.location,
                            result_lvalue,
                        )));
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Multiplication) => {
                    let operand_2 = self.stack.pop().unwrap();
                    let operand_2_lvalue = format!(
                        "line_{}_{}_multiplication_operand_2",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    let operand_2_rvalue = Self::stringify_operand(&operand_2);
                    match &operand_2 {
                        OperatorExpressionOperand::Literal(..) => self
                            .generator
                            .borrow_mut()
                            .write_alloc_literal(&operand_2_lvalue, &operand_2_rvalue),
                        OperatorExpressionOperand::Identifier(..) => self
                            .generator
                            .borrow_mut()
                            .write_reassign_identifier(&operand_2_lvalue, &operand_2_rvalue),
                        _ => unimplemented!(),
                    }

                    let operand_1 = self.stack.pop().unwrap();
                    let operand_1_lvalue = format!(
                        "line_{}_{}_multiplication_operand_1",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    let operand_1_rvalue = Self::stringify_operand(&operand_1);
                    match &operand_1 {
                        OperatorExpressionOperand::Literal(..) => self
                            .generator
                            .borrow_mut()
                            .write_alloc_literal(&operand_1_lvalue, &operand_1_rvalue),
                        OperatorExpressionOperand::Identifier(..) => self
                            .generator
                            .borrow_mut()
                            .write_reassign_identifier(&operand_1_lvalue, &operand_1_rvalue),
                        _ => unimplemented!(),
                    }

                    let result_lvalue = format!(
                        "line_{}_{}_multiplication_result",
                        expression_element.token.location.line(),
                        expression_element.token.location.column()
                    );
                    self.generator.borrow_mut().write_mul_result(
                        &result_lvalue,
                        &operand_1_lvalue,
                        &operand_2_lvalue,
                    );

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
                    unimplemented!();
                }
                OperatorExpressionObject::Operator(OperatorExpressionOperator::Not) => {
                    unimplemented!();
                }
            }
        }

        Ok(Self::stringify_operand(
            &self.stack.pop().expect("Must contain an element"),
        ))
    }

    fn evaluate_block(&mut self, block: BlockExpression) -> Result<String, Error> {
        log::trace!("Block expression       : {}", block);

        unimplemented!();
        //        let mut executor =
        //            Writer::new(Scope::new(Some(self.scope.clone())), self.generator.clone());
        //        for statement in block.statements {
        //            executor.execute(statement)?;
        //        }
        //        if let Some(expression) = block.expression {
        //            executor.evaluate(*expression)
        //        } else {
        //            Ok("void".to_owned())
        //        }
    }

    fn evaluate_conditional(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<String, Error> {
        log::trace!("Conditional expression : {}", conditional);

        unimplemented!();
        //        let result = match self.evaluate(*conditional.condition)? {
        //            value => {
        //                return Err(Error::ConditionalExpectedBooleanExpression(
        //                    conditional.location,
        //                    Value::Void,
        //                ))
        //            }
        //        };
        //
        //        if result.is_true() {
        //            let mut executor =
        //                Writer::new(Scope::new(Some(self.scope.clone())), self.generator.clone());
        //            executor.evaluate_block(conditional.main_block)
        //        } else if let Some(else_if) = conditional.else_if {
        //            let mut executor =
        //                Writer::new(Scope::new(Some(self.scope.clone())), self.generator.clone());
        //            executor.evaluate_conditional(*else_if)
        //        } else if let Some(else_block) = conditional.else_block {
        //            let mut executor =
        //                Writer::new(Scope::new(Some(self.scope.clone())), self.generator.clone());
        //            executor.evaluate_block(else_block)
        //        } else {
        //            Ok("void".to_owned())
        //        }
    }

    fn stringify_operand(operand: &OperatorExpressionOperand) -> String {
        match operand {
            OperatorExpressionOperand::Identifier(identifier) => identifier.name().to_owned(),
            OperatorExpressionOperand::Literal(literal) => literal.to_string(),
            _ => unimplemented!(),
        }
    }

    fn next_sequence(&mut self) -> String {
        self.sequence += 1;
        format!("{:06}", self.sequence)
    }
}
