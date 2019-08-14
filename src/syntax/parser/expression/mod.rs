//!
//! The expression parser.
//!

mod boolean;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::Error;

use self::boolean::Parser as BooleanParser;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        BooleanParser::default().parse(stream)
    }
}
