//!
//! The expression parser.
//!

mod boolean;

use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::Error;

use self::boolean::Parser as BooleanParser;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(self, stream: TokenStream) -> Result<(TokenStream, Vec<Token>), Error> {
        BooleanParser::default().parse(stream)
    }
}
