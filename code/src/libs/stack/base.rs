pub trait StackMethods<T> {
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, value: T);
}
