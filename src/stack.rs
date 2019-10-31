pub struct Stack<T> {
    elements: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.elements.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.elements.len() {
            None
        } else {
            self.elements.get(self.elements.len() - index - 1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::<i32>::new();

        assert_eq!(stack.top(), None);
        assert_eq!(stack.get(0), None);
        assert_eq!(stack.size(), 0);

        stack.push(1);
        assert_eq!(stack.top(), Some(&1));
        assert_eq!(stack.get(0), Some(&1));
        assert_eq!(stack.get(1), None);
        assert_eq!(stack.size(), 1);

        stack.push(7);
        assert_eq!(stack.top(), Some(&7));
        assert_eq!(stack.get(0), Some(&7));
        assert_eq!(stack.get(1), Some(&1));
        assert_eq!(stack.get(2), None);
        assert_eq!(stack.size(), 2);

        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);;
        assert_eq!(stack.size(), 0);
    }
}
