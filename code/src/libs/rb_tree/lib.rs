use super::node::{TreeNode, TreeNodePtr};
use std::cmp::Ordering;

pub struct RBTree<K: Ord, V> {
    pub(super) root: TreeNodePtr<K, V>,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        RBTree {
            root: TreeNodePtr::None,
        }
    }

    pub fn height(&self) -> usize {
        self.root.height()
    }

    pub fn put(&mut self, key: K, value: V) {
        self.root = RBTree::put_node(&mut self.root, key, value);
        if let TreeNodePtr::Some(h) = &mut self.root {
            h.is_red = false; //Tree root is always black
        }
    }

    pub fn get(&self, search_key: K) -> Option<&V> {
        let mut current = &self.root;
        while let TreeNodePtr::Some(node) = current {
            match search_key.cmp(&node.key) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return Some(&node.value),
            }
        }
        None
    }

    fn put_node(node: &mut TreeNodePtr<K, V>, new_key: K, new_value: V) -> TreeNodePtr<K, V> {
        match node {
            TreeNodePtr::None => {
                return TreeNodePtr::Some(Box::new(TreeNode::create(new_key, new_value)))
            }
            TreeNodePtr::Some(node) => match new_key.cmp(&node.key) {
                Ordering::Less => {
                    node.left = RBTree::put_node(&mut node.left, new_key, new_value);
                }
                Ordering::Greater => {
                    node.right = RBTree::put_node(&mut node.right, new_key, new_value);
                }
                Ordering::Equal => node.value = new_value,
            },
        }
        // Red-Black Tree Balancing
        if node.right().is_red() && !node.left().is_red() {
            //Case 2
            *node = RBTree::rotate_left(node.take());
        }
        if node.left().is_red() && node.left().left().is_red() {
            //Case 4
            *node = RBTree::rotate_right(node.take());
        }
        if node.left().is_red() && node.right().is_red() {
            //Case 3, 4
            RBTree::flip_colors(node);
        }
        node.take()
    }

    fn rotate_left(mut node: TreeNodePtr<K, V>) -> TreeNodePtr<K, V> {
        debug_assert!(node.right().is_red());
        if let TreeNodePtr::Some(h) = &mut node {
            let mut right_node = h.right.take();
            if let TreeNodePtr::Some(x) = &mut right_node {
                x.is_red = h.is_red;
                h.is_red = true;

                h.right = x.left.take();
                x.left = node;
            }
            return right_node;
        }
        node
    }

    fn rotate_right(mut node: TreeNodePtr<K, V>) -> TreeNodePtr<K, V> {
        debug_assert!(node.left().is_red());
        if let TreeNodePtr::Some(h) = &mut node {
            let mut left_node = h.left.take();
            if let TreeNodePtr::Some(x) = &mut left_node {
                x.is_red = h.is_red;
                h.is_red = true;
                h.left = x.right.take();
                x.right = node;
            }
            return left_node;
        }
        node
    }

    fn flip_colors(node: &mut TreeNodePtr<K, V>) {
        debug_assert!(!node.is_red());
        debug_assert!(node.left().is_red());
        debug_assert!(node.right().is_red());
        if let TreeNodePtr::Some(h) = node {
            h.is_red = true;
            if let TreeNodePtr::Some(left) = &mut h.left {
                left.is_red = false;
            }
            if let TreeNodePtr::Some(right) = &mut h.right {
                right.is_red = false;
            }
        }
    }
}

impl<K: Ord, V> Default for RBTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
