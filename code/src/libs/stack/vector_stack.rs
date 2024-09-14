use super::super::vector::Vector;
use super::StackMethods;

pub struct Stack<T> {
    container: Vector<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            container: Vector::<T>::new(),
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
