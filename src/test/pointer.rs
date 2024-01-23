
#![allow(unused)]
//https://doc.rust-lang.org/std/primitive.pointer.html#method.offset

// *const T 포인터 타입: T 타입의 데이터를 가리키는 상수 포인터
fn test_offset_const() {
    println!("test_offset_const");

    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();

    unsafe {
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
    }

    
    let x = 5;
    let ptr: *const i32 = &x;

    println!("addr of x: {:p}", ptr);
    unsafe {
        println!("x: {}", *ptr);
    }

    let buf: &[u8] = &[0x00, 0x01, 0x02, 0x03];
    println!("{:#04x} {:#04x} {:#04x} {:#04x}", buf[0], buf[1], buf[2], buf[3]);

    // 길이를 알 수 있는 slice는 포인터로 바꿀 수 없다. 정적 크기만 포인터로 사용될 수 있다.
    // 대신에 시작 주소를 포인터로 바꾼다.
    //let ptr: *const [u8] = buf;
    let ptr: *const u8 = buf.as_ptr();
    println!("addr of buf: {:p}", ptr);
    unsafe {
        println!("{:#04x} {:#04x} {:#04x} {:#04x}",
            *ptr.offset(0), *ptr.offset(1), *ptr.offset(2), *ptr.offset(3));
    }
    
}

fn test_offset_mut() {
    println!("test_offset_mut");

    let mut x = 5;
    let ptr: *mut i32 = &mut x;

    println!("addr of x: {:p}", ptr);
    unsafe {
        println!("x: {}", *ptr);
        *ptr = 7;
        println!("x: {}", *ptr);
    }

    let buf: &mut [u8] = &mut [0x00, 0x01, 0x02, 0x03];
    println!("{:#04x} {:#04x} {:#04x} {:#04x}", buf[0], buf[1], buf[2], buf[3]);

    let ptr: *mut u8 = buf.as_mut_ptr();
    println!("addr of buf: {:p}", ptr);
    unsafe {
        for i in 0..4 {
            print!("{:#04x} ", *ptr.offset(i));
        }
        println!();
            
        for i in 0..4 {
            *ptr.offset(i) += 10;
        }

        for i in 0..4 {
            print!("{:#04x} ", *ptr.offset(i));
        }
        println!();
       
    }
}


pub fn test_pointer() {
    println!("test_pointer");

    test_offset_const();
    test_offset_mut();
}