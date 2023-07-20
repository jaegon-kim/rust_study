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

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
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

    let point2 = Point { x:20.0, ..point };
    println!(" -");
    println!(" point : {:?}", point);
    println!(" point2: {:?}", point2);
    println!(" point2.x: {}, point2.y: {}", point2.x, point2.y);

    let Point { x: left_edge, y: top_edge } = point;
    println!(" point : {:?}, left_edge: {}, top_edge: {}",
        point, left_edge, top_edge);

    let rect = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bot_right: point2,
    };

    println!("rect: {:?}", rect);

    let unit = Unit;

    let pair = Pair (100, 101.0);
    println!("pair: {} {}", pair.0, pair.1);

    let Pair (integer, float) = pair;
    println!("integer: {}, float: {}", integer, float);

}

