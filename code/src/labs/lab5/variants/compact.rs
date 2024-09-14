use std::fmt;

use super::base::TableMethods;
use super::impl_display_for_table;

pub struct CompactTable<T> {
    data: Vec<T>,
    default_value: T,
    columns: usize,
    rows: usize,
}

impl<T: Clone> TableMethods<T> for CompactTable<T> {
    fn get(&self, i: usize, j: usize) -> T {
        if i % 2 != 0 {
            return self.default_value.clone();
        }
        self.data[(i / 2) * self.columns + j].clone()
    }

    fn set(&mut self, i: usize, j: usize, elem: T) {
        if i % 2 != 0 {
            return;
        }
        self.data[(i / 2) * self.columns + j] = elem;
    }

    fn get_size(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

impl<T: Clone> CompactTable<T> {
    pub fn new(rows: usize, columns: usize, default: T) -> Self {
        CompactTable {
            data: vec![default.clone(); columns * rows.div_ceil(2)],
            default_value: default.clone(),
            columns,
            rows,
        }
    }
}

impl_display_for_table!(CompactTable);
