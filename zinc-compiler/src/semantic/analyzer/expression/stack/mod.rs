//!
//! The semantic analyzer expression evaluation stack.
//!

pub mod element;

use self::element::Element;

///
/// The semantic analyzer expression evaluation stack.
///
#[derive(Debug, Default)]
pub struct Stack {
    /// The evaluation stack elements vector.
    elements: Vec<Element>,
}

impl Stack {
    /// The evaluation stack default capacity.
    const ELEMENTS_INITIAL_CAPACITY: usize = 16;

    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(Self::ELEMENTS_INITIAL_CAPACITY),
        }
    }

    ///
    /// Returns the top element reference, that is, peek the evaluation stack.
    ///
    /// Should not be called if the stack is empty. In such cases, the method will panic.
    ///
    pub fn top(&self) -> &Element {
        self.elements
            .last()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
    }

    ///
    /// Pops an element from the evaluation stack.
    ///
    pub fn push(&mut self, element: Element) {
        self.elements.push(element);
    }

    ///
    /// Pops an element from the evaluation stack.
    ///
    pub fn pop(&mut self) -> Element {
        self.elements
            .pop()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
    }
}
