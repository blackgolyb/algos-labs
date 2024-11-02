use std::{
    fmt::{self, Display},
    ptr,
};

pub struct Node<T> {
    pub value: T,
    pub prev: *mut Node<T>,
    pub next: *mut Node<T>,
}

pub struct List<T> {
    pub head: *mut Node<T>,
    pub tail: *mut Node<T>,
    len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            prev: ptr::null_mut(),
            next: self.head,
        }));

        unsafe {
            if !self.head.is_null() {
                (*self.head).prev = new_node;
            }

            self.head = new_node;

            if self.tail.is_null() {
                self.tail = new_node;
            }

            self.len += 1;
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            prev: self.tail,
            next: ptr::null_mut(),
        }));

        unsafe {
            if !self.tail.is_null() {
                (*self.tail).next = new_node;
            }

            self.tail = new_node;

            if self.head.is_null() {
                self.head = new_node;
            }

            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }

        unsafe {
            let old_tail = self.tail;
            self.tail = (*old_tail).prev;
            (*old_tail).prev = ptr::null_mut();

            if self.tail.is_null() {
                self.head = ptr::null_mut();
            } else {
                (*self.tail).next = ptr::null_mut();
            }

            self.len -= 1;
            Some(Box::from_raw(old_tail).value)
        }
    }

    pub fn remove(&mut self, mut index: i64) -> Option<T> {
        if index < 0 {
            index = self.len as i64 + index;
        }

        if index >= self.len as i64 {
            return None;
        }

        unsafe {
            let mut node = self.head;
            for _ in 0..index {
                node = (*node).next;
            }

            if !(*node).next.is_null() {
                (*(*node).next).prev = (*node).prev
            }

            if !(*node).prev.is_null() {
                (*(*node).prev).next = (*node).next
            }

            if node == self.head {
                self.head = (*node).next;
            }

            if node == self.tail {
                self.tail = (*node).prev;
            }

            self.len -= 1;
            Some(Box::from_raw(node).value)
        }
    }

    pub fn get(&self, mut index: i64) -> Option<&T> {
        if index < 0 {
            index = self.len as i64 + index;
        }

        if index >= self.len as i64 {
            return None;
        }

        unsafe {
            let mut node = self.head;
            for _ in 0..index {
                node = (*node).next;
            }
            return Some(&mut (*node).value);
        }
    }

    pub fn get_mut(&mut self, mut index: i64) -> Option<&mut T> {
        if index < 0 {
            index = self.len as i64 + index;
        }

        if index >= self.len as i64 {
            return None;
        }

        unsafe {
            let mut node = self.head;
            for _ in 0..index {
                node = (*node).next;
            }
            return Some(&mut (*node).value);
        }
    }
}

impl<T: PartialEq> List<T> {
    pub fn index(&mut self, value: T) -> Option<usize> {
        let mut node = self.head;

        unsafe {
            for i in 0..self.len {
                if (*node).value == value {
                    return Some(i);
                }
                node = (*node).next;
            }
        }

        None
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let mut node = self.head;
            for _ in 0..self.len {
                write!(f, "{} ", (*node).value)?;
                node = (*node).next;
            }
        }
        Ok(())
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
