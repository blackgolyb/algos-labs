use crate::libs::sort::{testing::test_perf, variants::BubbleSort};

// impl AsIndex for i64 {
//     fn as_index(&self) -> usize {
//         (self / 10) as usize
//     }
// }

pub fn main() {
    let test_cases: Vec<usize> = vec![20, 1000, 5000, 10000, 50000];
    let mut sort = BubbleSort::new();
    let file_name = "bubble.csv".to_string();

    test_perf(&mut sort, file_name, test_cases);
}
