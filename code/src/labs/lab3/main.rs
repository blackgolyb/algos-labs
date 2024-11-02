use std::{
    fmt::{Debug, Display},
    io::{self, Write},
    str::FromStr,
};

use crate::libs::bytes::{print_byte, print_bytes, print_raw_bytes, ParsedItem, ParsedVec, ShowBytes};

fn test_vec<T: Display + ShowBytes>(vec: &Vec<T>) {
    print!("addr: {:p} (", vec);
    let addr = vec as *const Vec<T> as usize;
    print_raw_bytes(addr, 8);
    // print_bytes(&addr, 8);
    println!(")");

    let size = size_of::<Vec<T>>();
    println!("bytes size: {}", size);

    print!("capacity: {} (\x1b[34m", vec.capacity());
    vec.capacity().show_bytes();
    println!("\x1b[0m)");

    print!("length: {} (\x1b[32m", vec.len());
    vec.len().show_bytes();
    println!("\x1b[0m)");

    print!("ptr: \x1b[31m");
    (vec.as_ptr() as usize).show_bytes();
    println!("\x1b[0m");

    let ptr = vec as *const Vec<T> as *const u8;
    print!("bin: \x1b[34m");
    print_bytes(ptr, 8);
    print!(" \x1b[31m");
    print_bytes(unsafe { ptr.add(8) }, 8);
    print!(" \x1b[32m");
    print_bytes(unsafe { ptr.add(16) }, 8);
    println!("\x1b[0m");

    if vec.len() == 0 {
        return;
    }

    let elem_size = size_of::<T>();
    let size = elem_size * vec.capacity();
    let actual_size = elem_size * vec.len();
    let ptr = vec.as_ptr() as *const u8;

    print!("Inner: ");
    for i in 0..size {
        if i >= actual_size {
            print!("\x1b[90m");
        }
        if i % elem_size == 0 && i != 0 {
            print!("| ");
        }
        print_byte(unsafe { ptr.add(i) });
        print!(" ");
    }
    println!("\x1b[0m");

    for (index, item) in vec.iter().enumerate() {
        print!("Element {index}: ");
        print!("(");
        let addr = item as *const T as usize;
        print_raw_bytes(addr, 8);
        print!(") ");
        item.show_bytes();
        println!();
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
            println!();
        }
        Err(_) => eprintln!("Error parsing input: {buffer}"),
    }
}

fn test_item() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    let p1: ParsedItem<u32> = ParsedItem::Value(!0);
    let p2: ParsedItem<u32> = ParsedItem::NestedVec(vec.into());
    println!("ParsedItem<u32>: {}   u32: {}\n", size_of::<ParsedItem<u32>>(), size_of::<u32>());

    let size = size_of::<ParsedItem<u32>>();
    print!("ParsedItem::Value: ");
    print_bytes(&p1 as *const ParsedItem<u32> as *const u8, size);
    println!("\n");
    print!("ParsedItem::NestedVec: ");
    print_bytes(&p2 as *const ParsedItem<u32> as *const u8, size);
    println!("\n");
}

pub fn main() {
    // test_value::<i16>("Enter short int value: ");
    // test_value::<f64>("Enter double value: ");
    // test_value::<char>("Enter char value: ");
    // test_value::<ParsedVec<usize>>("Enter array value: ");

    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // test_vec(&vec);

    // test_item();

}

// [[1  ,2] , [  3 , 4]    ]

// [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
// [[[1], [2], [3]], [[4], [5], [6]], [[7], [8], [9]]]
// [[1], [4], [7]]
// [1, [2, 3], [], [4, [5, 6]]]
// [1,   [   2   , 3], [     ], [], [4, [5 ,6]], [[7 , 8], 9]   ,10]

// 00000000 00000000 01010110 00001110 00111111 01110010 11011100 00011000
// 00000000 00000000 01010110 00001110 00111111 01110010 11011100 00000000
// 00000000 11011100 01110010 00111111 00001110 01010110 00000000 00000000

// 00000100 00000000 00000000 00000000 00000000 00000000 00000000 00000000 01110000 11011100 01110010 00111111 00001110 01010110 00000000 00000000 00000010 00000000 00000000 00000000 00000000 00000000 00000000 00000000
// 00000100 00000000 00000000 00000000 00000000 00000000 00000000 00000000 01110000 11011100 01110010 00111111 00001110 01010110 00000000 00000000 00000010 00000000 00000000 00000000 00000000 00000000 00000000 00000000[2, [1, 2], [], [1, [2, 3]]]
