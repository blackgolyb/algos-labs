use std::error::Error;
use std::time::Instant;

use csv::Writer;

use super::variants::{effective, iterational, recursive};

fn timeit(f: Box<dyn Fn() -> ()>) -> String {
    let now = Instant::now();
    f();
    now.elapsed().as_nanos().to_string()
}

fn perf_pascale_triangle(out_file: String, limit: u64) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(out_file)?;
    wtr.write_record(&["n", "effective", "iterational", "recursive"])?;
    for n in 0..=limit {
        let e = timeit(Box::new(move || effective::pascale_triangle(n as usize)));
        let i = timeit(Box::new(move || iterational::pascale_triangle(n)));
        let r = timeit(Box::new(move || recursive::pascale_triangle(n)));
        wtr.write_record(&[n.to_string(), e, i, r])?;
    }
    Ok(())
}

pub fn main() {
    perf_pascale_triangle("perf_lab2.csv".to_string(), 33).unwrap();
}
