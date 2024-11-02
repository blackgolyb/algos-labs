fn binome(n: u64, k: u64) -> u64 {
    if n == 0 || k == 0 || k == n {
        return 1;
    }
    binome(n - 1, k - 1) + binome(n - 1, k)
}

pub fn pascale_triangle(n: u64) {
    fn draw(n: u64, line: u64, i: u64) {
        if line == n {
            return;
        }

        let t = binome(line, i);
        print!("{} ", t);

        if i == line {
            println!();
            draw(n, line + 1, 0);
        } else {
            draw(n, line, i + 1);
        }
    }
    draw(n, 0, 0);
}
