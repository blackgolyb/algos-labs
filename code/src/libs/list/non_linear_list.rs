use std::fmt::Debug;
use std::{fmt, vec};
use std::{fmt::Display, ptr};

use super::double_linked_list::List;

#[derive(Debug)]
struct Node<T>(Option<T>, *mut List<Node<T>>);

#[derive(Debug)]
pub struct MultiList<T> {
    root: Node<T>,
    dimentions: Option<usize>,
    limits: Vec<Option<usize>>,
}

impl<T: Debug> MultiList<T> {
    pub fn new(dimentions: Option<usize>, limits: Option<Vec<Option<usize>>>) -> MultiList<T> {
        let (dimentions, limits) = match (dimentions, limits) {
            (None, None) => (None, Vec::new()),
            (None, Some(limits)) => (None, limits),
            (Some(dimentions), None) => {
                Self::validate_dimantions(dimentions, 0);
                (None, Vec::new())
            },
            (Some(dimentions), Some(limits)) => {
                Self::validate_dimantions(dimentions, limits.len());
                (Some(dimentions), limits)
            }
        };

        Self {
            root: Self::new_list_node(),
            dimentions,
            limits,
        }
    }

    fn validate_dimantions(dimentions:usize, limits: usize) {
        if dimentions == 0 {
            panic!("Dimentions must be greater than zero");
        }

        if dimentions > limits {
            panic!("Dimentions must be less than or equal to the limits vector length");
        }
    }

    fn new_list_node() -> Node<T> {
        Node(None, Box::into_raw(Box::new(List::<Node<T>>::new())))
    }

    pub fn push(&mut self, value: T, indices: &[usize]) -> bool {
        unsafe {
            let mut current_list = &mut self.lists;

            for i in 0..(self.dimentions - 1) as usize {
                if indices[i] >= self.limits[i] {
                    return false;
                }

                let tmp = (*current_list.1).get(indices[i] as i64);
                if tmp.is_none() {
                    return false;
                }
                current_list = tmp.unwrap();
            }

            (*current_list.1).push(Node(Some(value), ptr::null_mut()));
        }

        true
    }

    pub fn get(&self, indices: &[usize]) -> Option<&T> {
        unsafe {
            let mut current_list = &self.lists;

            for i in 0..self.dimentions as usize {
                if indices[i] >= self.limits[i] {
                    return None;
                }

                let tmp = (*current_list.1).get(indices[i] as i64);
                if tmp.is_none() {
                    return None;
                }
                current_list = tmp.unwrap();
            }

            current_list.0.as_ref()
        }
    }

    pub fn remove(&mut self, indices: &[usize]) -> Option<T> {
        unsafe {
            let mut current_list = &mut self.lists;

            for i in 0..(self.dimentions - 1) as usize {
                if indices[i] >= self.limits[i] {
                    return None;
                }

                current_list = (*current_list.1).get(indices[i] as i64).unwrap();
            }

            let node = (*current_list.1).remove(indices[self.dimentions as usize - 1] as i64);

            return match node {
                None => None,
                Some(node) => Some(node.0.unwrap()),
            };
        }
    }
}

impl<T: Display + Debug> MultiList<T> {
    fn display_one_list(&self, f: &mut fmt::Formatter<'_>, mut indices: Vec<usize>) -> fmt::Result {
        let len = self.limits[self.limits.len() - 1];
        let d = self.dimentions as usize - 1;

        write!(f, "[")?;
        for i in 0..d - 1 {
            write!(f, "{}, ", indices[i])?;
        }
        if d != 0 {
            write!(f, "{}", indices[d - 1])?;
        }
        write!(f, "]: ")?;

        indices.push(0);
        let mut elem: Option<&T> = None;
        let mut prev: Option<&T> = None;
        for i in 0..=len {
            if prev.is_some() {
                write!(f, "{}, ", prev.unwrap())?;
            }
            indices[self.dimentions as usize - 1] = i;
            prev = elem;
            elem = self.get(indices.as_slice());
            if elem.is_none() {
                break;
            }
        }
        if prev.is_some() {
            write!(f, "{}", prev.unwrap())?;
        }
        write!(f, "\n")?;
        Ok(())
    }

    fn display(
        &self,
        f: &mut fmt::Formatter<'_>,
        indices: &mut Vec<usize>,
        level: u8,
    ) -> fmt::Result {
        if level >= self.dimentions - 1 {
            self.display_one_list(f, indices.clone())?;
            return Ok(());
        }

        for i in 0..self.limits[level as usize] {
            indices[level as usize] = i;
            self.display(f, indices, level + 1)?;
        }

        Ok(())
    }
}

impl<T: Display + Debug> Display for MultiList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut indices = vec![0 as usize; self.dimentions as usize - 1];

        self.display(f, &mut indices, 0)?;
        return Ok(());
    }
}
