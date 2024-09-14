use std::fmt::Display;
use std::time::Instant;

use super::variants::base::TableMethods;
use super::variants::compact::CompactTable;
use super::variants::standard::StandardTable;

fn table_test<T: TableMethods<i32> + Display>(title: &str, mut table: T) {
    println!("{:=^60}", title);
    let cyles = 1000;
    let (rows, columns) = table.get_size();

    for i in 0..rows {
        for j in 0..columns {
            table.set(i, j, (i + j) as i32);
        }
    }
    println!("{}", table);

    // odd
    let mut read: u128 = 0;
    let mut write: u128 = 0;
    for _ in 0..cyles {
        for i in 0..rows {
            if i % 2 == 0 {
                continue;
            }
            for j in 0..columns {
                let now = Instant::now();
                table.set(i, j, (i + j) as i32);
                write += now.elapsed().as_nanos();
            }
        }
    }
    for _ in 0..cyles {
        for i in 0..rows {
            if i % 2 == 0 {
                continue;
            }
            for j in 0..columns {
                let now = Instant::now();
                table.get(i, j);
                read += now.elapsed().as_nanos();
            }
        }
    }
    println!("Odd rows Times (Read/Write): {} ns / {} ns", read, write);

    // even
    let mut read: u128 = 0;
    let mut write: u128 = 0;
    for _ in 0..cyles {
        for i in 0..rows {
            if i % 2 != 0 {
                continue;
            }
            for j in 0..columns {
                let now = Instant::now();
                table.set(i, j, (i + j) as i32);
                write += now.elapsed().as_nanos();
            }
        }
    }
    for _ in 0..cyles {
        for i in 0..rows {
            if i % 2 != 0 {
                continue;
            }
            for j in 0..columns {
                let now = Instant::now();
                table.get(i, j);
                read += now.elapsed().as_nanos();
            }
        }
    }
    println!("Even rows Times (Read/Write): {} ns / {} ns", read, write);
    println!("{:=^60}\n", "");
}

pub fn main() {
    let rows = 6;
    let columns = 10;

    table_test("Compact Table", CompactTable::<i32>::new(rows, columns, 0));
    table_test(
        "Starndard Table",
        StandardTable::<i32>::new(rows, columns, 0),
    );
}
