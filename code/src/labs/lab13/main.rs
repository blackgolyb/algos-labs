use crate::libs::sort::{testing::test_perf, variants::TournamentSort};

pub fn main() {
    let test_cases: Vec<usize> = vec![20, 1000, 5000, 10000, 50000];

    let mut sort = TournamentSort::new();
    let file_name = "turnament.csv".to_string();
    test_perf(&mut sort, file_name, test_cases.clone());
}
