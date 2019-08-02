//!
//! The syntax analyzer of inputs.
//!

use std::ops::Deref;
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro::TokenTree;

use crate::syntax;
use crate::syntax::Error;
use crate::syntax::Keyword;
use crate::syntax::VariableName;
use crate::syntax::TypeName;

use super::TokenIterator;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Keyword,
    Bracket,
    ElementVariable,
    ElementColon,
    ElementType,
    ElementSemicolon,
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

pub struct InputsAnalyzer {
    state: State,
    inputs: Inputs,
}

#[derive(Debug)]
pub struct Inputs {
    count: usize,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            count: 0,
        }
    }
}

impl InputsAnalyzer {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            inputs: Inputs::new(),
        }
    }

    pub fn analyze(mut self, mut iterator: TokenIterator) -> Result<(TokenIterator, Inputs), Error> {
        loop {
            if self.is_end(iterator.peek()) {
                return Ok((iterator, self.inputs));
            }

            if let Some(tree) = iterator.next() {
                self.tree(tree)?;
            } else {
                return Err(Error::UnexpectedEnd)
            }
        }
    }

    fn is_end(&self, tree: Option<&TokenTree>) -> bool {
        if let State::ElementVariable = self.state {
            if let Some(TokenTree::Ident(ref ident)) = tree {
                if let Ok(Keyword::Witness) = Keyword::from_str(&ident.to_string().deref()) {
                    return true;
                }
            }
        }
        false
    }

    fn stream(&mut self, stream: TokenStream) -> Result<(), Error> {
        for tree in stream.into_iter() {
            self.tree(tree)?;
        }
        Ok(())
    }

    fn tree(&mut self, tree: TokenTree) -> Result<(), Error> {
        match self.state {
            State::Keyword => self.keyword(tree),
            State::Bracket => self.bracket(tree),
            State::ElementVariable => self.element_variable(tree),
            State::ElementColon => self.element_colon(tree),
            State::ElementType => self.element_type(tree),
            State::ElementSemicolon => self.element_semicolon(tree),
        }
    }

    fn keyword(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE keyword: {}", tree);

        const EXPECTED: [&'static str; 1] = ["inputs"];

        match tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string();
                if let Ok(Keyword::Inputs) = Keyword::from_str(&ident.deref()) {
                    self.state = State::Bracket;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn bracket(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE bracket: {}", tree);

        const EXPECTED: [&'static str; 1] = ["{"];

        match tree {
            TokenTree::Group(ref group) => {
                if let syntax::GROUP_DELIMITER = group.delimiter() {
                    self.state = State::ElementVariable;
                    self.stream(group.stream())?;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn element_variable(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE element_variable: {}", tree);

        const EXPECTED: [&'static str; 2] = ["{variable}", "witness"];

        match tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string();
                match VariableName::from_str(&ident.deref()) {
                    Ok(_name) => {
                        self.state = State::ElementColon;
                        Ok(())
                    },
                    Err(error) => Err(Error::InvalidVariableName(ident, error)),
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn element_colon(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE element_colon: {}", tree);

        const EXPECTED: [&'static str; 1] = [":"];

        match tree {
            TokenTree::Punct(ref punct) => {
                if punct.as_char() == syntax::COLON {
                    self.state = State::ElementType;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn element_type(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE element_type: {}", tree);

        const EXPECTED: [&'static str; 1] = ["{type}"];

        match tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string();
                if let Ok(_type) = TypeName::from_str(&ident.deref()) {
                    self.state = State::ElementSemicolon;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn element_semicolon(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE element_semicolon: {}", tree);

        const EXPECTED: [&'static str; 1] = [";"];

        match tree {
            TokenTree::Punct(ref punct) => {
                if punct.as_char() == syntax::SEMICOLON {
                    self.inputs.count += 1;

                    self.state = State::ElementVariable;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }
}
