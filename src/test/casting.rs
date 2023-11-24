#![allow(overflowing_literals)]
#![allow(unused)]

pub fn test_casting() {
    println!("test_casting");

    let decimal = 65.4321_f32;

    //error (no implicit conversion)
    //let integer: u8 = decimal;
    let integer = decimal as u8;
    let character = integer as char;

    //error (f32 cannot be converted to a char)
    //let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16); // 1000

    println!("1000 as a u8 is : {}", 1000 as u8); // 232

    println!("  -1 as a u8 is : {}", (-1i8) as u8); //255

    println!("1000 mod 256 is : {}", 1000 % 256); //232

    println!(" 128 as a i16 is: {}", 128 as i16); //128

    println!(" 128 as a i8 is : {}", 128 as i8); //-128

    println!("1000 as a u8 is : {}", 1000 as u8); //232

    println!(" 232 as a i8 is : {}", 232 as i8); //-24
    
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8); //255

    println!("-100.0 as u8 is : {}", -100.0_f32 as u8); //0

    println!("   nan as u8 is : {}", f32::NAN as u8); //0

    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }


}


