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

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("xs[0] = {}, len: {}", xs[0], xs.len());

    let ys: [i32; 500] = [0; 500];
    println!("ys[0] = {}, len: {}", ys[0], ys.len());
    println!("ys[10] = {}", ys[10]);
    println!("size of ys = {}", mem::size_of_val(&ys));

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
    for v in xs {
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

