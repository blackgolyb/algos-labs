use csv::Writer;

use crate::libs::list::double_linked_list::List;

use crate::libs::search::binary_search;
use crate::libs::search::linear_search;
use crate::libs::search::logger::{Logger, Metrics};

fn test(n: usize, elem: i64) -> (Metrics, Option<usize>, Metrics, Option<usize>) {
    let mut list = List::<i64>::new();
    let mut logger = Logger::new();

    for i in 0..n as i64 {
        list.push(i);
    }

    let res_linear = linear_search(&mut list, elem, &mut logger);
    let metrics_linear = logger.get_metrics();
    let res_binary = binary_search(&mut list, elem, &mut logger);
    let metrics_binary = logger.get_metrics();

    (metrics_linear, res_linear, metrics_binary, res_binary)
}

pub fn main() {
    let out_file = "perf_lab9.csv".to_string();
    let test_cases: Vec<usize> = vec![20, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000];
    let elem_rate = 0.66666666;
    // let elem_rate = 0.1;

    let mut wtr = Writer::from_path(out_file).unwrap();
    wtr.write_record(&[
        "n",
        "linear_compares",
        "linear_shifts",
        "linear_time",
        "binary_compares",
        "binary_shifts",
        "binary_time",
    ])
    .unwrap();

    for n in test_cases {
        let elem = (elem_rate * (n as f64)).round() as i64;
        let (m1, r1, m2, r2) = test(n, elem);
        println!("Test Case   n = {n}   element = {elem}");
        println!("Linear Search: Found: {r1:?} -- {m1}");
        println!("Binary Search: Found: {r2:?} -- {m2}\n");

        let Metrics(c1, s1, t1) = m1;
        let Metrics(c2, s2, t2) = m2;
        wtr.write_record(&[
            n.to_string(),
            c1.to_string(),
            s1.to_string(),
            t1.as_nanos().to_string(),
            c2.to_string(),
            s2.to_string(),
            t2.as_nanos().to_string(),
        ])
        .unwrap();
    }
}
