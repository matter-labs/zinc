//!
//! The expression parser.
//!

mod arithmetic;
mod boolean;

pub use self::arithmetic::Parser as ArithmeticParser;
pub use self::boolean::Parser as BooleanParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::Error;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        log::trace!("expression");

        stream.borrow_mut().backtrack();
        match BooleanParser::default().parse(stream.clone()) {
            Ok(expression) => return Ok(expression),
            Err(error) => {
                log::trace!("expression ROLLBACK: {}", error);
                stream.borrow_mut().rollback();
            }
        }

        ArithmeticParser::default().parse(stream)
    }
}
