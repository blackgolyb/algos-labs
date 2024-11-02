pub struct Logger {
    swaps: i64,
    compares: i64,
    time: std::time::Instant,
    result_time: std::time::Duration,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            swaps: 0,
            compares: 0,
            time: std::time::Instant::now(),
            result_time: std::time::Duration::new(0, 0),
        }
    }

    pub fn log_compare(&mut self) {
        self.compares += 1;
    }

    pub fn log_swap(&mut self) {
        self.swaps += 1;
    }

    pub fn start(&mut self) {
        self.time = std::time::Instant::now();
        self.compares = 0;
        self.swaps = 0;
    }

    pub fn end(&mut self) {
        self.result_time = self.time.elapsed();
    }

    pub fn marge_sub_logger(&mut self, sub_logger: &mut Self) {
        self.compares += sub_logger.compares;
        self.swaps += sub_logger.swaps;
    }
}

pub(super) struct LoggedVec<'a, T> {
    vec: Vec<T>,
    logger: &'a mut Logger,
}
