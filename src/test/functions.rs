#![allow(unused)]

// Basic example
//https://doc.rust-lang.org/rust-by-example/fn.html

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

// Methods
// https://doc.rust-lang.org/rust-by-example/fn/methods.html

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // 'Point' associated functions
    // Associated function don't need to be called with an instance
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0}
    }

    // Associated function which have 2 parameters
    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // Method
    // `Self` = `Rectangle` ('Self' is the type of the caller object)
    // fn area(self: &Self) -> f65 // &self is sugar
    fn area(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // '&mut self' desugar to 'self: &mut Self'
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }    
} 

// 'Pair' owns resources: 2 heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // self is sugar self:Self
    fn destroy(self) {
        // Destructure 'self'
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);

    }
}

pub fn test_functions() {
    println!("test_functions");

    fizzbuzz_to(100);

    println!("test_functions_methods");

    let rectangle = Rectangle {
        // Associated functions are called using '::'
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // '&self' is implicitly passed
    //println!("Rectangle perimeter: {}", rectangle.perimeter(&rectangle));
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    // error: cannot borrow `rectangle` as mutable, 
    //        as it is not declared as mutable
    //rectangle.translate(1.0, 1.0);

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // mutable object can call mutable method
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // Previous 'destroy' call 'consumed' 'pair'
    //pair.destroy();

}


