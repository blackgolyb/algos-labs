use super::super::sort_preamble::*;
use super::super::variants::InsertionSort;

struct Bucket<T> {
    hash: usize,
    values: Vec<T>,
}

impl<T> Bucket<T> {
    fn new(hash: usize, value: T) -> Bucket<T> {
        Bucket {
            hash,
            values: vec![value],
        }
    }
}

sort!(
    BucketSort + AsIndex
    |args: SortArgs<T>| {
        let arr = args.0;
        let sort = args.1;

        let mut sub_sort = InsertionSort::new();

        let mut buckets: Vec<Bucket<T>> = vec![];
        let n = arr.len();

        for _ in 0..n {
            let val = arr.pop().unwrap();
            let hash = val.as_index();
            sort.log_swap();
            match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
                Ok(index) => buckets[index].values.push(val),
                Err(index) => buckets.insert(index, Bucket::new(hash, val)),
            }
        }

        unsafe {
            arr.set_len(n);
        }

        let mut i = 0;
        for mut bucket in buckets.into_iter() {
            sub_sort.sort(&mut bucket.values);
            sort.logger.marge_sub_logger(sub_sort.logger());
            for value in bucket.values {
                sort.log_swap();
                arr[i] = value;
                i += 1;
            }
        }
    }
);
