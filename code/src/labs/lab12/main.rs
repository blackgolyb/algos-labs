use crate::libs::sort::{
    testing::test_perf,
    variants::{BucketSort, HeapSort, RadixSort},
    AsIndex,
};

impl AsIndex for i64 {
    fn as_index(&self) -> usize {
        (self / 10) as usize
    }
}

pub fn main() {
    let test_cases: Vec<usize> = vec![20, 1000, 5000, 10000, 50000];

    let mut sort = HeapSort::new();
    let file_name = "heap.csv".to_string();
    test_perf(&mut sort, file_name, test_cases.clone());

    let mut sort = BucketSort::new();
    let file_name = "bucket.csv".to_string();
    test_perf(&mut sort, file_name, test_cases.clone());

    let mut sort = RadixSort::new();
    let file_name = "radix.csv".to_string();
    test_perf(&mut sort, file_name, test_cases.clone());
}
