//!
//! The field list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::field::Parser as FieldParser;
use crate::syntax::tree::field::Field;

#[derive(Default)]
pub struct Parser {
    fields: Vec<Field>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Vec<Field>, Option<Token>), Error> {
        loop {
            match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                token
                @
                Token {
                    lexeme: Lexeme::Identifier(_),
                    ..
                } => {
                    let (field, next) =
                        FieldParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    self.fields.push(field);
                }
                token => return Ok((self.fields, Some(token))),
            }

            match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => continue,
                token => return Ok((self.fields, Some(token))),
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::cell::RefCell;
//     use std::rc::Rc;
//
//     use super::Parser;
//     use crate::lexical::Lexeme;
//     use crate::lexical::Location;
//     use crate::lexical::Token;
//     use crate::lexical::TokenStream;
//     use crate::syntax::tree::field::Field;
//     use crate::syntax::tree::identifier::Identifier;
//     use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
//     use crate::syntax::tree::r#type::Type;
//
//     #[test]
//     fn ok_empty() {
//         let input = r#""#;
//
//         let expected = Ok((
//             Vec::<Field>::new(),
//             Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_single() {
//         let input = r#"a: u232"#;
//
//         let expected = Ok((
//             vec![Field::new(
//                 Location::new(1, 1),
//                 Identifier::new(Location::new(1, 1), "a".to_owned()),
//                 Type::new(Location::new(1, 4), TypeVariant::integer_unsigned(232)),
//             )],
//             Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_single_with_comma() {
//         let input = r#"a: u232,"#;
//
//         let expected = Ok((
//             vec![Field::new(
//                 Location::new(1, 1),
//                 Identifier::new(Location::new(1, 1), "a".to_owned()),
//                 Type::new(Location::new(1, 4), TypeVariant::integer_unsigned(232)),
//             )],
//             Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_multiple() {
//         let input = r#"a: u232, b: i128, c: u104"#;
//
//         let expected = Ok((
//             vec![
//                 Field::new(
//                     Location::new(1, 1),
//                     Identifier::new(Location::new(1, 1), "a".to_owned()),
//                     Type::new(Location::new(1, 4), TypeVariant::integer_unsigned(232)),
//                 ),
//                 Field::new(
//                     Location::new(1, 10),
//                     Identifier::new(Location::new(1, 10), "b".to_owned()),
//                     Type::new(Location::new(1, 13), TypeVariant::integer_signed(128)),
//                 ),
//                 Field::new(
//                     Location::new(1, 19),
//                     Identifier::new(Location::new(1, 19), "c".to_owned()),
//                     Type::new(Location::new(1, 22), TypeVariant::integer_unsigned(104)),
//                 ),
//             ],
//             Some(Token::new(Lexeme::Eof, Location::new(1, 26))),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
// }
