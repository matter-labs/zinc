//!
//! The syntax analyzer.
//!

mod inputs;
mod witness;

use proc_macro2::TokenStream;

use crate::syntax::CircuitProgram;
use crate::syntax::Error;

use self::inputs::InputsAnalyzer;
use self::witness::WitnessAnalyzer;

pub type TokenIterator = std::iter::Peekable<proc_macro2::token_stream::IntoIter>;

pub struct Analyzer {}

impl Analyzer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyze(&mut self, stream: TokenStream) -> Result<CircuitProgram, Error> {
        let iterator: TokenIterator = stream.into_iter().peekable();

        let (iterator, inputs) = InputsAnalyzer::new().analyze(iterator)?;
        let (_iterator, witness) = WitnessAnalyzer::new().analyze(iterator)?;

        Ok(CircuitProgram { inputs, witness })
    }
}
