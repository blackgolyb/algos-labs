use super::logger::{Logger, Metrics};

pub trait AsIndex {
    fn as_index(&self) -> usize;
}

pub trait SortLogging {
    fn logger(&mut self) -> &mut Logger;

    fn get_metrics(&mut self) -> Metrics {
        self.logger().get_metrics()
    }

    fn log_swap(&mut self) {
        self.logger().log_swap();
    }
    fn log_compare(&mut self) {
        self.logger().log_compare();
    }
}

pub trait Sort<T>
where
    T: Ord,
    Self: SortLogging,
{
    fn sort(&mut self, arr: &mut Vec<T>);

    fn swap(&mut self, arr: &mut Vec<T>, a: usize, b: usize) {
        self.logger().log_swap();
        arr.swap(a, b)
    }
    fn eq(&mut self, arr: &Vec<T>, a: usize, b: usize) -> bool {
        self.logger().log_compare();
        arr[a] == arr[b]
    }
    fn neq(&mut self, arr: &Vec<T>, a: usize, b: usize) -> bool {
        self.logger().log_compare();
        arr[a] != arr[b]
    }
    fn gt(&mut self, arr: &Vec<T>, a: usize, b: usize) -> bool {
        self.logger().log_compare();
        arr[a] > arr[b]
    }
    fn lt(&mut self, arr: &Vec<T>, a: usize, b: usize) -> bool {
        self.logger().log_compare();
        arr[a] < arr[b]
    }
    fn gte(&mut self, arr: &Vec<T>, a: usize, b: usize) -> bool {
        self.logger().log_compare();
        arr[a] >= arr[b]
    }
    fn lte(&mut self, arr: &Vec<T>, a: usize, b: usize) -> bool {
        self.logger().log_compare();
        arr[a] <= arr[b]
    }
}
