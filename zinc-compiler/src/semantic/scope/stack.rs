//!
//! The semantic analyzer scope stack.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::scope::Scope;

///
/// The scope stack is a linked list, where a child has access to its parent.
/// The global scope is the bottom element of the stack and has no parent.
/// Since the scopes are passed around the compiler, they are wrapped into an `Rc<RefCell<_>>`
/// to allow shared access.
///
pub struct Stack {
    elements: Vec<Rc<RefCell<Scope>>>,
}

impl Stack {
    const ELEMENTS_INITIAL_CAPACITY: usize = 16;

    ///
    /// Initializes a nested scope stack with an explicit parent.
    ///
    pub fn new(root: Rc<RefCell<Scope>>) -> Self {
        let mut elements = Vec::with_capacity(Self::ELEMENTS_INITIAL_CAPACITY);
        elements.push(root);
        Self { elements }
    }

    ///
    /// Initializes a scope stack starting from the global scope.
    ///
    pub fn new_global() -> Self {
        let mut elements = Vec::with_capacity(Self::ELEMENTS_INITIAL_CAPACITY);
        elements.push(Scope::new_global().wrap());
        Self { elements }
    }

    ///
    /// Returns the deepest scope in the current hierarchy.
    ///
    pub fn top(&self) -> Rc<RefCell<Scope>> {
        self.elements
            .last()
            .cloned()
            .expect(crate::panic::THERE_MUST_ALWAYS_BE_A_SCOPE)
    }

    ///
    /// Pushes the current scope deeper and initializes a new one with it as the parent.
    ///
    pub fn push(&mut self) {
        self.elements.push(Scope::new_child(self.top()));
    }

    ///
    /// Removes the deepest scope from the current hierarchy.
    ///
    pub fn pop(&mut self) {
        self.elements
            .pop()
            .expect(crate::panic::THERE_MUST_ALWAYS_BE_A_SCOPE);
    }
}
