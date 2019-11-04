use franklin_crypto::bellman::Variable;
use bellman::pairing::Engine;

#[derive(Debug, Clone)]
pub struct Primitive<E: Engine> {
    pub value: Option<E::Fr>,
    pub variable: Variable,
}

impl<E:Engine> Copy for Primitive<E> {}

#[derive(Debug)]
pub struct Stack<E: Engine> {
    elements: Vec<Primitive<E>>
}

impl<E: Engine> Stack<E> {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn push(&mut self, x: Primitive<E>) {
        self.elements.push(x);
    }

    pub fn pop(&mut self) -> Option<Primitive<E>> {
        self.elements.pop()
    }

    pub fn top(&self) -> Option<&Primitive<E>> {
        self.elements.last()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get(&self, index: usize) -> Option<Primitive<E>> {
        if index >= self.elements.len() {
            None
        } else {
            self.elements
                .get(self.elements.len() - index - 1)
                .map(|p| *p)
        }
    }
}

#[cfg(test)]
mod test {
//    use super::*;
//
//    #[test]
//    fn test_stack() {
//        let mut stack = Stack::<i32>::new();
//
//        assert_eq!(stack.top(), None);
//        assert_eq!(stack.get(0), None);
//        assert_eq!(stack.len(), 0);
//
//        stack.push(1);
//        assert_eq!(stack.top(), Some(&1));
//        assert_eq!(stack.get(0), Some(&1));
//        assert_eq!(stack.get(1), None);
//        assert_eq!(stack.len(), 1);
//
//        stack.push(7);
//        assert_eq!(stack.top(), Some(&7));
//        assert_eq!(stack.get(0), Some(&7));
//        assert_eq!(stack.get(1), Some(&1));
//        assert_eq!(stack.get(2), None);
//        assert_eq!(stack.len(), 2);
//
//        assert_eq!(stack.pop(), Some(7));
//        assert_eq!(stack.pop(), Some(1));
//        assert_eq!(stack.pop(), None);
//        assert_eq!(stack.len(), 0);
//    }
}
