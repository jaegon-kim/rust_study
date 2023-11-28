#![allow(unused)]

fn test_if_else() {
    println!("* test_if_else");

    let n = 5;

    if n < 5 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);
}

fn test_loop() {
    println!("* test_loop");
    let mut count = 0u32;

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}

fn test_loop_nesting_lables() {
    println!("* test_loop_nesting_lables");
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        // this will not be printed
        println!("This point will never be reached");
    }    
}

fn test_return_from_loop() {
    println!("* test_return_from_loop");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("result = {}", result);
}

fn test_while() {
    println!("* test_while");
    let mut n = 1;
    while n < 11 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5== 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

fn test_for_range() {
    println!("* test_for_range");
    for n in 1..11 {
        println!("{}", n);
    }

    for n in 1..=10 {
        println!("{}", n);
    }

}

fn test_for_and_iterator() {
    println!("* test_for_and_iterator");
    let names = vec!["Bob", "Frank", "Ferris"];

    println!("> for name without move");
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    //println!("names: {}", names);
    println!("names: {:?}", names);

    let names2 = vec!["Bob", "Frank", "Ferris"];
    println!("> for name with move");
    for name in names2.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // error ! - names2 were already moved
    //println!("names2: {:?}", names2);
 
    let mut names3 = vec!["Bob", "Frank", "Ferris"];
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names3: {:?}", names3);

}

fn test_match() {
    println!("* test_match");

    let number = 13;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false   => 0,
        true    => 1,
    };

    println!("{} -> {}", boolean, binary);
    
}


#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn test_match_destructuring() {
    println!("* test_match_destructuring");

    // Tuples
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z)   => println!("First is '0', 'y' is {:?}, and 'z' is {:?}", y, z),
        (1, ..)     => println!("Fist is '1', and the rest doesn't matter"),
        (.., 2)     => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _           => println!("It doesn't matter what they are"),
    }    

    // Array & Slices
    let array = [1, -2, 6];
    // error - pattern requires 3 elements but array has 5
    //let array = [3, -2, 6, 7, 8];
    match array {
        [0, second, third]  => 
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third]       => 
            println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
        [-1, second, ..]    => 
            println!("array[0] = -1, array[1] = {} and all the other ones were ignored", second),
        [3, second, tail @ ..]  =>
            println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        [first, middle @ .., last]  =>
            println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }

    // Enum
    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red      => println!("The color is Red!"),
        Color::Blue     => println!("The color is Blue!"),
        Color::Green    => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m , y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }

    // Pointer/ref
    let reference = &4;
    println!(" use match reference");
    match reference {
        &val => println!("  Got a value via destructuring: {:?}", val),
        &4 => println!("  Got a value via &4"),
        val => println!("  Got a value via dereferencing: {:?}", val),
    }
    println!(" use match *reference");
    match *reference {
        4 => println!("  Got a value 4"),
        val => println!("  Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;

    println!(" check 'ref r'");
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
    println!("mut_value `mut_value`: {:?}", mut_value);

    // structs
    #[derive(Debug)]
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo {
        x: (1, 2),
        y: 3
    };

    println!("foo = {:?}", foo);

    match foo {
        Foo {x : (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        Foo {y: 2, x:i}      => println!("y is 2, i = {:?}", i),
        Foo {y, ..}          => println!("y = {}, we don't care about x", y),
    }

    let faa = Foo { x: (1, 2), y: 3};
    let Foo { x: x0, y: y0 } = faa;
    //println!("Outside: x0 = {x0:?}, y0 = {y0}");
    println!("x0 = {:?}, y0 = {:?}", x0, y0);

}

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn test_guards() {
    println!("* test_guards");

    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }

    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater that zero"),

        // without below compiation error will happen
        // all possible cases should be handled, (it will be checked by compiler)
        _ => unreachable!("Should never happen."),
    }
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn test_binding() {

    println!("Tell me what type of person you are");
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1 ..=12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n)      => println!("Not interesting... {}", n),
        _            => (),
    }
}

fn test_if_let() {
    println!("tewst_if_let");

    // style-1 with 'match'
    let optional = Some(7); //Option<i32>
    match optional {
        Some(i) => {
            println!("This is a really long string and '{:?}'", i);
        },
        _ => {},
    }

    // style-2 with 'let'
    let number = Some(7);
    let letter: Option<i32> = None;
    //let letter: Option<i32> = Some(8);
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // This will print 'Didn't match a number. Let's go with a letter!'
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn test_if_let_enum() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Error
    //if Foo::Bar == a {}
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}

/*
//https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html
// It needs  rust 1.65

use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

fn test_let_else() {

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

}
*/

fn test_while_let() {
    let mut optional = Some(0);

    // Style-1
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            _ => {
                break;
            }

        }
    }

    // Style-2
    optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }

}


pub fn test_flow_ctrl() {
    println!("flow_ctrl");

    test_if_else();
    test_loop();
    test_loop_nesting_lables();
    test_return_from_loop();
    test_while();
    test_for_range();
    test_for_and_iterator();
    test_match();
    test_match_destructuring();
    test_guards();
    test_binding();
    test_if_let();
    //test_if_let_enum();
    test_while_let();
}
