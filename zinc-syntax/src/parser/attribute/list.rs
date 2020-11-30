//!
//! The attribute element list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::attribute::element::Parser as AttributeElementParser;
use crate::tree::attribute::element::Element as AttributeElement;

///
/// The attribute element list parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parsed attribute elements.
    elements: Vec<AttributeElement>,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an attribute element list.
    ///
    /// 'test, two = 42, three(default)'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(Vec<AttributeElement>, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                token
                @
                Token {
                    lexeme: Lexeme::Identifier(_),
                    ..
                } => {
                    let (element, next) =
                        AttributeElementParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    self.elements.push(element);
                }
                token => return Ok((self.elements, Some(token))),
            }

            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => continue,
                token => return Ok((self.elements, Some(token))),
            }
        }
    }
}
