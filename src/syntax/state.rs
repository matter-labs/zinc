//!
//! The syntax analyzer state.
//!

#[derive(Debug, Clone, Copy)]
pub enum State {
    InputsKeyword,
    InputsBrace,
    InputsElementVariableOrWitnessKeywordOrEnd,
    InputsElementColon,
    InputsElementType,
    InputsElementSemicolon,

    WitnessBrace,
    WitnessElementVariableOrEnd,
    WitnessElementColon,
    WitnessElementType,
    WitnessElementSemicolon,
}

impl State {
    pub fn new() -> Self {
        State::InputsKeyword
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
