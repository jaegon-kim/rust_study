#![allow(unused)]

fn tuple_reverse( p: (i32, bool)) -> (bool, i32) {
    let (i32_param, bool_param) = p;

    (bool_param, i32_param)
}

#[derive(Debug)]
struct Matrix (f32, f32, f32, f32);

pub fn test_tuples() {
    println!("test_tuples");

    let long_tuple = ( 1u8, 2u16, 3u32, 4u64,
                        -1i8, -2i16, -3i32, -4i64,
                            0.1f32, 0.2f64,
                                'a', true );

    println!(" long_tuple[0] = {}", long_tuple.0);
    println!(" long_tuple[1] = {}", long_tuple.1);

    /*
    for v in long_tuple.iter().enumerate() {
        println!(" {}", v);
    }
    */

    let tuple_of_tuple = ((1u8, 2u16), (3u32, 4u64, -5i8));
    println!(" tuple of tuple = {:?}", tuple_of_tuple);

    let pair = (1, true);
    println!(" pair = {:?}", pair);

    println!(" reverse(pair) = {:?}", tuple_reverse(pair));

    println!(" tuple -> {:?}", (5u32,));
    println!(" u32   -> {:?}", (5u32));

    let t = (1, "abc", 3.5, true);
    let (a, b, c, d) = t;
    println!(" {:?} {:?} {:?} {:?}", a, b, c, d);

    println!(" {:?}", Matrix(0.1, 0.2, 0.3, 0.4));
}
