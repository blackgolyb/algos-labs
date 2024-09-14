pub trait TableMethods<T> {
    fn get(&self, i: usize, j: usize) -> T;
    fn set(&mut self, i: usize, j: usize, elem: T);
    fn get_size(&self) -> (usize, usize);
}
