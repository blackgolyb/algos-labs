use csv::Writer;

use super::data::*;
use crate::libs::hash_table::HashTable;
use crate::libs::list::double_linked_list::List;
use crate::libs::search::binary_search;
use crate::libs::search::linear_search;
use crate::libs::search::logger::{Logger, Metrics};

pub fn test() {
    let n = 10;
    let file_name = format!("data_{n}.json");

    // let mut file = std::fs::File::create(&file_name).expect("Unable to create file");
    // generate_test_file(&mut file, n);

    // let mut file = std::fs::File::open(&file_name).expect("Unable to create file");
    // let mut data_vec = read_data_from_file(&mut file);

    let mut data_vec = generate_unique_data(n);
    // let mut data_vec = default_unique_data(n);
    println!("generated\n");

    let mut hash_table = HashTable::<Ipv4Address, Data>::new_with_capacity(n + 1);
    let mut list = List::<Data>::new();

    data_vec.sort();

    for data in data_vec {
        hash_table.insert(data.ip, data);
        list.push(data);
    }

    println!("{}", hash_table);
}

fn perf_test(n: usize, elem_rate: f64) -> (Metrics, Metrics, Metrics) {
    let elem_id = (n as f64 * elem_rate) as usize;
    let mut data_vec = generate_unique_data(n);

    let mut hash_table = HashTable::<Ipv4Address, Data>::new_with_capacity(n + 1);
    let mut list = List::<Data>::new();

    data_vec.sort();
    let elem = data_vec[elem_id];

    for data in data_vec {
        hash_table.insert(data.ip, data);
        list.push(data);
    }

    let mut logger = Logger::new();

    let res1 = linear_search(&mut list, elem, &mut logger);
    let m1 = logger.get_metrics();

    let res2 = binary_search(&mut list, elem, &mut logger);
    let m2 = logger.get_metrics();

    logger.start();
    let res3 = hash_table.get(elem.ip);
    logger.end();
    let m3 = logger.get_metrics();

    (m1, m2, m3)
}

fn perf() {
    let out_file = "perf_lab10.csv".to_string();
    let test_cases: Vec<usize> = vec![20, 1000, 5000, 10000, 50000];
    let elem_rate = 0.66666666;
    // let elem_rate = 0.1;

    let mut wtr = Writer::from_path(out_file).unwrap();
    wtr.write_record(&[
        "n",
        "linear_compares",
        "linear_time",
        "binary_compares",
        "binary_time",
        "hash_compares",
        "hash_time",
    ])
    .unwrap();

    for n in test_cases {
        let (m1, m2, m3) = perf_test(n, elem_rate);
        println!("Linear Search: {m1}");
        println!("Binary Search: {m2}");
        println!("Hash table: {m3}\n");

        let Metrics(c1, s1, t1) = m1;
        let Metrics(c2, s2, t2) = m2;
        let Metrics(c3, s3, t3) = m3;
        wtr.write_record(&[
            n.to_string(),
            c1.to_string(),
            t1.as_nanos().to_string(),
            c2.to_string(),
            t2.as_nanos().to_string(),
            c3.to_string(),
            t3.as_nanos().to_string(),
        ])
        .unwrap();
    }
}

pub fn main() {
    test();
    // perf();
}
