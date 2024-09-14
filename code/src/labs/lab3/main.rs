use core::fmt;
use std::{
    fmt::Display,
    io::{self, Write},
    str::FromStr,
};

use crate::libs::bytes::{impl_show_bytes, show, ShowBytes};

impl_show_bytes!(i8);
impl_show_bytes!(i16);
impl_show_bytes!(i32);
impl_show_bytes!(i64);
impl_show_bytes!(u8);
impl_show_bytes!(f64);

impl ShowBytes for char {
    fn show_bytes(&self) {
        let s = self.to_string();
        let bytes = s.as_bytes();
        for byte in bytes {
            for j in 0..8 {
                print!("{}", (byte >> (7 - j)) & 1);
            }
            print!(" ");
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseVecError;

#[derive(Debug)]
struct ParsedVec<T>(Vec<T>);

impl<T: FromStr> FromStr for ParsedVec<T> {
    type Err = ParseVecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or(ParseVecError)?;

        let vec = s
            .split(',')
            .map(|token| token.trim().parse::<T>())
            .collect::<Result<Vec<T>, _>>()
            .map_err(|_| ParseVecError)?;

        Ok(ParsedVec(vec))
    }
}

impl<T: Display> Display for ParsedVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content: String = self
            .0
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", content)
    }
}

impl<T: ShowBytes> ShowBytes for ParsedVec<T> {
    fn show_bytes(&self) {
        let elem_size = size_of::<T>();
        let size = elem_size * self.0.capacity();
        let actual_size = elem_size * self.0.len();
        let ptr = self.0.as_ptr() as *const u8;

        for i in 0..size {
            if i >= actual_size {
                print!("\x1b[90m");
            }
            if i % elem_size == 0 && i != 0 {
                print!("| ");
            }
            let byte = unsafe { *ptr.add(i) };
            for j in 0..8 {
                print!("{}", (byte >> (7 - j)) & 1);
            }
            print!(" ");
        }
        println!("\x1b[0m");

        for (index, item) in self.0.iter().enumerate() {
            print!("Element {}: ", index);
            item.show_bytes();
        }
    }
}

fn test_value<T: FromStr + Display + ShowBytes>(prompt: &str) {
    let mut buffer = String::new();
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    match buffer.trim().parse::<T>() {
        Ok(value) => {
            println!("Entered value: {value}");
            print!("Bin: ");
            value.show_bytes();
        }
        Err(_) => eprintln!("Error parsing input: {buffer}"),
    }
}

pub fn main() {
    test_value::<i16>("Enter short int value: ");
    test_value::<f64>("Enter double value: ");
    test_value::<char>("Enter char value: ");
    test_value::<ParsedVec<i16>>("Enter array value: ");
}
