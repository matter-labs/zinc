//!
//! The interpreter executor.
//!

use std::collections::HashMap;
use std::str;

use num_traits::Zero;

use crate::interpreter::Error;
use crate::interpreter::Evaluator;
use crate::interpreter::Field;
use crate::interpreter::Warning;
use crate::syntax::Statement;

#[derive(Default)]
pub struct Executor {
    evaluator: Evaluator,
    variables: HashMap<Vec<u8>, Field>,
}

impl Executor {
    pub fn execute(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Debug(debug) => {
                let result = self.evaluator.evaluate(debug.expression, &self.variables)?;
                println!("{}", result);
            }
            Statement::Let(r#let) => {
                if self.variables.contains_key(&r#let.identifier.name) {
                    log::warn!(
                        "{}",
                        Warning::RedeclaredVariable(
                            r#let.identifier.location,
                            unsafe { str::from_utf8_unchecked(&r#let.identifier.name) }.to_owned(),
                        )
                    );
                }
                let mut result = self.evaluator.evaluate(r#let.expression, &self.variables)?;
                if let Some(r#type) = r#let.r#type {
                    result.type_variant = r#type.variant;
                }
                self.variables.insert(r#let.identifier.name.clone(), result);
            }
            Statement::Require(require) => {
                let location = require.location;
                let result = self
                    .evaluator
                    .evaluate(require.expression, &self.variables)?;
                if result.value.is_zero() {
                    panic!("Require {} failed at line {}!", require.id, location.line);
                }
            }
        }
        Ok(())
    }
}
