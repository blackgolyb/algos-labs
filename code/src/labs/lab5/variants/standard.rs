use std::fmt;

use super::base::TableMethods;
use super::impl_display_for_table;

pub struct StandardTable<T> {
    data: Vec<Vec<T>>,
    default_value: T,
    columns: usize,
    rows: usize,
}

impl<T: Clone> TableMethods<T> for StandardTable<T> {
    fn get(&self, i: usize, j: usize) -> T {
        self.data[i][j].clone()
    }

    fn set(&mut self, i: usize, j: usize, elem: T) {
        if i % 2 != 0 {
            self.data[i][j] = self.default_value.clone();
            return;
        }
        self.data[i][j] = elem;
    }

    fn get_size(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

impl<T: Clone> StandardTable<T> {
    pub fn new(rows: usize, columns: usize, default: T) -> Self {
        let mut data: Vec<Vec<T>> = Vec::new();
        for _ in 0..rows {
            data.push(vec![default.clone(); columns]);
        }
        StandardTable {
            data,
            default_value: default.clone(),
            columns,
            rows,
        }
    }
}

impl_display_for_table!(StandardTable);
