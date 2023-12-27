

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

// DRY(Don't Repeat Yourself)
// https://doc.rust-lang.org/rust-by-example/macros/dry.html
/*
use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // 'tt' designator
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),)    
        );
    }
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

fn test_dry() {
    #[allow(unused_imports)]
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();
                    super::$func(&mut x, &y);

                    asser_eq!(x, z);
                }
            }
        }
    }

    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);

}
*/

// DSL(Domain Specific Language)
// https://doc.rust-lang.org/rust-by-example/macros/dsl.html
//  + 
// Variadic Interfaces
// https://doc.rust-lang.org/rust-by-example/macros/variadics.html

macro_rules! calculate {
    // single expression
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
    // multiple expression
    (eval $e:expr, $(eval $es:expr),+) => {
        {
            calculate! { eval $e }
            calculate! { $(eval $es),+}
        }
    };
}

fn test_dsl() {
    calculate! {
        // single expression
        eval 1 + 2 // eval is not a Rust keyword
    }

    calculate! {
        // single expression
        eval (1 + 2) * (3 / 4)
    } 
}

fn test_variadic_interfaces() {
    calculate! {
        // multiple expression
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }

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

    // Commeting as the compilation warning
    //test_dry();
    test_dsl();
    test_variadic_interfaces();

}