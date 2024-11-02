use super::super::sort_preamble::*;

fn heapify<T: Ord>(sort: &mut HeapSort, arr: &mut Vec<T>, n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && sort.gt(arr, left, largest) {
        largest = left;
    }

    if right < n && sort.gt(arr, right, largest) {
        largest = right;
    }

    if largest != i {
        sort.swap(arr, i, largest);
        heapify(sort, arr, n, largest);
    }
}

sort! {
    HeapSort | args: SortArgs<T> | {
        let arr = args.0;
        let sort = args.1;

        let n = arr.len();
        for i in (0..n / 2).rev() {
            heapify(sort, arr, n, i);
        }

        for i in (1..n).rev() {
            sort.swap(arr, 0, i);
            heapify(sort, arr, i, 0);
        }
    }
}
