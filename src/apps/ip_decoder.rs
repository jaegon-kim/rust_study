#![allow(unused)]

use crate::apps::ip_header::Ipv4Header;

fn dump_buf<T>(buf: & [T], len: usize, print_element: fn(&T)) {
    let l: usize = if buf.len() > len { len } else { buf.len() };
    print!("[len:{}] ", l);
    for i in 0..l {
        print_element(&buf[i])
    }
    println!();
}

pub fn decode_ip(buf: &mut [u8], len: usize) -> Option<&mut Ipv4Header> {
    println!("- decode_ip");
    dump_buf::<u8>(buf, len, |b| print!("{:02x} ", b));

    let opt_ip_hdr:Option<&mut Ipv4Header> = Ipv4Header::new(buf);
    if opt_ip_hdr.is_none() {
        println!("Too short buffer for Ipv4");
        return None;
    }

    let ip_hdr: &mut Ipv4Header = opt_ip_hdr.unwrap();
    println!("ip header {}, len: {}, addr: {:?}",
    ip_hdr, std::mem::size_of_val(ip_hdr),std::ptr::addr_of!(*ip_hdr));

    ip_hdr.set_version(6u8);
    ip_hdr.set_hdr_length(0xa);
    ip_hdr.set_tos(0xff);
    ip_hdr.set_total_length(0x1001);
    ip_hdr.set_id(0x1234);
    ip_hdr.set_frag_offset(0x0000);
    ip_hdr.set_ttl(ip_hdr.decode_ttl() - 1);
    ip_hdr.set_protocol(0xff);
    ip_hdr.set_hdr_checksum(0x0000);
    ip_hdr.set_src(&[192, 168, 0, 1]);
    ip_hdr.set_dst(&[192, 168, 0, 100]);
    dump_buf::<u8>(buf, len, |b| print!("{:02x} ", b));
    println!("ip header {}", ip_hdr);

    Some(ip_hdr)
}
