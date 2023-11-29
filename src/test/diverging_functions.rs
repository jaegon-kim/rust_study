#![allow(unused)]
#![feature(never_type)]

// Diverging functions never return. 
// They are marked using ! (empty type)

// return empty type
// experimental
//fn foo() -> ! {
//    panic!("This call never returns");
//}

// return some type(?)
fn some_fn() {
    ()
}

fn test_call_some_fn() {
    let a: () = some_fn();
    println!("This function returns and you can see this line.");
}

//fn test_call_panic() {
//    let x: ! = panic!("This call never returns");
//    println!("You will never see this line");
//}

// https://doc.rust-lang.org/rust-by-example/fn/diverging.html
fn test() {
    fn some_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = 
                match i%2 == 1 {
                    true => i,
                    false => continue,
                };
                acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", some_odd_numbers(9));
}


pub fn test_diverging_functions() {
    println!("test diverging_functions");

    //foo();
    test_call_some_fn();
    //test_call_panic();
    test();
}

