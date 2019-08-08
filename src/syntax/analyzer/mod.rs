//!
//! The syntax analyzer.
//!

mod inputs;
mod r#type;
mod witness;

pub use self::inputs::InputsAnalyzer;
pub use self::r#type::TypeAnalyzer;
pub use self::witness::WitnessAnalyzer;

use crate::lexical::Stream as LexicalStream;
use crate::syntax::CircuitProgram;
use crate::Error;

pub type TokenIterator = std::iter::Peekable<LexicalStream>;

#[derive(Default)]
pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(&mut self, iterator: TokenIterator) -> Result<CircuitProgram, Error> {
        let (iterator, inputs) = InputsAnalyzer::default().analyze(iterator)?;
        let (_iterator, witnesses) = WitnessAnalyzer::default().analyze(iterator)?;

        Ok(CircuitProgram { inputs, witnesses })
    }
}
