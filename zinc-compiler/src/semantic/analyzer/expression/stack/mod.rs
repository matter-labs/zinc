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
    elements: Vec<Element>,
}

static PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND: &str =
    "Operand stack balance is kept by the evaluation logic";

impl Stack {
    const DEFAULT_INITIAL_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(Self::DEFAULT_INITIAL_CAPACITY),
        }
    }

    pub fn push(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Element {
        self.elements
            .pop()
            .expect(PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND)
    }
}
