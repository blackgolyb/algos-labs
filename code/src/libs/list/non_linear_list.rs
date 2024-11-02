use std::fmt::Debug;
use std::{fmt, mem, vec};
use std::{fmt::Display, ptr};

use super::double_linked_list::List;

#[derive(Debug)]
pub struct Node<T>(Option<T>, *mut List<Node<T>>);

#[derive(Debug)]
pub struct MultiList<T> {
    root: Node<T>,
    dimentions: Option<usize>,
    limits: Vec<Option<i64>>,
}

impl<T> MultiList<T> {
    pub fn new(
        dimentions: Option<usize>,
        limits: Option<Vec<Option<i64>>>,
    ) -> Result<MultiList<T>, String> {
        let (dimentions, limits) = match (dimentions, limits) {
            (None, None) => (None, Vec::new()),
            (None, Some(limits)) => (None, limits),
            (Some(dimentions), None) => {
                Self::validate_dimantions(dimentions, 0)?;
                (None, Vec::new())
            }
            (Some(dimentions), Some(limits)) => {
                Self::validate_dimantions(dimentions, limits.len())?;
                (Some(dimentions), limits)
            }
        };

        Ok(Self {
            root: Self::new_list_node(),
            dimentions,
            limits,
        })
    }

    fn validate_dimantions(dimentions: usize, limits: usize) -> Result<(), String> {
        if dimentions == 0 {
            return Err("Dimentions must be greater than zero".into());
        }

        if limits > dimentions {
            return Err("Dimentions must be less than or equal to the limits vector length".into());
        }

        Ok(())
    }

    fn new_list_node() -> Node<T> {
        Node(None, Box::into_raw(Box::new(List::<Node<T>>::new())))
    }

    fn new_value_node(value: T) -> Node<T> {
        Node(Some(value), ptr::null_mut())
    }

    fn new_void_node() -> Node<T> {
        Node(None, ptr::null_mut())
    }

    fn check_limits(&self, index: i64, level: usize) -> bool {
        if level >= self.limits.len() {
            return true;
        }

        match self.limits[level] {
            None => false,
            Some(limit) => index < limit,
        }
    }
}

impl<'a, T> MultiList<T> {
    fn _get_node(&self, node: &'a Node<T>, indices: Vec<i64>, level: usize) -> Option<&'a Node<T>> {
        // println!("level: {}", level);
        if level >= indices.len() {
            return Some(node);
        }

        if !self.check_limits(indices[level], level) {
            return None;
        }

        unsafe {
            if node.1.is_null() {
                return None;
            }

            let tmp = (*node.1).get_mut(indices[level] as i64);
            if tmp.is_none() {
                return None;
            }
            self._get_node(tmp.unwrap(), indices, level + 1)
        }
    }

    fn get_node(&self, indices: Vec<i64>) -> Option<&Node<T>> {
        self._get_node(&self.root, indices, 0)
    }

    pub fn get_value(&self, indices: Vec<i64>) -> Option<&T> {
        match self.get_node(indices) {
            None => None,
            Some(node) => match &node.0 {
                None => None,
                Some(value) => Some(value),
            },
        }
    }

    pub fn get_list(&self, indices: Vec<i64>) -> Option<&List<Node<T>>> {
        match self.get_node(indices) {
            None => None,
            Some(node) => {
                if node.1.is_null() {
                    return None;
                }
                unsafe { Some(&(*node.1)) }
            }
        }
    }
}

