//!
//! The syntax analyzer of witnesses.
//!

use std::ops::Deref;
use std::str::FromStr;

use log::*;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::syntax;
use crate::syntax::Error;
use crate::syntax::Identificator;
use crate::syntax::Keyword;
use crate::syntax::TypeKeyword;
use crate::syntax::Witness;

use super::TokenIterator;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Keyword,
    Group,
    ElementVariable,
    ElementColon,
    ElementType,
    ElementSemicolon,
    End,
}

impl State {
    pub fn new() -> Self {
        State::Keyword
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WitnessAnalyzer {
    state: State,
    witnesses: Vec<Witness>,
}

impl WitnessAnalyzer {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            witnesses: vec![],
        }
    }

    pub fn analyze(
        mut self,
        mut iterator: TokenIterator,
    ) -> Result<(TokenIterator, Vec<Witness>), Error> {
        loop {
            if let State::End = self.state {
                return Ok((iterator, self.witnesses));
            }

            if let Some(tree) = iterator.next() {
                self.tree(tree)?;
            } else {
                return Err(Error::UnexpectedEnd);
            }
        }
    }

    fn stream(&mut self, stream: TokenStream) -> Result<(), Error> {
        for tree in stream.into_iter() {
            self.tree(tree)?;
        }
        if let State::ElementVariable = self.state {
            self.state = State::End;
        }
        Ok(())
    }

    fn tree(&mut self, tree: TokenTree) -> Result<(), Error> {
        match self.state {
            State::Keyword => self.keyword(tree),
            State::Group => self.group(tree),
            State::ElementVariable => self.element_variable(tree),
            State::ElementColon => self.element_colon(tree),
            State::ElementType => self.element_type(tree),
            State::ElementSemicolon => self.element_semicolon(tree),
            State::End => unreachable!(),
        }
    }

    fn keyword(&mut self, tree: TokenTree) -> Result<(), Error> {
        trace!("keyword: {}", tree);

        const EXPECTED: [&str; 1] = ["witness"];

        match tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string();
                if let Ok(Keyword::Witness) = Keyword::from_str(&ident.deref()) {
                    self.state = State::Group;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            }
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    pub fn group(&mut self, tree: TokenTree) -> Result<(), Error> {
        trace!("group: {}", tree);

        const EXPECTED: [&str; 1] = ["{"];

        match tree {
            TokenTree::Group(ref group) => {
                if let syntax::GROUP_DELIMITER = group.delimiter() {
                    self.state = State::ElementVariable;
                    self.stream(group.stream())?;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            }
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    pub fn element_variable(&mut self, tree: TokenTree) -> Result<(), Error> {
        trace!("element_variable: {}", tree);

        const EXPECTED: [&str; 1] = ["{variable}"];

        match tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string();
                match Identificator::from_str(&ident.deref()) {
                    Ok(_variable) => {
                        self.state = State::ElementColon;
                        Ok(())
                    }
                    Err(error) => Err(Error::InvalidIdentificator(ident, error)),
                }
            }
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    pub fn element_colon(&mut self, tree: TokenTree) -> Result<(), Error> {
        trace!("element_colon: {}", tree);

        const EXPECTED: [&str; 1] = [":"];

        match tree {
            TokenTree::Punct(ref punct) => {
                if punct.as_char() == syntax::COLON {
                    self.state = State::ElementType;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            }
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    pub fn element_type(&mut self, tree: TokenTree) -> Result<(), Error> {
        trace!("element_type: {}", tree);

        const EXPECTED: [&str; 1] = ["{type}"];

        match tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string();
                match TypeKeyword::from_str(&ident.deref()) {
                    Ok(_keyword) => {
                        self.state = State::ElementSemicolon;
                        Ok(())
                    }
                    Err(error) => Err(Error::InvalidTypeKeyword(ident, error)),
                }
            }
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    pub fn element_semicolon(&mut self, tree: TokenTree) -> Result<(), Error> {
        trace!("element_semicolon: {}", tree);

        const EXPECTED: [&str; 1] = [";"];

        match tree {
            TokenTree::Punct(ref punct) => {
                if punct.as_char() == syntax::SEMICOLON {
                    self.witnesses.count += 1;

                    self.state = State::ElementVariable;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            }
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }
}
