#![allow(unused)]

use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

struct Structure2(i32);

impl fmt::Display for Structure2 {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " toString: {}", self.0)
    }
}

struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        write!(f, "")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat > 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon > 0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}, {}: {:.3}", lat_c, self.lat.abs(), lon_c, self.lon.abs())
    }
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

pub fn test_print() {
    println!("test_print");
    println!("{:?} month is a year", 12);
    println!("{1:?} {0:?} is {actor:?} name", "Jaegon", "Kim", actor="actor's");
    println!("Now {:?} will be printed", Structure(32i32));
    println!("Now {:?} will be printed", Deep(Structure(34)));

    let name = "Kim";
    let age = 10;
    let person = Person{name, age};
    println!("Person = {:#?}", person);

    println!("using fmt::Display '{}'", Structure2(35));

    let minmax = MinMax(10, 1);
    println!("using fmt::Display '{}'", minmax);

    let point = Point{x: 100.2, y: 100.3};
    println!("using fmt::Display '{}'", point); // print Display
    println!("using fmt::Display '{:?}'", point); // print Debug

    let list = List(vec![1, 2, 3, 4, 5]);
    println!("using fmt::Display '{}'", list);

    //for (city) in [ // this will cause 'borrow the array with `&` or call `.iter()` on it to iterate over it'
    for (city) in &[
        City {name: "Suwon", lat: 10.1, lon:20.2},
        City {name: "Seoul", lat: 20.1, lon: 21.1},
        City {name: "Jeju", lat: 30.1, lon:33.1}
    ] {
        println!("{}", city);
    }

    //for (color) in [ // this will cause 'borrow the array with `&` or call `.iter()` on it to iterate over it'
    for (color) in &[
        Color{r:10, g:11, b:12},
        Color{r:11, g:12, b:13},
        Color{r:12, g:13, b:14}
    ] {
        println!("{:?}", color);
        //println!("{:#?}", color);
    }

    let buf: [u8; 12]= [
        0x96, 0x0C, 0x96, 0x0C, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 
    ];

    // Hex printing format
    print!("[len:{}]", buf.len());
    for b in buf {
        print!("{:02x} ", b); // 96 0c ..
        //print!("{:02X} ", b); // 96 0C ..
        //print!("{:#04x} ", b); // 0x96 0x0c .. ('4' include '0x')
        //print!("{:#04X} ", b); // 0x96 0x0C ..
    }
    println!();

}