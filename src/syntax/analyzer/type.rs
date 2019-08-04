//!
//! The syntax analyzer of type.
//!

use std::ops::Deref;
use std::str::FromStr;

use log::*;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::syntax;
use crate::syntax::Error;
use crate::syntax::Identifier;
use crate::syntax::Input;
use crate::syntax::Keyword;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;
use crate::syntax::TypeBuilderError;
use crate::syntax::TypeKeyword;

use super::TokenIterator;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordOrParenthesis,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordOrParenthesis
    }
}

#[derive(Default)]
pub struct TypeAnalyzer {
    state: State,
    builder: TypeBuilder,
}

impl TypeAnalyzer {
    pub fn analyze(mut self, mut iterator: TokenIterator) -> Result<(TokenIterator, Type), Error> {
        loop {
            if let State::End = self.state {
                return Ok((iterator, self.builder.finish().map_err(Error::InvalidType)?));
            }

            if let Some(tree) = iterator.next() {
                self.tree(tree)?;
            } else {
                return Err(Error::UnexpectedEnd);
            }
        }
    }

    fn tree(&mut self, tree: TokenTree) -> Result<(), Error> {
        match self.state {
            State::KeywordOrParenthesis => self.keyword_or_parenthesis(tree),
            State::End => unreachable!(),
        }
    }

    fn keyword_or_parenthesis(&mut self, tree: TokenTree) -> Result<(), Error> {
        trace!("keyword_or_parenthesis: {}", tree);

        const EXPECTED: [&str; 2] = ["{keyword}", "("];

        match tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string();
                match TypeKeyword::from_str(&ident.deref()) {
                    Ok(keyword) => {
                        self.builder.set_keyword(keyword);

                        self.state = State::End;
                        Ok(())
                    }
                    Err(error) => Err(Error::InvalidTypeKeyword(ident, error)),
                }
            }
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }
}
