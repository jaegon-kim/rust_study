#![allow(unused)]

use std::mem;
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point {x: 0.0, y: 0.0}
}


fn boxed_origin() -> Box<Point> {
    Box::new(Point {x: 0.0, y:0.0})
}


pub fn test_box() {
    println!("Test box");

    let point: Point = origin();
    println!("point {:?} (size:{})", point, mem::size_of_val(&point));

    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point {x: 0.0, y: -4.0}
    };
    println!("rectangle {:?} (size:{})", rectangle, mem::size_of_val(&rectangle));

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
            top_left: origin(),
            bottom_right: Point {x: 3.0, y: -4.0}
        }
    );
    println!("rectangle {:?} (size:{})", boxed_rectangle, mem::size_of_val(&boxed_rectangle));

    let boxed_point: Box<Point> = Box::new(origin());
    println!("boxed_point {:?} (size:{})", boxed_point, mem::size_of_val(&boxed_point));

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
    println!("box_in_a_box {:?} (size:{})", box_in_a_box, mem::size_of_val(&box_in_a_box));

    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point {:?} (size:{})", unboxed_point, mem::size_of_val(&unboxed_point));


}
