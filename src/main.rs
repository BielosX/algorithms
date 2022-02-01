mod graph;
mod heap;
mod list;
mod tree;

use std::io::{stdin, stdout, Write};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut red_black_tree = tree::RedBlackTree::new();
    for _ in 0..10 {
        let mut value = rng.gen_range(0..1000);
        while red_black_tree.contains(value) {
            value = rng.gen_range(0..1000);
        }
        red_black_tree.insert(value);
    }
    print!("{}", red_black_tree);
}
