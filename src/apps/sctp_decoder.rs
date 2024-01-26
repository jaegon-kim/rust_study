#![allow(unused)]

use crate::apps::sctp_header::SctpHeader;
use crate::apps::sctp_header::SctpChunkHeader;
use crate::apps::sctp_header::SctpChunkType;
use crate::apps::sctp_header::SctpInitChunk;

fn dump_buf<T>(buf: & [T], len: usize, print_element: fn(&T)) {
    let l: usize = if buf.len() > len { len } else { buf.len() };
    print!("[len:{}] ", l);
    for i in 0..l {
        print_element(&buf[i])
    }
    println!();
}

pub fn decode_sctp(buf: &mut [u8], len: usize) {
    println!("- decode_sctp");
    
    dump_buf::<u8>(buf, len, |b| print!("{:02x} ", b));

    let opt_sctp_hdr:Option<&mut SctpHeader> = SctpHeader::new(buf);
    if opt_sctp_hdr.is_none() {
        println!("Too short buffer for SCTP");
        return;
    }

    let sctp_hdr: &mut SctpHeader = opt_sctp_hdr.unwrap();
    println!("sctp header {}, len: {}, addr: {:?}",
        sctp_hdr, std::mem::size_of_val(sctp_hdr),std::ptr::addr_of!(*sctp_hdr));

    // sctp_header is mutable
    //sctp_hdr.set_checksum(0xAABBCCDDu32);
    //dump_buf::<u8>(buf, len, |b| print!("{:02x} ", b));
    //println!("sctp header {}", sctp_hdr);

    let sctp_chunk_hdr: &mut SctpChunkHeader = sctp_hdr.payload::<SctpChunkHeader>().unwrap();
    println!("sctp chunk header {}", sctp_chunk_hdr);

    // sctp_chunk_header is mutable
    //sctp_chunk_hdr.set_chunktype(SctpChunkType::Init);
    //sctp_chunk_hdr.set_flags(0x55);
    //dump_buf::<u8>(buf, len, |b| print!("{:02x} ", b));
    //println!("sctp chunk header {}", sctp_chunk_hdr);

    match sctp_chunk_hdr.decode_chunktype() {
        SctpChunkType::Init => {
            let sctp_init: &mut SctpInitChunk = sctp_hdr.payload::<SctpInitChunk>().unwrap();
            decoding_sctp_init(sctp_init);
        },
        _ => {
            println!("Else")
        },
    }
    dump_buf::<u8>(buf, len, |b| print!("{:02x} ", b));
}

fn decoding_sctp_init(sctp_init: &mut SctpInitChunk) {
    println!("sctp init {}", sctp_init);

    match sctp_init.parse_parameters() {
        Some(params) => {
            for p in params {
                print!("param: {} value: {{", p);
                for v in p.value() {
                    print!("{}({:#04x}), ", v, v);
                }
                println!("}}");
            }
        },
        None => {
            print!("Invalid parameters in sctp_init"); 
        }
    }

}


pub fn test_decoding_sctp() {
    println!("* test_decoding_sctp");
    let buf: &mut [u8] = &mut [
        0x96, 0x0c, 0x96, 0x0c, 0x00, 0x00, 0x00, 0x00, 
    ];
    decode_sctp(buf, buf.len());

    let buf: &mut [u8] = &mut [
        0x96, 0x0c, 0x96, 0x0c, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x34, 
        0x29, 0xa4, 0xb8, 0x40, 0x00, 0x01, 0xa0, 0x00, 
        0x00, 0x02, 0x00, 0x02, 0xfc, 0xfd, 0xd3, 0x10, 
        0x00, 0x05, 0x00, 0x08, 0x0a, 0xfb, 0x47, 0x92, 
        0x00, 0x05, 0x00, 0x08, 0xc0, 0xa8, 0x38, 0x02, 
        0x00, 0x0c, 0x00, 0x06, 0x00, 0x05, 0x00, 0x00, 
        0x80, 0x00, 0x00, 0x04, 0xc0, 0x00, 0x00, 0x04, 
    ];
    decode_sctp(buf, buf.len());

}