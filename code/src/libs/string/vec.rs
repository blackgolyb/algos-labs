use std::{fmt, ops::Add, str::FromStr};

use crate::libs::vector::Vector;

#[derive(Debug, Clone)]
pub struct VecString {
    vec: Vector<char>,
}

impl VecString {
    pub fn new() -> Self {
        Self { vec: Vector::new() }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn with_len(len: usize) -> Self {
        Self {
            vec: Vector::with_capacity(len),
        }
    }

    pub fn push(&mut self, c: char) {
        self.vec.push(c);
    }

    pub fn push_str(&mut self, s: &str) {
        for c in s.chars() {
            self.push(c);
        }
    }

    pub fn push_another(&mut self, s: &VecString) {
        for c in s.vec.iter() {
            self.push(c.clone());
        }
    }

    pub fn substring(&self, start: i64, end: i64) -> Self {
        Self {
            vec: self.vec.slice(start, end),
        }
    }

    pub fn full_size(&self) -> usize {
        return self.vec.full_size()
    }
}

impl Add<&str> for VecString {
    type Output = Self;

    fn add(mut self, other: &str) -> Self {
        self.push_str(other);
        self
    }
}

impl Add<&VecString> for VecString {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self {
        self.push_another(&other);
        self
    }
}

impl From<&str> for VecString {
    fn from(value: &str) -> Self {
        let mut string = Self::with_len(value.len());
        string.push_str(value);
        string
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVecStringError;

impl FromStr for VecString {
    type Err = ParseVecStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(VecString::from(s))
    }
}

impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.vec.iter() {
            write!(f, "{c}")?;
        }

        Ok(())
    }
}
