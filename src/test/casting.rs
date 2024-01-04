#![allow(overflowing_literals)]
#![allow(unused)]

fn test_casting_primitive() {
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

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct Header {
    src_port: u16,
    dst_port: u16,
    vtag: u32,
    checksum: u32,
}

fn test_casting_buf_to_struct() {

    let buf: [u8; 12]= [ 0x96, 0x0C, 0x96, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    //transmute with copy
    let h: Header = unsafe{std::mem::transmute(buf)};
    
    println!("Header {:?}, len: {}", h, std::mem::size_of_val(&h));
    println!(" - Address of buf({:?}), h({:?}) are different. Memory copy happend",
        std::ptr::addr_of!(buf), std::ptr::addr_of!(h));

    //transmute without copy
    let ref_h: &Header = unsafe{std::mem::transmute(buf.as_ptr())};

    println!("Header {:?}, len: {}", ref_h, std::mem::size_of_val(&ref_h));
    println!(" - Address of buf({:?}), h({:?}) are same. No memory copy",
        std::ptr::addr_of!(buf), std::ptr::addr_of!(*ref_h));

    let long_buf: &[u8]= &[
        0x96, 0x0C, 0x96, 0x0C, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x34,
        0x29, 0xA4, 0xB8, 0x40, 0x00, 0x01, 0xA0, 0x00,
        0x00, 0x02, 0x00, 0x02, 0xFC, 0xFD, 0xD3, 0x10,
        0x00, 0x00, 0x2C, 0x00, 0x06, 0x00, 0x05, 0x00,
        0x00, 0x80, 0x00, 0x00, 0x04, 0xC0, 0x00, 0x00,
        0x04 // 49 bytes
    ];

    println!("size of Header: {} bytes", std::mem::size_of::<Header>());
    println!("size of long_buf: {} bytes", std::mem::size_of_val(long_buf));

    // '&Header' and 'long_buf.as_ptr()' are same size - transmutable
    let ref_h_for_long_buf: &Header = unsafe{std::mem::transmute(long_buf.as_ptr())};
    println!("Header {:?}, len: {}", ref_h_for_long_buf, std::mem::size_of_val(&ref_h_for_long_buf));
    println!(" - Address of buf({:?}), h({:?}) are same. No memory copy",
        std::ptr::addr_of!(*long_buf), std::ptr::addr_of!(*ref_h_for_long_buf));

    // Error - reference to packed field is unaligned    
    // println!("src_port: {}", ref_h_for_long_buf.src_port);
    //
    // Instead of direct field access, pointer access is required
    unsafe {
        let ptr: *const u8 = ref_h_for_long_buf as *const _ as *const u8;

        let ptr_src_port = ptr.offset(0) as *const _ as *const u16;
        println!("src_port: {:04x} (addr:{:?})", *ptr_src_port, std::ptr::addr_of!(*ptr_src_port));
        println!("Endian processing of src_port: {:04x}", u16::from_be(*ptr_src_port));

        let ptr_dst_port = ptr.offset(2) as *const _ as *const u16;
        println!("dst_port: {:04x} (addr:{:?})", *ptr_dst_port, std::ptr::addr_of!(*ptr_dst_port));
        println!("Endian processing of dst_port: {:04x}", u16::from_be(*ptr_dst_port));

    }


}

pub fn test_casting() {
    test_casting_primitive();
    test_casting_buf_to_struct();
}


