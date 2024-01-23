#![allow(unused)]

pub fn test_hex_stream2array() {
    println!("test_hex_stream2array");

    let buf = b"960c960c00000000000000000100003429a4b8400001a0000002000\
                            2fcfdd310000500080afb479200050008c0a83802000c00060005000080000004c0000004";
    
    if buf.len() % 2 != 0 {
        println!("Invalid buf len: {}", buf.len());
        return;
    }

    for i in 0..buf.len()/2 {
        print!("0x{}{}, ", buf[2 * i] as char, buf[2 * i + 1] as char);

        if i != 0 && ((i + 1) % 8) == 0 {
            println!();
        }
    }
    println!();
}