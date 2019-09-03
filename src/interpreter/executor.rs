//!
//! The interpreter executor.
//!

use std::cell::RefCell;
use std::rc::Rc;
use std::str;

use num_traits::Zero;

use crate::interpreter::Error;
use crate::interpreter::Evaluator;
use crate::interpreter::Place;
use crate::interpreter::Scope;
use crate::interpreter::Warning;
use crate::syntax::Statement;

#[derive(Default)]
pub struct Executor {
    evaluator: Evaluator,
    scope: Rc<RefCell<Scope>>,
}

impl Executor {
    pub fn execute(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Debug(debug) => {
                let result = self
                    .evaluator
                    .evaluate(debug.expression, self.scope.clone())?;
                println!("{}", result);
            }
            Statement::Let(r#let) => {
                if self.scope.borrow().is_variable_declared(&r#let.identifier) {
                    log::warn!(
                        "{}",
                        Warning::RedeclaredVariable(
                            r#let.identifier.location,
                            unsafe { str::from_utf8_unchecked(&r#let.identifier.name) }.to_owned(),
                        )
                    );
                }
                let mut result = self
                    .evaluator
                    .evaluate(r#let.expression, self.scope.clone())?;
                if let Some(r#type) = r#let.r#type {
                    result = result
                        .cast(r#type.variant)
                        .map_err(|error| Error::Operator(r#type.location, error))?
                }
                let place = Place::new(r#let.identifier.clone(), result, r#let.is_mutable);
                self.scope.borrow_mut().declare_variable(place);
            }
            Statement::Require(require) => {
                let result = self
                    .evaluator
                    .evaluate(require.expression, self.scope.clone())?;
                if result.field.is_zero() {
                    return Err(Error::RequireFailure(require.location, require.id));
                }
            }
            Statement::Expression(expression) => {
                self.evaluator.evaluate(expression, self.scope.clone())?;
            }
        }
        Ok(())
    }
}
