#![allow(unused)]

pub fn test_freezing() {
    println!("test_freezing");
    let mut _mutable_integer = 7i32;
    {
        // freezing !! - 'mut' is disappeared
        let _mutable_integer = _mutable_integer;

        // error
        //_mutable_integer = 50;
    }
    _mutable_integer = 3;
}
