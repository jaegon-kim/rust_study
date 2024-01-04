
#![allow(unused)]
//https://doc.rust-lang.org/std/primitive.pointer.html#method.offset

fn test_offset() {
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();

    unsafe {
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
    }
}


pub fn test_pointer() {
    println!("test_pointer");

    test_offset();
}