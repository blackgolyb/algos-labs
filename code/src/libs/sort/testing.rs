use csv::Writer;
use rand::Rng;

use super::{Metrics, Sort};

pub fn test_i64<T: Sort<i64>>(sort: &mut T, n: usize) -> Metrics {
    let mut rng = rand::thread_rng();
    let mut arr = Vec::<i64>::new();
    for _ in 0..n {
        let t: usize = rng.gen();
        arr.push((t % (n * 10)) as i64);
    }

    sort.sort(&mut arr);
    assert!(arr.is_sorted());
    sort.get_metrics()
}

pub fn test_perf<T: Sort<i64>>(sort: &mut T, file_name: String, test_cases: Vec<usize>) {
    let mut wtr = Writer::from_path(file_name).unwrap();
    wtr.write_record(&["n", "compares", "swaps", "time"])
        .unwrap();

    for n in test_cases {
        let m = test_i64(sort, n);
        wtr.write_record(&[
            n.to_string(),
            m.compares.to_string(),
            m.swaps.to_string(),
            m.time.as_nanos().to_string(),
        ])
        .unwrap();
    }
}
