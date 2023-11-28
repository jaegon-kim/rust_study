#![allow(unused)]

pub fn test_expression() {
    println!("test_expression");

    let x = 5; // variable binding
    x;
    x + 1;
    15;

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

