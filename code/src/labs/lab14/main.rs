use crate::libs::rb_tree::RBTree;

pub fn main() {
    let mut tree = RBTree::<i32, i32>::new();
    let data = [1, 2, 3, 4, 5, 6, 7];

    for i in data {
        tree.put(i * 10, i * 10);
        // println!("{tree}");
    }

    println!("{tree}");
    let h = tree.height() as i32;
    println!("tree height: {h}\n\n");
    tree.put(h, h);
    let h = tree.height();
    println!("{tree}");
    println!("tree height: {h}");
}
