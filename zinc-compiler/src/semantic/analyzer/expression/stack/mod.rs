//!
//! The expression semantic analyzer stack.
//!

pub mod element;

use self::element::Element;

pub struct Stack {
    elements: Vec<Element>,
}

static PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND: &str =
    "Operand stack balance is kept by the evaluation logic";

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    const DEFAULT_INITIAL_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(Self::DEFAULT_INITIAL_CAPACITY),
        }
    }

    pub fn push(&mut self, operand: Element) {
        self.elements.push(operand)
    }

    pub fn pop(&mut self) -> Element {
        self.elements
            .pop()
            .expect(PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND)
    }
}
