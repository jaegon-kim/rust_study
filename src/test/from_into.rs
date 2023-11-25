#![allow(unused)]

use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number1 {
    value: i32,
}

impl From<i32> for Number1 {
    fn from(item: i32) -> Self {
        Number1 {value: item}
    }
}

#[derive(Debug)]
struct Number2 {
    value: i32,
}

impl Into<Number2> for i32 {
    fn into(self) -> Number2 {
        Number2 {value: self}
    }
}


pub fn test_from_into() {
    println!("test_from_into");

    //let my_str = "hello";
    //let my_string = String:from(my_str);
    //println!("my_str:{}, my_string:{}", my_str, my_string);

    let num = Number1::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number2 = int.into();
    println!("My name is {:?}", num);

}