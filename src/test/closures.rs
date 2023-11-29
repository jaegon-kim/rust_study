#![allow(unused)]

fn basic() {

    println!("Basic closure");

    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 {
        i + outer_var
    };
    let closure_inferred  = |i: i32| i + outer_var;
    let one = || 1;

    println!("{}", closure_annotated(1));
    println!("{}", closure_inferred(1));
    println!("{}", one());
}

fn capturing() {
    use std::mem;
    let color = String::from("green");
    let print = || println!("'color': {}", color);

    // calling closure using borrow (for color)
    print();

    // 'collor' can be borrowed immutably again
    // the closure only holds an immutable reference to 'color'
    let _reborrow = &color;
    print();

    let _color_moved = color;
    // error:  color was moved above, but print() borrows color
    //print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    inc();

    // error !
    // let _reborrow = &count; 
    inc();

    // count can be borrowd
    let _count_reborrowd = &mut count;
    //inc();

    let movable = Box::new(3);

    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // error - value used here after move
    //consume();

    // using 'move' before vertical pipes forces closure
    // to take ownership of captured variables
    
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //error - haystack is moved to closure 'contains'
    //println!("There're {} elements in vec", haystack.len());

    let haystack2 = vec![1, 2, 3];
    let contains2 = |needle2| haystack2.contains(needle2);
    println!("{}", contains2(&1));
    println!("{}", contains2(&4));
    // haystack2 was not moved
    println!("There're {} elements in vec", haystack2.len());


}


pub fn test_closures() {
    println!("test_closures");
    basic();
    capturing();
}
