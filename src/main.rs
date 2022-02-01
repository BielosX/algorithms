mod graph;
mod heap;
mod list;
mod tree;

use std::io::{stdin, stdout, Write};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut red_black_tree = tree::RedBlackTree::new();
    for _ in 0..100 {
        let mut value = rng.gen_range(0..1000);
        while red_black_tree.contains(value) {
            value = rng.gen_range(0..1000);
        }
        red_black_tree.insert(value);
    }
    let mut guessed = false;
    while !guessed {
        let mut input = String::new();
        print!("Your guess: ");
        stdout().flush();
        stdin().read_line(&mut input).expect("Enter correct string");
        println!("Provided: {}", input);
        let value: i32 = input.trim().parse().expect("Integer value expected");
        if red_black_tree.contains(value) {
            guessed = true;
            println!("Great");
        } else {
            println!("Nope, try again");
        }
    }
}
