use std::fmt;


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct SctpHeader {
    src_port: u16,
    dst_port: u16,
    vtag: u32,
    checksum: u32,
}

impl fmt::Display for SctpHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "src_port: {}", self.decode_src_port())?;
        write!(f, ", dst_port: {}", self.decode_dst_port())?;
        write!(f, ", vtag: {}", self.decode_vtag())?;
        write!(f, ", checksum: {}", self.decode_checksum())?;
        Ok(())
    }
}

impl SctpHeader {
    fn new(buf: &[u8]) -> Option<&mut SctpHeader> {
        if buf.len() < std::mem::size_of::<SctpHeader>() {
            return None;
        }
        unsafe{
            std::mem::transmute(buf.as_ptr())
        }
    }

    fn payload<T>(&self) -> Option<&mut T> {

        let ptr: *const u8 = self as *const _ as *mut u8;
        unsafe {
            let offset = std::mem::size_of::<SctpHeader>() as isize;
            let ptr_payload = ptr.offset(offset);
            std::mem::transmute(ptr_payload)
        }
    }

    fn decode_src_port(&self) -> u16 {
        u16::from_be(self.src_port)
    }

    fn set_src_port(&mut self, val: u16) {
        self.src_port = u16::to_be(val);
    }

    fn decode_dst_port(&self) -> u16 {
        u16::from_be(self.dst_port)
    }

    fn set_dst_port(&mut self, val: u16) {
        self.dst_port = u16::to_be(val);
    }

    fn decode_vtag(&self) -> u32 {
        u32::from_be(self.vtag)
    }

    fn set_vtag(&mut self, val: u32) {
        self.vtag = u32::to_be(val);
    }

    fn decode_checksum(&self) -> u32 {
        u32::from_be(self.checksum)
    }

    fn set_checksum(&mut self, val: u32) {
        self.checksum = u32::to_be(val);
    }

}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct SctpChunkHeader {
    chunktype: u8,
    flags: u8,
    length: u16,
}

impl SctpChunkHeader {
    fn decode_flags(&self) -> u8 {
        u8::from_be(self.flags)
    }

    fn set_flags(&mut self, val: u8) {
        self.flags = u8::to_be(val);
    }
}

fn dump_buf<T>(buf: &[T], print_element: fn(&T)) {
    print!("[len:{}] ", buf.len());
    for b in buf {
        print_element(b);
    }
    println!();
}


pub fn test_decoding_sctp() {
    println!("sctp decoding");

    let buf: &mut [u8] = &mut [
        0x96, 0x0C, 0x96, 0x0C, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x34,
        0x29, 0xA4, 0xB8, 0x40, 0x00, 0x01, 0xA0, 0x00,
        0x00, 0x02, 0x00, 0x02, 0xFC, 0xFD, 0xD3, 0x10,
        0x00, 0x00, 0x2C, 0x00, 0x06, 0x00, 0x05, 0x00,
        0x00, 0x80, 0x00, 0x00, 0x04, 0xC0, 0x00, 0x00,
        0x04
    ];

    buf[8] = 0xFFu8; // field can be updated

    dump_buf::<u8>(buf, |b| print!("{:02x} ", b));

    let sctp_header: &mut SctpHeader = SctpHeader::new(&buf).unwrap();
    println!("sctp header {}, len: {}, addr: {:?}",
        sctp_header, std::mem::size_of_val(sctp_header), std::ptr::addr_of!(*sctp_header));

    sctp_header.set_src_port(0x1111u16);
    sctp_header.set_dst_port(0x2222u16);
    sctp_header.set_vtag(0x55667788u32);
    sctp_header.set_checksum(0xAABBCCDDu32);

    dump_buf::<u8>(buf, |b| print!("{:02x} ", b));
    println!("sctp header {}", sctp_header);

    let sctp_chunk_header = sctp_header.payload::<SctpChunkHeader>().unwrap();
    println!("sctp_chunk_header.flags {}", sctp_chunk_header.decode_flags());
    sctp_chunk_header.set_flags(0x55);
    dump_buf::<u8>(buf, |b| print!("{:02x} ", b));

}