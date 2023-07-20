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

    println!(" true AND false = {}", true && false);
    println!(" true OR false = {}", true || false);

    // binary number
    println!(" 0010 AND 0111 = {:04b}", 0b0010u32 & 0b0111u32);

    println!(" 1 <<5 = {}", 1u32 << 5 );
    println!(" One million = {}", 1_000_000u32);

}
