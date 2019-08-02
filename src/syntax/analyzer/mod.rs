//!
//! The syntax analyzer.
//!

mod inputs;
mod witness;

use proc_macro::TokenStream;

use crate::syntax::Error;

use self::inputs::Inputs;
use self::inputs::InputsAnalyzer;
use self::witness::Witness;
use self::witness::WitnessAnalyzer;

pub type TokenIterator = std::iter::Peekable<proc_macro::token_stream::IntoIter>;

#[derive(Debug)]
pub struct Circuit {
    inputs: Inputs,
    witness: Witness,
}

pub struct Analyzer {}

impl Analyzer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyze(&mut self, stream: TokenStream) -> Result<Circuit, Error> {
        let iterator: TokenIterator = stream.into_iter().peekable();

        let (iterator, inputs) = InputsAnalyzer::new().analyze(iterator)?;
        let (_iterator, witness) = WitnessAnalyzer::new().analyze(iterator)?;

        Ok(Circuit {
            inputs,
            witness,
        })
    }
}
