#![allow(unused)]


#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair (i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_lef: Point,
    bot_right: Point,
}

pub fn test_structures() {
    println!("test_structures");

    let name = String::from("Kingkong");
    let age = 46;
    let person = Person { name, age };
    println!(" person: {:?}", person);

    let person2 = Person { name: String::from("AAA"), age: 100 };
    println!(" person: {:?}", person2);

    let point = Point { x: 10.0, y: 11.0};
    println!(" point: {:?}", point);

}

