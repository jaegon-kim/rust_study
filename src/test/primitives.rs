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

    test_literal_and_operator();

}

fn test_literal_and_operator() {
    println!("test_literal_and_operator");

    println!(" 1 + 2 = {}", 1u32 + 2);
    println!(" 1 - 2 = {}", 1i32 - 2);

    println!(" 1e4 = {}, -2.5e-3 = {}", 1e4, -2.5e-3);

}
