use super::super::sort_preamble::*;

fn counting_sort(sort: &mut RadixSort, arr: &mut Vec<i64>, exp: i64) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    for &num in arr.iter() {
        let index = (num / exp % 10) as usize;
        count[index] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        let index = (num / exp % 10) as usize;
        output[count[index] as usize - 1] = num;
        sort.log_swap();
        count[index] -= 1;
    }

    for i in 0..n {
        arr[i] = output[i];
        sort.log_swap();
    }
}

sort! {
    RadixSort<i64> |args: SortArgs<i64>| {
        let arr = args.0;
        let sort = args.1;

        let mut max_i = 0;
        for i in 0..arr.len() {
            if sort.gt(arr, i, max_i) {
                max_i = i;
            }
        }
        let max = arr[max_i];

        let mut exp = 1;
        while max / exp > 0 {
            counting_sort(sort, arr, exp);
            exp *= 10;
        }
    }
}
