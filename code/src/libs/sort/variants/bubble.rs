use super::super::sort_preamble::*;

sort! {
    BubbleSort | args: SortArgs<T> | {
        let arr = args.0;
        let sort = args.1;

        let n = arr.len();
        for i in 0..n {
            let mut swapped = false;
            for j in 0..(n - i - 1) {
                if sort.gt(arr, j, j + 1) {
                    sort.swap(arr, j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }
}
