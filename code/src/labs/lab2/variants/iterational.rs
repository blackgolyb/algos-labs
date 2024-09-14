fn binome(n: u64, k: u64) -> u64 {
    let mut res = 1;
    for i in 0..k {
        res = res * (n - i) / (i + 1);
    }
    res
}

pub fn pascale_triangle(n: u64) {
    let mut t: u64;
    for line in 0..n {
        for i in 0..=line {
            t = binome(line, i);
            print!("{} ", t);
        }
        println!();
    }
}
