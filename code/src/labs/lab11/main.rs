use rand::Rng;

use crate::libs::sort::{
    variants::{BubbleSort, BucketSort, HeapSort, InsertionSort, RadixSort},
    AsIndex, Sort,
};

impl AsIndex for i64 {
    fn as_index(&self) -> usize {
        (self / 10) as usize
    }
}

pub fn main() {
    let mut rng = rand::thread_rng();
    let mut arr = Vec::<i64>::new();
    let n = 100;

    for _ in 0..n {
        let t: u32 = rng.gen();
        arr.push((t % (n * 10)) as i64);
    }

    println!("{:?}", arr);
    // let mut sort = BucketSort::new();
    // let mut sort = InsertionSort::new();
    let mut sort = RadixSort::new();
    // let mut sort = HeapSort::new();
    // let mut sort = BubbleSort::new();
    sort.sort(&mut arr);
    println!("{:?}", arr);
    println!("{}", arr.is_sorted());
}
