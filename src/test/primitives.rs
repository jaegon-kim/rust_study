#![allow(unused)]
pub fn test_primitive() {
    println!("test primitive(no warning)");
    let logical: bool = true;
    let a_logical: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 1.0;
    let default_integer = 7;

    let mut inferred_int = 12;
    inferred_int = 10000000i64;

    let mut mutable = 12;
    mutable = 60;

    //mutable = true; // this causes error
    let mutable = true; // this causes warning   

}