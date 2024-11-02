use std::fmt::Display;

pub struct Logger {
    time: std::time::Instant,
    compare_count: usize,
    shift: usize,
    result_time: std::time::Duration,
}

pub struct Metrics(pub usize, pub usize, pub std::time::Duration);

impl Logger {
    pub fn new() -> Self {
        Self {
            time: std::time::Instant::now(),
            compare_count: 0,
            shift: 0,
            result_time: std::time::Duration::new(0, 0),
        }
    }

    pub fn log_compare(&mut self) {
        self.compare_count += 1;
    }

    pub fn log_shift(&mut self) {
        self.shift += 1;
    }

    pub fn start(&mut self) {
        self.time = std::time::Instant::now();
        self.compare_count = 0;
        self.shift = 0;
    }

    pub fn end(&mut self) {
        self.result_time = self.time.elapsed();
    }

    pub fn get_metrics(&self) -> Metrics {
        Metrics(self.compare_count, self.shift, self.result_time)
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Comparisons: {}, Shifts: {}, Execution Time: {:?}",
            self.0, self.1, self.2
        )
    }
}
