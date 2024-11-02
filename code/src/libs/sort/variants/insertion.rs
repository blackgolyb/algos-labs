use super::super::sort_preamble::*;

sort! {
    InsertionSort | args: SortArgs<T> | {
        let arr = args.0;
        let sort = args.1;

        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && sort.gt(arr, j-1, j) {
                sort.swap(arr, j, j - 1);
                j -= 1;
            }
        }
    }
}
