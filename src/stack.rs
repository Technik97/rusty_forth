use std::fmt::Display;

#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn pop(&mut self) -> T {
        match self.elements.pop() {
            Some(element) => element,
            None => {
                eprintln!("Stack underflow");
                std::process::exit(1);
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Display> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stack: [")?;
        for (i, elem) in self.elements.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", elem)?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_new() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_push() {
        let mut stack = Stack::new();
        stack.push(1);
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
        assert!(stack.is_empty());
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn test_stack_underflow() {
        let mut stack: Stack<i32> = Stack::new();
        stack.pop();
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn test_stack_size() {
        let mut stack = Stack::new();
        assert_eq!(stack.size(), 0);
        stack.push(1);
        assert_eq!(stack.size(), 1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_stack_display() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(format!("{}", stack), "Stack: [1, 2, 3]");
    }
}
