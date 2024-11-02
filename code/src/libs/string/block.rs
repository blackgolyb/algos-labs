use std::fmt;
use std::mem;
use std::ops::Add;
use std::str::FromStr;

use crate::libs::list::double_linked_list::Node;

use super::super::list::double_linked_list::List;
use super::super::utils::clamp_range;
use super::vec::VecString;

#[derive(Clone)]
enum Split {
    Sized(usize),
    Symbol(char),
}

pub struct BlockString {
    list: List<VecString>,
    split: Split,
    len: usize,
    current_block: usize,
}

impl BlockString {
    pub fn new_with_symbol_block(split_char: char) -> Self {
        Self::new_raw(Split::Symbol(split_char))
    }

    pub fn new_with_sized_block(size: usize) -> Self {
        Self::new_raw(Split::Sized(size))
    }

    fn new_raw(split: Split) -> Self {
        let mut list = List::new();
        list.push(VecString::new());
        Self {
            list,
            split,
            len: 0,
            current_block: 0,
        }
    }

    pub fn push(&mut self, c: char) {
        match self.split {
            Split::Sized(size) => {
                let block = self.list.get_mut(self.current_block as i64).unwrap();
                if block.len() + 1 > size {
                    let mut v = VecString::new();
                    v.push(c);
                    self.list.push(v);
                    self.current_block += 1;
                } else {
                    block.push(c);
                }
            }
            Split::Symbol(symbol) => {
                if symbol == c {
                    self.list.push(VecString::new());
                    self.current_block += 1;
                } else {
                    self.list
                        .get_mut(self.current_block as i64)
                        .unwrap()
                        .push(c);
                }
            }
        };
        self.len += 1;
    }

    pub fn push_block(&mut self, block: VecString) {
        self.len += block.len();
        self.list.push(block);
        self.current_block += 1;
    }

    pub fn push_str(&mut self, s: &str) {
        for c in s.chars() {
            self.push(c);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn substring(&self, start: i64, end: i64) -> Self {
        let (start, end) = clamp_range(self.len(), start, end);

        let mut res = Self {
            list: List::new(),
            split: self.split.clone(),
            len: 0,
            current_block: 0,
        };

        if start >= end {
            res.list.push(VecString::new());
            return res;
        }

        let is_sized = match self.split {
            Split::Sized(_) => true,
            Split::Symbol(_) => false,
        };
        let mut block_i = 0;
        let mut i = 0;
        let mut blocks_len = 0;
        for total in 0..end {
            let block = self.list.get(block_i).unwrap();
            if i >= block.len() as i64 {
                if total >= start {
                    let start = (start - blocks_len).max(0);
                    res.push_block(block.substring(start, -1));
                }

                block_i += 1;
                if is_sized {
                    blocks_len += i;
                    i = 0;
                } else {
                    blocks_len += i + 1;
                    i = -1;
                };
            }

            i += 1;
        }

        if blocks_len <= end {
            let block = self.list.get(block_i).unwrap();
            let start = (start - blocks_len).max(0);
            let end = end - blocks_len - 1;
            if !is_sized && end == -1 {
                res.push_block(VecString::new());
            } else {
                res.push_block(block.substring(start, end));
            }
        }

        res
    }

    pub fn full_size(&self) -> usize {
        let mut size = mem::size_of::<Self>();
        let len = self.list.len();
        for i in 0..len {
            size += self.list.get(i as i64).unwrap().full_size();
            size += mem::size_of::<*mut Node<VecString>>() * 2;
        }
        size
    }
}

impl Add<&str> for BlockString {
    type Output = Self;

    fn add(mut self, other: &str) -> Self {
        self.push_str(other);
        self
    }
}

impl From<&str> for BlockString {
    fn from(value: &str) -> Self {
        let mut string = Self::new_with_sized_block(255);
        string.push_str(value);
        string
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBlockStringError;

impl FromStr for BlockString {
    type Err = ParseBlockStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BlockString::from(s))
    }
}

impl fmt::Display for BlockString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.list.len();
        for i in 0..len {
            self.list.get(i as i64).unwrap().fmt(f)?;
            match self.split {
                Split::Symbol(c) if i < (len - 1) => write!(f, "{c}")?,
                _ => (),
            };
            // write!(f, "|")?
        }

        Ok(())
    }
}
