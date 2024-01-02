#![allow(unused)]

pub fn test_vectors() {
    println!("test_vectors");

    let ci:Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into {:?}", ci);

    // error - println!("xs:{:?}", xs);
    //ci.push(0);

    let mut xs = vec![1i32, 2, 3];
    println!("xs:{:?}", xs);

    xs.push(4);
    println!("xs:{:?}, len:{}, xs[0]:{}", xs, xs.len(), xs[0]);

    println!("pop last element:{:?}, xs: {:?}", xs.pop(), xs);

    // index out of bounds
    //println!("xs[4] =  {}", xs[3]);

    for x in xs.iter() {
        println!(">{}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!(">({}, {})", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }

    println!("updated vector: {:?}", xs);
}