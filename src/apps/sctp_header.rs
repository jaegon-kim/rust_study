#![allow(unused)]

use std::vec::Vec;
use std::fmt;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SctpHeader {
    src_port: u16,
    dst_port: u16,
    vtag: u32,
    checksum: u32,
}

impl fmt::Display for SctpHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{")?;
        write!(f, "src_port: {}", self.decode_src_port())?;
        write!(f, ", dst_port: {}", self.decode_dst_port())?;
        write!(f, ", vtag: {}", self.decode_vtag())?;
        write!(f, ", checksum: {}", self.decode_checksum())?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl SctpHeader {
    /* // mutable borrowing of buffer
    pub fn new(buf: &mut [u8]) -> Option<&mut SctpHeader> {
        if buf.len() < std::mem::size_of::<SctpHeader>() {
            return None;
        }
        unsafe{
            std::mem::transmute(buf.as_mut_ptr())
        }
    }
    */

    pub fn new(buf: &[u8]) -> Option<&mut SctpHeader> {
        if !Self::affordable(buf) {
            return None;
        }
        unsafe{
            std::mem::transmute(buf.as_ptr())
        }
    }

    pub fn affordable(buf: &[u8]) -> bool {
        let chunk_hdr_end= std::mem::size_of::<SctpHeader>() +
                                  std::mem::size_of::<SctpChunkHeader>();
        if buf.len() < chunk_hdr_end {
            return false;
        }

        let ptr: *const u8 = buf as *const _ as *mut u8;
        let chunk_hdr: &mut SctpChunkHeader = 
            unsafe { std::mem::transmute(ptr.offset(std::mem::size_of::<SctpHeader>() as isize)) };

        let chunk_end = std::mem::size_of::<SctpHeader>() +
                        chunk_hdr.decode_length() as usize;
        if (buf.len() < chunk_end) {
            return false;
        }

        return true;
    }

    /*
    // mutable borrowing
    pub fn payload<T>(&mut self) -> Option<&mut T> {

        //let ptr: *const u8 = self as *const _ as *mut u8;
        let ptr = self as *mut SctpHeader as *mut u8;
        unsafe {
            let offset: isize = std::mem::size_of::<SctpHeader>() as isize;
            let ptr_payload = ptr.offset(offset);
            std::mem::transmute(ptr_payload)
        }
    }
    */
    pub fn payload<T>(&self) -> Option<&mut T> {

        let ptr: *const u8 = self as *const _ as *mut u8;
        unsafe {
            let offset: isize = std::mem::size_of::<SctpHeader>() as isize;
            let ptr_payload = ptr.offset(offset);
            std::mem::transmute(ptr_payload)
        }
    }

    pub fn decode_src_port(&self) -> u16 {
        u16::from_be(self.src_port)
    }

    pub fn set_src_port(&mut self, val: u16) {
        self.src_port = u16::to_be(val);
    }

    pub fn decode_dst_port(&self) -> u16 {
        u16::from_be(self.dst_port)
    }

    pub fn set_dst_port(&mut self, val: u16) {
        self.dst_port = u16::to_be(val);
    }

    pub fn decode_vtag(&self) -> u32 {
        u32::from_be(self.vtag)
    }

    pub fn set_vtag(&mut self, val: u32) {
        self.vtag = u32::to_be(val);
    }

    pub fn decode_checksum(&self) -> u32 {
        u32::from_be(self.checksum)
    }

    pub fn set_checksum(&mut self, val: u32) {
        self.checksum = u32::to_be(val);
    }

}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum SctpChunkType {
    Data = 0u8,
    Init = 1u8,
    InitAck = 2u8,
    Sack = 3u8,
    Heartbeat = 4u8,
    HeartbeatAck = 5u8,
    Abort = 6u8,
    Shutdown = 7u8,
    ShutdownAck = 8u8,
    Error = 9u8,
    CookieEcho = 10u8,
    CookieAck = 11u8,
    Ecne = 12u8,
    Cwr = 13u8,
    ShutdownComplete = 14u8,
    InvalidChunkType = 0xffu8,
}

impl SctpChunkType {
    fn decode(v: u8) -> SctpChunkType {
        match v {
            0u8  => SctpChunkType::Data,
            1u8  => SctpChunkType::Init,
            2u8  => SctpChunkType::InitAck,
            3u8  => SctpChunkType::Sack,
            4u8  => SctpChunkType::Heartbeat,
            5u8  => SctpChunkType::HeartbeatAck,
            6u8  => SctpChunkType::Abort,
            7u8  => SctpChunkType::Shutdown,
            8u8  => SctpChunkType::ShutdownAck,
            9u8  => SctpChunkType::Error,
            10u8 => SctpChunkType::CookieEcho,
            11u8 => SctpChunkType::CookieAck,
            12u8 => SctpChunkType::Ecne,
            13u8 => SctpChunkType::Cwr,
            14u8 => SctpChunkType::ShutdownComplete,
            _ => SctpChunkType::InvalidChunkType,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SctpChunkHeader {
    chunktype: u8,
    flags: u8,
    length: u16,
}

impl fmt::Display for SctpChunkHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{")?;
        write!(f, "chunktype: {:?}", self.decode_chunktype())?;
        write!(f, ", flags: {}", self.decode_flags())?;
        write!(f, ", length: {}", self.decode_length())?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl SctpChunkHeader {
    pub fn decode_chunktype(&self) -> SctpChunkType {
        SctpChunkType::decode(u8::from_be(self.chunktype))
    }

    pub fn set_chunktype(&mut self, val: SctpChunkType) {
        self.chunktype = u8::to_be(val as u8);
    }

    pub fn decode_flags(&self) -> u8 {
        u8::from_be(self.flags)
    }

    pub fn set_flags(&mut self, val: u8) {
        self.flags = u8::to_be(val);
    }

    pub fn decode_length(&self) -> u16 {
        u16::from_be(self.length)
    }

    pub fn set_length(&mut self, val: u16) {
        self.length = u16::to_be(val);
    }

}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SctpInitChunk {
    header: SctpChunkHeader,
    init_tag: u32,
    a_rwnd: u32,
    num_out_stream: u16,
    num_in_stream: u16,
    initial_tsn: u32,
}

impl fmt::Display for SctpInitChunk {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{")?;
        write!(f, "{}", self.header)?;
        write!(f, ", init_tag:{:#x}", self.decode_init_tag())?;
        write!(f, ", a_rwnd:{}", self.decode_a_rwnd())?;
        write!(f, ", num_out_stream:{}", self.decode_num_out_stream())?;
        write!(f, ", num_in_stream:{}", self.decode_num_in_stream())?;
        write!(f, ", initial_tsn:{:#x}", self.decode_initial_tsn())?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl SctpInitChunk {

    pub fn parse_parameters(&mut self) -> Option<Vec<&mut SctpParamHeader>> {

        let ptr_init_hdr: *const u8 = self as *const _ as *mut u8;
        let init_hdr_size: isize = std::mem::size_of::<SctpInitChunk>() as isize;
        let chunk_len: isize = self.header.decode_length() as isize;
        let total_param_size: isize = chunk_len - init_hdr_size;
        //println!("total_param_size: {}, offset_parm: {}", total_param_size, init_hdr_size);
        if total_param_size < 0 {
            return None;
        }

        unsafe {
            let mut ptr_param = ptr_init_hdr.offset(init_hdr_size);
            let mut offset = 0;
            let mut vec_param = Vec::new();

            while offset < total_param_size {
                let param: &mut SctpParamHeader = std::mem::transmute(ptr_param);
                let next_offset = param.next_offset();
                if next_offset <= 0 {
                    return None;
                }
                offset += next_offset;
                if (offset > chunk_len) {
                    return None;
                }
                ptr_param = ptr_init_hdr.offset(init_hdr_size + offset);
                //println!("param: {}, next offset: {}, offset: {}, value_len: {}", 
                //            param, param.next_offset(), offset, param.value_len());
                //print!("value: {{");            
                //for v in param.value() {
                //    print!("{}({:#04x}), ", v, v);
                //}
                //println!("}}");
                vec_param.push(param);
            }
            Some(vec_param)
        }
    }

    pub fn decode_init_tag(&self) -> u32 {
        u32::from_be(self.init_tag)
    }

    pub fn set_init_tag(&mut self, val: u32) {
        self.init_tag = u32::to_be(val);
    }

    pub fn decode_a_rwnd(&self) -> u32 {
        u32::from_be(self.a_rwnd)
    }

    pub fn set_a_rwnd(&mut self, val: u32) {
        self.a_rwnd = u32::to_be(val);
    }

    pub fn decode_num_out_stream(&self) -> u16 {
        u16::from_be(self.num_out_stream)
    }

    pub fn set_num_out_stream(&mut self, val: u16) {
        self.num_out_stream = u16::to_be(val);
    }

    pub fn decode_num_in_stream(&self) -> u16 {
        u16::from_be(self.num_in_stream)
    }

    pub fn set_num_in_stream(&mut self, val: u16) {
        self.num_in_stream = u16::to_be(val);
    }

    pub fn decode_initial_tsn(&self) -> u32 {
        u32::from_be(self.initial_tsn)
    }

    pub fn set_initial_tsn(&mut self, val: u32) {
        self.initial_tsn = u32::to_be(val);
    }
}


#[allow(dead_code)]
#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum SctpParamType {
    // INIT parameters
    // https://datatracker.ietf.org/doc/html/rfc4960#section-3.3.2.1
    Ip4Address = 5u16,
    Ip6Address = 6u16,
    CookiePreservative = 9u16,
    HostNameAddress = 11u16,
    SupportedAddressTypes = 12u16,
    EcnCapable = 0x8000u16,

    // INIT-ACK parameters
    // https://datatracker.ietf.org/doc/html/rfc4960#section-3.3.3.1
    StateCookie = 7u16,
    UnrecognizedParameter = 8u16,

    //https://www.rfc-editor.org/rfc/rfc3758.html
    // Forward-TSN-Supported Parameter For INIT and INIT ACK
    ForwardTSN = 0xc000u16,

    // HEATBEAT parameters
    // https://datatracker.ietf.org/doc/html/rfc4960#section-3.3.5
    HeartBeatInfo = 1u16,

    UnknownParamType(u16),
}

impl SctpParamType {
    pub fn decode(v: u16) -> SctpParamType {
        match v {
            5u16  => SctpParamType::Ip4Address,
            6u16  => SctpParamType::Ip6Address,
            9u16  => SctpParamType::CookiePreservative,
            11u16  => SctpParamType::HostNameAddress,
            12u16  => SctpParamType::SupportedAddressTypes,
            0x8000u16 => SctpParamType::EcnCapable,

            7u16 => SctpParamType::StateCookie,
            8u16 => SctpParamType::UnrecognizedParameter,

            0xc000u16 => SctpParamType::ForwardTSN,
            1u16 => SctpParamType::HeartBeatInfo,
            _ => SctpParamType::UnknownParamType(v),
        }
    }

    pub fn encode(&self) -> u16 {
        match self {
            SctpParamType::Ip4Address => 5u16,
            SctpParamType::Ip6Address => 6u16,
            SctpParamType::CookiePreservative => 9u16,
            SctpParamType::HostNameAddress => 11u16,
            SctpParamType::SupportedAddressTypes => 12u16,
            SctpParamType::EcnCapable => 0x8000u16,

            SctpParamType::StateCookie => 7u16,
            SctpParamType::UnrecognizedParameter => 8u16,

            SctpParamType::ForwardTSN => 0xc000u16,
            SctpParamType::HeartBeatInfo => 1u16,
            SctpParamType::UnknownParamType(v) => *v,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SctpParamHeader {
    param_type: u16,
    param_length: u16,
}

impl fmt::Display for SctpParamHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{")?;
        write!(f, "type: {:?}", self.decode_param_type())?;
        write!(f, ", length: {}", self.decode_param_length())?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl SctpParamHeader {

    fn align4(v: isize) -> isize {
        (v + 3) & 0xfffffffcisize
    }

    fn next_offset(&self) -> isize {
        Self::align4(self.decode_param_length() as isize)
    }

    pub fn value(&mut self) -> &mut [u8] {
        let ptr = self as *mut SctpParamHeader as *mut u8;
        unsafe {
            let offset: isize = std::mem::size_of::<SctpParamHeader>() as isize;
            let len = self.decode_param_length() as usize - offset as usize;
            unsafe{std::slice::from_raw_parts_mut(ptr.offset(offset), len)}
        }
    }

    pub fn decode_param_type(&self) -> SctpParamType {
        SctpParamType::decode(u16::from_be(self.param_type))
    }

    pub fn set_param_type(&mut self, val: SctpParamType) {

        self.param_type = u16::to_be(val.encode());
    }

    pub fn decode_param_length(&self) -> u16 {
        u16::from_be(self.param_length)
    }

    pub fn value_len(&self) -> isize {
        self.decode_param_length() as isize - std::mem::size_of::<SctpParamHeader>() as isize
    }

    pub fn set_param_length(&mut self, val: u16) {
        self.param_length = u16::to_be(val);
    }
}
