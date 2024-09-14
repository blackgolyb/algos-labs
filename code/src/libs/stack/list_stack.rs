use std::fmt::{self, Display};

use super::super::list::double_linked_list::List;
use super::StackMethods;

pub struct Stack<T> {
    container: List<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            container: List::<T>::new(),
        }
    }
}

impl<T> StackMethods<T> for Stack<T> {
    fn pop(&mut self) -> Option<T> {
        self.container.pop()
    }

    fn push(&mut self, value: T) {
        self.container.push(value);
    }
}

impl<T: Display> Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.container.fmt(f)
    }
}
