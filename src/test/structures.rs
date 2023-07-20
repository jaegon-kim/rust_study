#![allow(unused)]

pub fn test_structures() {
    println!("test_structures");
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair (i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_lef: Point,
    bot_right: Point,
}


