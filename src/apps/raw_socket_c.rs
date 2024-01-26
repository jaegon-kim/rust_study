use std::os::raw::c_int;
use std::ptr;
use std::ffi::CString;

use crate::apps::sctp_decoder::decode_sctp;
use crate::apps::ip_decoder::decode_ip;
use crate::apps::sctp_header::SctpHeader;

extern "C" {
    fn socket(af: c_int, socktype: c_int, protocol: c_int) -> c_int;
//    fn bind(s: c_int, name: *const std::os::raw::c_void, namelen: c_int) -> c_int;
//    fn listen(s: c_int, backlog: c_int) -> c_int;
//    fn accept(s: c_int, addr: *mut std::os::raw::c_void, addrlen: *mut c_int) -> c_int;
//    fn connect(s: c_int, name: *const std::os::raw::c_void, namelen: c_int) -> c_int;
//    fn send(s: c_int, buf: *const std::os::raw::c_void, len: usize, flags: c_int) -> c_int;
    fn recv(s: c_int, buf: *mut std::os::raw::c_void, len: usize, flags: c_int) -> c_int;
//    fn perror(s: *const libc::c_char);
}

fn perror() {
    let msg = CString::new("Error").unwrap();
    unsafe {
        libc::setlocale(libc::LC_ALL, ptr::null());
        libc::perror(msg.as_ptr());
    }
}

pub fn test_raw_socket_c() {
    println!("* test_raw_socket");

    let af = 2; // AF_INET
    let st = 3; // SOCK_RAW
    let proto = 132; // SCTP packets.
    let socket = unsafe{socket(af, st, proto)};
    println!("socket fd: {}", socket);

    /*
    let sin = std::net::SocketAddrV4::new([127, 0, 0, 1].into(), 38412);
    let plen = mem::size_of::<std::net::SocketAddrV4>() as i32;
    let r = unsafe{bind(socket, &sin as *const _ as *const _, plen)};
    if r < 0 {
        println!("bind failed");
        perror();
        return;
    }

    let backlog = 5;
    let r = unsafe{listen(socket, backlog)};
    if r < 0 {
        println!("listen failed");
        return;
    }
    */

    loop {
        /*
        let mut client_addr = unsafe {std::mem::zeroed()};
        let mut client_len = mem::size_of::<std::net::SocketAddrV4>() as i32;
        let sock = unsafe{accept(socket, &mut client_addr as * mut _ as *mut _, &mut client_len)};
        */

        //let mut buf = vec![0u8; 1024];
        let buf: &mut[u8] = &mut [0u8; 1024];
        let r = unsafe{recv(socket, buf.as_mut_ptr() as * mut _ as * mut _, buf.len(), 0)};
        if r < 0 {
            println!("recv failed");
            perror();
            break;
        }
        println!("\nReceived {} bytes", r);
        let opt_ip = decode_ip(buf, 68 as usize);
        //let opt_ip = decode_ip(buf, 20 as usize);
        if opt_ip.is_none() {
            continue;
        }
        let ip = opt_ip.unwrap();

        let opt_sctp = ip.payload::<SctpHeader>();
        println!("{:?}", opt_sctp);
        
        //decode_sctp(&mut buf[20..], (r - 20) as usize);
    }

}