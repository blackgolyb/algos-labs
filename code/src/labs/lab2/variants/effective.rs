pub fn pascale_triangle(n: usize) {
    let mut line = vec![1; n];
    let mut buffer = vec![1; n];

    if n == 0 {
        return;
    }

    println!("{}", buffer[0]);

    for i in 1..n {
        print!("{} ", buffer[0]);
        for j in 1..i {
            buffer[j] = line[j - 1] + line[j];
            print!("{} ", buffer[j]);
        }
        println!("{}", buffer[i]);
        (line, buffer) = (buffer, line);
    }
}
