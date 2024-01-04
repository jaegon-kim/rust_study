#![allow(unused)]

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("analyze_slice");
    println!(" slice[0] = {}, len: {}", slice[0], slice.len());
}

fn print_slice(slice: &[i32]) {
    print!("print_slice ");
    print!("[");
    for v in slice {
        print!(" {}", v);
    }
    print!(" ]\n");
}

pub fn test_array_slice() {
    println!("test_array_slice");

    // Ref: https://stackoverflow.com/questions/57848114/what-are-the-differences-between-and-vec
    // array (residing in stack)
    let a = [1, 2, 3];
    // slice (reference to an array(in the stack))
    let b = &[1, 2, 3];
    // Vector
    let c = vec![1, 2, 3];

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("xs[0] = {}, len: {}", xs[0], xs.len());

    let ys: [i32; 500] = [0; 500];
    println!("ys[0] = {}, len: {}", ys[0], ys.len());
    println!("ys[10] = {}", ys[10]);
    println!("size of ys = {}", mem::size_of_val(&ys)); // size of ys = 2000 (4 byte x 500)

    analyze_slice(&xs);
    analyze_slice(&xs[1..4]);
    analyze_slice(&ys[10..12]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    println!();
    for i in 0..5 {
        println!(" {}", i);
    }

    println!();
    for i in 0..xs.len() {
        println!(" {}", i);
    }

    println!();

    //for v in xs { // borrow the array with `&` or call `.iter()` on it to iterate over it
    //for v in &xs { // This is 'Ok'
    for v in xs.iter() { // This is also 'Ok'
        println!(" {}", v);
    }
    

    print_slice(&xs);
    print_slice(&xs[1..xs.len()-1]);
    print_slice(&xs[1..xs.len()]);
    //print_slice(&ys);
    print_slice(&ys[10..12]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!(" {}: {}", i, xval),
            None => println!(" {} is None", i),
        }
    }


}

