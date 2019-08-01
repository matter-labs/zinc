//!
//! The syntax analyzer.
//!

use std::ops::Deref;
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro::Delimiter;

use crate::syntax::State;
use crate::syntax::Error;
use crate::syntax::Keyword;
use crate::syntax::TypeName;

pub struct Analyzer {
    state: State,
}

impl Analyzer {
    const COLON: char = ':';
    const SEMICOLON: char = ';';

    pub fn new() -> Self {
        Self { state: State::default() }
    }

    pub fn analyze(&mut self, stream: TokenStream) -> Result<(), Error> {
        self.stream(stream)?;
        if let State::InputsElementVariableOrWitnessKeywordOrEnd | State::WitnessElementVariableOrEnd = self.state {
            Ok(())
        } else {
            Err(Error::UnexpectedEnd(self.state))
        }
    }

    fn stream(&mut self, stream: TokenStream) -> Result<(), Error> {
        for tree in stream.into_iter() {
            self.tree(tree)?;
        }
        Ok(())
    }

    fn tree(&mut self, tree: TokenTree) -> Result<(), Error> {
        match self.state {
            State::InputsKeyword => self.inputs_keyword(tree),
            State::InputsBrace => self.inputs_brace(tree),
            State::InputsElementVariableOrWitnessKeywordOrEnd => self.inputs_element_variable_or_witness_keyword_or_end(tree),
            State::InputsElementColon => self.inputs_element_colon(tree),
            State::InputsElementType => self.inputs_element_type(tree),
            State::InputsElementSemicolon => self.inputs_element_semicolon(tree),

            State::WitnessBrace => self.witness_brace(tree),
            State::WitnessElementVariableOrEnd => self.witness_element_variable_or_end(tree),
            State::WitnessElementColon => self.witness_element_colon(tree),
            State::WitnessElementType => self.witness_element_type(tree),
            State::WitnessElementSemicolon => self.witness_element_semicolon(tree),
        }
    }

    fn inputs_keyword(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE inputs_keyword: {}", tree);

        const EXPECTED: [&'static str; 1] = ["inputs"];

        match tree {
            TokenTree::Ident(ref ident) => {
                if EXPECTED.contains(&ident.to_string().deref()) {
                    self.state = State::InputsBrace;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn inputs_brace(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE inputs_brace: {}", tree);

        const EXPECTED: [&'static str; 1] = ["{"];

        match tree {
            TokenTree::Group(ref group) => {
                if let Delimiter::Brace = group.delimiter() {
                    self.state = State::InputsElementVariableOrWitnessKeywordOrEnd;
                    self.stream(group.stream())?;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn inputs_element_variable_or_witness_keyword_or_end(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE inputs_element_variable_or_witness: {}", tree);

        const EXPECTED: [&'static str; 2] = ["{variable}", "witness"];

        match tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string();
                if ident.as_str() == "witness" {
                    self.state = State::WitnessBrace;
                    Ok(())
                } else if let Err(_variable) = Keyword::from_str(&ident.to_string().deref()) {
                    self.state = State::InputsElementColon;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn inputs_element_colon(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE inputs_element_colon: {}", tree);

        const EXPECTED: [&'static str; 1] = [":"];

        match tree {
            TokenTree::Punct(ref punct) => {
                if punct.as_char() == Self::COLON {
                    self.state = State::InputsElementType;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn inputs_element_type(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE inputs_element_type: {}", tree);

        const EXPECTED: [&'static str; 1] = ["{type}"];

        match tree {
            TokenTree::Ident(ref ident) => {
                if let Ok(_type) = TypeName::from_str(&ident.to_string().deref()) {
                    self.state = State::InputsElementSemicolon;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn inputs_element_semicolon(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE inputs_element_semicolon: {}", tree);

        const EXPECTED: [&'static str; 1] = [";"];

        match tree {
            TokenTree::Punct(ref punct) => {
                if punct.as_char() == Self::SEMICOLON {
                    self.state = State::InputsElementVariableOrWitnessKeywordOrEnd;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn witness_brace(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE witness_brace: {}", tree);

        const EXPECTED: [&'static str; 1] = ["{"];

        match tree {
            TokenTree::Group(ref group) => {
                if let Delimiter::Brace = group.delimiter() {
                    self.state = State::WitnessElementVariableOrEnd;
                    self.stream(group.stream())?;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn witness_element_variable_or_end(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE witness_element_variable_or_witness: {}", tree);

        const EXPECTED: [&'static str; 1] = ["{variable}"];

        match tree {
            TokenTree::Ident(ref ident) => {
                if let Err(_variable) = Keyword::from_str(&ident.to_string().deref()) {
                    self.state = State::WitnessElementColon;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn witness_element_colon(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE witness_element_colon: {}", tree);

        const EXPECTED: [&'static str; 1] = [":"];

        match tree {
            TokenTree::Punct(ref punct) => {
                if punct.as_char() == Self::COLON {
                    self.state = State::WitnessElementType;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn witness_element_type(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE witness_element_type: {}", tree);

        const EXPECTED: [&'static str; 1] = ["{type}"];

        match tree {
            TokenTree::Ident(ref ident) => {
                if let Ok(_type) = TypeName::from_str(&ident.to_string().deref()) {
                    self.state = State::WitnessElementSemicolon;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }

    fn witness_element_semicolon(&mut self, tree: TokenTree) -> Result<(), Error> {
        println!("TRACE witness_element_semicolon: {}", tree);

        const EXPECTED: [&'static str; 1] = [";"];

        match tree {
            TokenTree::Punct(ref punct) => {
                if punct.as_char() == Self::SEMICOLON {
                    self.state = State::WitnessElementVariableOrEnd;
                    Ok(())
                } else {
                    Err(Error::Expected(EXPECTED.to_vec(), tree.to_string()))
                }
            },
            _ => Err(Error::Expected(EXPECTED.to_vec(), tree.to_string())),
        }
    }
}
