//!
//! The semantic analyzer scope stack.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::scope::Scope;

pub struct Stack {
    elements: Vec<Rc<RefCell<Scope>>>,
}

static PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE: &str =
    "Scope stack balance is kept by the evaluation logic";

impl Stack {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;

    pub fn new(root: Rc<RefCell<Scope>>) -> Self {
        let mut elements = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
        elements.push(root);
        Self { elements }
    }

    pub fn new_global() -> Self {
        let mut elements = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
        elements.push(Rc::new(RefCell::new(Scope::new_global())));
        Self { elements }
    }

    pub fn top(&self) -> Rc<RefCell<Scope>> {
        self.elements
            .last()
            .cloned()
            .expect(PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }

    pub fn push(&mut self) {
        self.elements.push(Scope::new_child(self.top()));
    }

    pub fn push_scope(&mut self, scope: Rc<RefCell<Scope>>) {
        self.elements.push(scope);
    }

    pub fn pop(&mut self) {
        self.elements
            .pop()
            .expect(PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE);
    }
}
