//!
//! The semantic analyzer scope stack.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::scope::r#type::Type as ScopeType;
use crate::semantic::scope::Scope;

///
/// The scope stack is a linked list, where a child has access to its parent.
/// The global scope is the bottom element of the stack and has no parent.
/// Since the scopes are passed around the compiler, they are wrapped into an `Rc<RefCell<_>>`
/// to allow shared access.
///
pub struct Stack {
    /// The scope stack elements, where the deepest scope is the last element.
    pub elements: Vec<Rc<RefCell<Scope>>>,
}

impl Stack {
    /// The scope stack array default capacity.
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
    /// Returns the deepest scope in the current hierarchy.
    ///
    pub fn top(&self) -> Rc<RefCell<Scope>> {
        self.elements
            .last()
            .cloned()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
    }

    ///
    /// Pushes the current scope deeper and initializes a new one with it as the parent.
    ///
    pub fn push(&mut self, name: Option<String>, r#type: ScopeType) {
        let name = match name {
            Some(name) => format!("{} {}", self.top().borrow().name, name),
            None => format!("{} => {}", self.top().borrow().name, "child"),
        };

        self.elements
            .push(Scope::new_child(name, r#type, self.top()));
    }

    ///
    /// Removes the deepest scope from the current hierarchy.
    ///
    pub fn pop(&mut self) {
        self.elements
            .pop()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
    }
}
