#![allow(unused)]
//use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};
//use nix::sys::socket::{bind, sockaddr_in, sockopt, sockopt_len, setsockopt};
//use nix::sys::uio::IoVec;
//use nix::unistd::read;

pub fn test_raw_socket_nix() {

    /*
    // Raw socket 생성
    let socket_fd = socket(
        AddressFamily::Inet,
        SockType::Raw,
        SockFlag::empty(),
        None,
    ).expect("Failed to create socket");

    // 소켓에 바인딩
    let sockaddr = sockaddr_in!(0; s_addr: u32::from_ne_bytes([0, 0, 0, 0]));
    bind(socket_fd, &sockaddr).expect("Failed to bind socket");

    // Raw 소켓에 옵션 설정 (예: IP_HDRINCL을 활성화)
    let hdrincl: libc::c_int = 1;
    setsockopt(socket_fd, sockopt::IpHdrInclude, &hdrincl).expect("Failed to set socket option");

    // 패킷 수신 루프
    loop {
        let mut buffer = [0u8; 4096];

        // 패킷 수신
        let iov = [IoVec::from_mut_slice(&mut buffer)];
        let size = read(socket_fd, &iov).expect("Failed to read from socket");

        // 여기에서 패킷을 처리하거나 로깅할 수 있습니다.
        println!("Received {} bytes: {:?}", size, &buffer[..size]);
    }
    */
}
