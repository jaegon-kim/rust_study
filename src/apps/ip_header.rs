#![allow(unused)]

use std::fmt;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Ipv4Header {
    version_hdr_length: u8,
    tos: u8,
    total_length: u16,
    id: u16,
    frag_offset: u16,
    ttl: u8,
    protocol: u8,
    hdr_checksum: u16,
    src: [u8; 4],
    dst: [u8; 4] 
}

impl fmt::Display for Ipv4Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{")?;
        write!(f, "version: {}", self.decode_version())?;
        write!(f, ", hdr_length: {}", self.decode_hdr_length())?;
        write!(f, ", tos: {}", self.decode_tos())?;
        write!(f, ", total_length: {}", self.decode_total_length())?;
        write!(f, ", id: {}", self.decode_id())?;
        write!(f, ", frag_offset: {}", self.decode_frag_offset())?;
        write!(f, ", ttl: {}", self.decode_ttl())?;
        write!(f, ", protocol: {}", self.decode_protocol())?;
        write!(f, ", hdr_checksum: {}", self.decode_hdr_checksum())?;
        write!(f, ", src: {:?}", self.decode_src())?;
        write!(f, ", dst: {:?}", self.decode_dst())?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl Ipv4Header {
    pub fn new(buf: &[u8]) -> Option<&mut Ipv4Header> {
        if !Self::affordable(buf) {
            return None;
        }
        unsafe {
            std::mem::transmute(buf.as_ptr())
        }
    }

    pub fn affordable(buf: &[u8]) -> bool {
        let ip_hdr_end: usize = std::mem::size_of::<Ipv4Header>();
        if buf.len() < ip_hdr_end {
            return false;
        }
        return true;
    }

    pub fn len() -> usize {
        std::mem::size_of::<Ipv4Header>()
    }

    pub fn payload<T>(&self) -> Option<&mut T> {

        let ptr: *const u8 = self as *const _ as *mut u8;
        unsafe {
            let offset: isize = std::mem::size_of::<Ipv4Header>() as isize;
            let ptr_payload = ptr.offset(offset);
            std::mem::transmute(ptr_payload)
        }
    }

    pub fn decode_version(&self) -> u8 {
        u8::from_be((self.version_hdr_length & 0xf0u8) >> 4)
    }

    pub fn set_version(&mut self, val: u8) {
        self.version_hdr_length = self.version_hdr_length & 0x0fu8; 
        self.version_hdr_length = self.version_hdr_length | (((u8::to_be(val) & 0x0fu8) << 4));
    }

    pub fn decode_hdr_length(&self) -> u8 {
        u8::from_be(self.version_hdr_length & 0x0fu8)
    }

    pub fn set_hdr_length(&mut self, val: u8) {
        self.version_hdr_length = self.version_hdr_length & 0xf0u8;
        self.version_hdr_length = self.version_hdr_length | (u8::to_be(val) & 0x0fu8);
    }

    pub fn decode_tos(&self) -> u8 {
        u8::from_be(self.tos)
    }

    pub fn set_tos(&mut self, val: u8) {
        self.tos = u8::to_be(val);
    }

    pub fn decode_total_length(&self) -> u16 {
        u16::from_be(self.total_length)
    }

    pub fn set_total_length(&mut self, val: u16) {
        self.total_length= u16::to_be(val);
    }

    pub fn decode_id(&self) -> u16 {
        u16::from_be(self.id)
    }

    pub fn set_id(&mut self, val: u16) {
        self.id = u16::to_be(val);
    }

    pub fn decode_frag_offset(&self) -> u16 {
        u16::from_be(self.frag_offset)
    }

    pub fn set_frag_offset(&mut self, val: u16) {
        self.frag_offset = u16::to_be(val);
    }

    pub fn decode_ttl(&self) -> u8 {
        u8::from_be(self.ttl)
    }

    pub fn set_ttl(&mut self, val: u8) {
        self.ttl = u8::to_be(val);
    }

    pub fn decode_protocol(&self) -> u8 {
        u8::from_be(self.protocol)
    }

    pub fn set_protocol(&mut self, val: u8) {
        self.protocol = u8::to_be(val);
    }

    pub fn decode_hdr_checksum(&self) -> u16 {
        u16::from_be(self.hdr_checksum)
    }

    pub fn set_hdr_checksum(&mut self, val: u16) {
        self.hdr_checksum = u16::to_be(val);
    }

    pub fn decode_src(&self) -> [u8; 4] {
        [u8::from_be(self.src[0]),
            u8::from_be(self.src[1]),
                u8::from_be(self.src[2]),
                    u8::from_be(self.src[3])]
    }

    pub fn set_src(&mut self, val: &[u8; 4]) {
        self.src[0]= u8::to_be(val[0]);
        self.src[1]= u8::to_be(val[1]);
        self.src[2]= u8::to_be(val[2]);
        self.src[3]= u8::to_be(val[3]);
    }

    pub fn decode_dst(&self) -> [u8; 4] {
        [u8::from_be(self.dst[0]),
            u8::from_be(self.dst[1]),
                u8::from_be(self.dst[2]),
                    u8::from_be(self.dst[3])]
    }

    pub fn set_dst(&mut self, val: &[u8; 4]) {
        self.dst[0]= u8::to_be(val[0]);
        self.dst[1]= u8::to_be(val[1]);
        self.dst[2]= u8::to_be(val[2]);
        self.dst[3]= u8::to_be(val[3]);
    }









}