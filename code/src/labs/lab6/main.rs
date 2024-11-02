use std::time::Instant;
use crate::libs::string::block::BlockString;
use crate::libs::string::vec::VecString;

fn test_all_substrings(test_case: &str) {
    let len = test_case.len();

    let mut s_symbol = BlockString::new_with_symbol_block(' ');
    let mut s_sized = BlockString::new_with_sized_block(4);
    let mut s_vec = VecString::new();
    s_symbol.push_str(test_case);
    s_sized.push_str(test_case);
    s_vec.push_str(test_case);

    for i in 1..=(len + 1) as i64 {
        for j in 0..=len as i64 {
            let s = s_symbol.substring(j, -i);
            let t1 = format!("({j} {}): |{s}|", -i);
            let s = s_sized.substring(j, -i);
            let t2 = format!("({j} {}): |{s}|", -i);
            let s = s_vec.substring(j, -i);
            let t3 = format!("({j} {}): |{s}|", -i);

            println!("{t1}");
            assert_eq!(t1, t2);
            assert_eq!(t2, t3);
        }
    }
}
macro_rules! timeit_substring {
    ($string:ident, $string_src:ident, $title:expr) => {
        println!("{:=^40}", $title);
        let s = 4;
        let e = -5;
        let n = 100;

        $string.push_str($string_src);
        let substring = $string.substring(s, e);

        println!("original: |{}|", $string);
        println!("substring: |{}|", substring);
        println!("full size: {}", $string.full_size());

        let now = Instant::now();
        for _ in 0..n {
            $string.substring(s, e);
        }
        println!("PC time: {} ns  repeats: {n}", now.elapsed().as_nanos());
        println!("{:=^40}\n", "");
    };
}

pub fn main() {
    let test_case = "  01 23 45 67 89  ";
    
    let mut s_symbol = BlockString::new_with_symbol_block(' ');
    let mut s_sized = BlockString::new_with_sized_block(4);
    let mut s_vec = VecString::new();
    
    println!("\n");
    timeit_substring!(s_symbol, test_case, "BlockString(Symbol)");
    timeit_substring!(s_sized, test_case, "BlockString(Sized)");
    timeit_substring!(s_vec, test_case, "VecString");
    // test_all_substrings(test_case);
}
