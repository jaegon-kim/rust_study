#![allow(unused)]

use rand::prelude::*;

//https://rust-random.github.io/book/guide-start.html

pub fn test_rand() {
    let x8: u8 = random();
    println!("{}", x8);

    let x64: u64 = random();
    println!("{}", x64);

}