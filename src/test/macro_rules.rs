
macro_rules! say_hello {
    // no argument
    () => {
        println!("Hello")
    };
}

// https://doc.rust-lang.org/rust-by-example/macros/designators.html
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

// https://doc.rust-lang.org/rust-by-example/macros/overload.html
// test will compare `$left` and `$right`
macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
            stringify!($left), stringify!($right), $left && $right)
    };

    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
            stringify!($left), stringify!($right), $left || $right)
    };
}

//https://doc.rust-lang.org/rust-by-example/macros/repeat.html
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}


pub fn test_macro_rules() {
    println!("* macro_rules");
    say_hello!();

    // designators
    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!(
        {
            let x = 1u32;
            x * x + 2 * x - 1
        }
    );

    // overload
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    // Repeat
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5 * 1 * 1 * 1, 2 * 3, 4));

}