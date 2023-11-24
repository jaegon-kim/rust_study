#![allow(unused)]

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;


pub fn test_alias() {
    println!("test_alias");

    let nanosecond: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!("{} nanoseconds + {} inches = {} unit?",
        nanosecond,
        inches,
        nanosecond + inches);
}
