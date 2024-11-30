use core::fmt;
use std::fmt::Write;

use super::lib::RBTree;
use super::node::{TreeNode, TreeNodePtr};

impl<K: Ord + fmt::Display, V> fmt::Display for RBTree<K, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.root {
            TreeNodePtr::Some(node) => write!(formatter, "{}", &node),
            _ => write!(formatter, "Empty"),
        }
    }
}

fn write_layers(layers: &Vec<bool>, formatter: &mut fmt::Formatter, pos: u8) -> fmt::Result {
    let n = layers.len();
    let mut s: String = String::new();
    if n > 1 {
        s = layers[..n - 1]
            .iter()
            .map(|l| match l {
                true => "│   ",
                false => "    ",
            })
            .collect();
    }
    if n != 0 {
        s += match pos {
            1 => "├───",
            2 => "└───",
            _ => "",
        }
    }
    write!(formatter, "{}", s)
}

fn write_nil(layers: &Vec<bool>, formatter: &mut fmt::Formatter, pos: u8) -> fmt::Result {
    write_layers(layers, formatter, pos)?;
    writeln!(formatter, "\x1b[40mNIL\x1b[0m")
}

impl<K: Ord + fmt::Display, V> TreeNode<K, V> {
    fn display_subtree(
        &self,
        formatter: &mut fmt::Formatter,
        levels: &mut Vec<bool>,
        position: u8,
    ) -> fmt::Result {
        let c = if self.is_red { 41 } else { 40 };
        write_layers(levels, formatter, position)?;
        writeln!(formatter, "\x1b[{}m{}\x1b[0m", c, self.key)?;

        levels.push(true);
        let n_levels = levels.len();
        if let TreeNodePtr::Some(leaf) = &self.left {
            leaf.display_subtree(formatter, levels, 1)?;
        } else {
            write_nil(levels, formatter, 1)?;
        }
        levels[n_levels - 1] = false;
        if let TreeNodePtr::Some(leaf) = &self.right {
            leaf.display_subtree(formatter, levels, 2)?;
        } else {
            write_nil(levels, formatter, 2)?;
        }

        levels.pop();
        Ok(())
    }
}

impl<K: Ord + fmt::Display, V> fmt::Display for TreeNode<K, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut levels = Vec::new();
        self.display_subtree(formatter, &mut levels, 0)
    }
}
