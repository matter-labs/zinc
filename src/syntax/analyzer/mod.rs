//!
//! The syntax analyzer.
//!

mod inputs;
mod r#type;
mod witness;

pub use self::inputs::InputsAnalyzer;
pub use self::r#type::TypeAnalyzer;
pub use self::witness::WitnessAnalyzer;

use proc_macro2::TokenStream;

use crate::syntax::CircuitProgram;
use crate::syntax::Error;

pub type TokenIterator = std::iter::Peekable<proc_macro2::token_stream::IntoIter>;

#[derive(Default)]
pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(&mut self, stream: TokenStream) -> Result<CircuitProgram, Error> {
        let iterator: TokenIterator = stream.into_iter().peekable();

        let (iterator, inputs) = InputsAnalyzer::default().analyze(iterator)?;
        let (_iterator, witness) = WitnessAnalyzer::default().analyze(iterator)?;

        Ok(CircuitProgram { inputs, witness })
    }
}
