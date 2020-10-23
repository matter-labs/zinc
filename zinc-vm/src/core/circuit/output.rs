//!
//! The virtual machine circuit output.
//!

use zinc_build::Value as BuildValue;

///
/// The virtual machine circuit output.
///
pub struct Output {
    /// The circuit output result, which is public data for now.
    pub result: BuildValue,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(result: BuildValue) -> Self {
        Self { result }
    }
}