impl<T> MultiList<T> {
    fn _insert_node(
        &mut self,
        parent: &mut Node<T>,
        indices: Vec<i64>,
        node: Node<T>,
        level: usize,
        insert_value: bool,
        insert_list: bool,
    ) {
        // println!("level {}", level);
        if level >= indices.len() {
            // println!("{:?}", indices);
            // println!(">)");
            if insert_value {
                parent.0 = node.0;
            }
            if insert_list {
                parent.1 = node.1;
            }
            return;
        }

        let index = indices[level];
        if !self.check_limits(index, level) {
            return;
        }

        unsafe {
            if parent.1.is_null() {
                parent.1 = Box::into_raw(Box::new(List::<Node<T>>::new()))
            }

            let list = parent.1;
            if list.is_null() {
                return;
            }

            // println!("dopovn");
            for _ in 0..=(index - (*list).len() as i64) {
                (*list).push(Self::new_void_node());
            }

            let next = (*list).get_mut(index);
            if next.is_none() {
                return;
            }

            self._insert_node(
                next.unwrap(),
                indices,
                node,
                level + 1,
                insert_value,
                insert_list,
            )
        }
    }

    fn insert_node(
        &mut self,
        indices: Vec<i64>,
        node: Node<T>,
        insert_value: bool,
        insert_list: bool,
    ) {
        let mut root = mem::replace(&mut self.root, Self::new_list_node());
        let res = self._insert_node(&mut root, indices, node, 0, insert_value, insert_list);
        self.root = root;
        res
    }

    pub fn insert_value(&mut self, indices: Vec<i64>, value: T) {
        self.insert_node(indices, Self::new_value_node(value), true, false);
    }

    pub fn insert_list(&mut self, indices: Vec<i64>) {
        self.insert_node(indices, Self::new_list_node(), false, true);
    }

    pub fn delete_value(&mut self, indices: Vec<i64>) {
        self.insert_node(indices, Self::new_void_node(), true, false);
    }

    pub fn delete_list(&mut self, indices: Vec<i64>) {
        self.insert_node(indices, Self::new_void_node(), false, true);
    }
}

macro_rules! display_node {
    ($f:ident, $fmt_none:expr, $fmt_value:expr, $fmt_list:expr, $fmt_list_and_value:expr, $node:expr) => {
        match $node {
            Some(item) => match item {
                Node(None, list) => {
                    if list.is_null() {
                        write!($f, $fmt_none)?;
                    } else {
                        write!($f, $fmt_list)?;
                    }
                }
                Node(Some(value), list) => {
                    if list.is_null() {
                        write!($f, $fmt_value, value)?;
                    } else {
                        write!($f, $fmt_list_and_value, value)?;
                    }
                }
                _ => write!($f, $fmt_none)?,
            },
            None => write!($f, $fmt_none)?,
        };
    };
}

impl<T: Display> MultiList<T> {
    fn display_one_list(
        &self,
        f: &mut fmt::Formatter<'_>,
        node: &Node<T>,
        indices: &Vec<i64>,
    ) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..(indices.len() as i64 - 1) {
            write!(f, "{}, ", indices[i as usize])?;
        }
        if indices.len() > 0 {
            write!(f, "{}", indices[indices.len() - 1])?;
        }
        write!(f, "]: ")?;

        let list = unsafe { &mut (*node.1) };
        for i in 0..(list.len() as i64 - 1) {
            display_node!(f, "None, ", "{}, ", "[], ", "{}+[], ", list.get_mut(i));
        }

        if list.len() > 0 {
            display_node!(
                f,
                "None",
                "{}",
                "[]",
                "{}+[]",
                list.get_mut(list.len() as i64 - 1)
            );
        }

        writeln!(f)?;

        Ok(())
    }

    fn display(
        &self,
        f: &mut fmt::Formatter<'_>,
        parent: &Node<T>,
        indices: &Vec<i64>,
        level: usize,
    ) -> fmt::Result {
        if parent.1.is_null() {
            return Ok(());
        }

        self.display_one_list(f, parent, indices)?;

        let list = unsafe { &mut (*parent.1) };

        for i in 0..list.len() {
            let node = list.get_mut(i as i64);
            if node.is_none() {
                continue;
            }
            let mut new_indices = indices.to_vec();
            new_indices.push(i as i64);
            self.display(f, node.unwrap(), &new_indices, level + 1)?;
        }
        Ok(())
    }
}

impl<T: Display + Debug> Display for MultiList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, &self.root, &vec![], 0)?;
        Ok(())
    }
}
