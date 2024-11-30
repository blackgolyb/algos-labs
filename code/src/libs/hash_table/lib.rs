use std::{
    cell::RefCell,
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

type EntryRef<K, V> = Rc<RefCell<Entry<K, V>>>;

struct Entry<K, V>
where
    K: Hash + Eq,
{
    key: K,
    value: Rc<RefCell<V>>,
    next: Option<EntryRef<K, V>>,
}

pub struct HashTable<K, V>
where
    K: Hash + Eq,
{
    buckets: Vec<Option<EntryRef<K, V>>>,
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self::new_with_capacity(255)
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            buckets: vec![None; capacity],
        }
    }

    pub fn capacity(&self) -> usize {
        self.buckets.len()
    }

    fn new_element(key: K, value: V) -> EntryRef<K, V> {
        Rc::new(RefCell::new(Entry {
            key,
            value: Rc::new(RefCell::new(value)),
            next: None,
        }))
    }

    pub fn get_index(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        let mut initial_hash = hasher.finish();

        let base: u64 = 3;
        let mut hash = 0;
        let mut i = 0;

        while initial_hash > 0 {
            hash += (initial_hash % 10) * base.pow(i);
            initial_hash /= 10;
            i += 1;
        }

        (hash % self.capacity() as u64) as usize
    }

    pub fn contains(&self, key: K) -> bool {
        let index = self.get_index(&key);
        let elem = &self.buckets[index];
        if elem.is_none() {
            return false;
        }

        let mut elem = elem.clone();

        loop {
            let e = elem.as_ref().unwrap();
            if e.borrow().key == key {
                return true;
            }

            if e.borrow().next.is_none() {
                return false;
            }

            let next = e.borrow_mut().next.clone();
            elem = next;
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.get_index(&key);
        let elem = &mut self.buckets[index];

        if elem.is_none() {
            self.buckets[index] = Some(Self::new_element(key, value));
            return;
        }

        let mut elem = elem.clone();

        loop {
            let e = elem.as_ref().unwrap();
            if e.borrow().key == key {
                *e.borrow_mut().value.borrow_mut() = value;
                return;
            }

            if e.borrow().next.is_none() {
                e.borrow_mut().next = Some(Self::new_element(key, value));
                return;
            }

            let next = e.borrow_mut().next.clone();
            elem = next;
        }
    }

    pub fn get(&self, key: K) -> Option<Rc<RefCell<V>>> {
        let index = self.get_index(&key);
        let elem = &self.buckets[index];
        if elem.is_none() {
            return None;
        }

        let mut elem = elem.clone();

        loop {
            let e = elem.as_ref().unwrap();
            if e.borrow().key == key {
                return Some(e.borrow().value.clone());
            }

            if e.borrow().next.is_none() {
                return None;
            }

            let next = e.borrow_mut().next.clone();
            elem = next;
        }
    }
}

impl<K, V> fmt::Display for HashTable<K, V>
where
    K: Hash + Eq,
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.buckets.len() {
            if self.buckets[i].is_none() {
                continue;
            }
            let mut elem = (&self.buckets[i]).clone();

            let mut pos = 0;

            loop {
                let e = elem.as_ref().unwrap();
                write!(
                    f,
                    "{key} => {value}   |   [ buckets id: {i};\t linked list position: {pos} ]\n",
                    key = e.borrow().key,
                    value = e.borrow().value.borrow()
                )?;

                if e.borrow().next.is_none() {
                    break;
                }

                let next = e.borrow_mut().next.clone();
                elem = next;
                pos += 1;
            }
        }
        Ok(())
    }
}
