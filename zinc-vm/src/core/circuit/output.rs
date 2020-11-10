//!
//! The virtual machine circuit output.
//!

///
/// The virtual machine circuit output.
///
pub struct Output {
    /// The circuit output result, which is the public data for now.
    pub result: zinc_build::Value,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(result: zinc_build::Value) -> Self {
        Self { result }
    }
}
