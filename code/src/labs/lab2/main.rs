use std::error::Error;
use std::time::Instant;

use csv::Writer;

use super::variants::{effective, iterational, recursive, recursive_effective};

fn timeit(f: Box<dyn Fn() -> ()>) -> String {
    let now = Instant::now();
    for _ in 0..100 {
        f();
    }
    (now.elapsed().as_nanos() / 100).to_string()
}

fn perf_pascale_triangle(out_file: String, limit: u64) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(out_file)?;
    // wtr.write_record(&["n", "iterational_effective", "recursive_effective", "iterational", "recursive"])?;
    wtr.write_record(&["n", "iterational_effective", "recursive_effective", "iterational"])?;
    for n in 0..=limit {
        let ie = timeit(Box::new(move || effective::pascale_triangle(n as usize)));
        let i = timeit(Box::new(move || iterational::pascale_triangle(n)));
        // let r = timeit(Box::new(move || recursive::pascale_triangle(n)));
        let re = timeit(Box::new(move || recursive_effective::pascale_triangle(n)));
        wtr.write_record(&[n.to_string(), ie, re, i])?;
    }
    Ok(())
}

pub fn main() {
    // perf_pascale_triangle("perf_lab2.csv".to_string(), 34).unwrap();
    effective::pascale_triangle(15);
}
