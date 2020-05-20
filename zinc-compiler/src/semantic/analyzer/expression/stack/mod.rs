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

static THERE_MUST_ALWAYS_BE_AN_OPERAND: &str =
    "Operand stack balance is kept by the evaluation logic";

impl Stack {
    const ELEMENTS_INITIAL_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(Self::ELEMENTS_INITIAL_CAPACITY),
        }
    }

    pub fn top(&self) -> &Element {
        self.elements.last().expect(THERE_MUST_ALWAYS_BE_AN_OPERAND)
    }

    pub fn push(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Element {
        self.elements.pop().expect(THERE_MUST_ALWAYS_BE_AN_OPERAND)
    }
}
