use std::{
    borrow::Cow,
    net::{Ipv4Addr, Ipv6Addr},
};

use bin_proto::{BitDecode, BitEncode, Discriminable};

#[derive(Debug, PartialEq, Eq, BitEncode, BitDecode)]
pub struct IpPacket {
    #[codec(bits = 4)]
    #[codec(write_value = Discriminable::discriminant(&self.header))]
    pub version: u8,
    #[codec(tag = version)]
    pub header: IpHeader,
}

#[derive(Debug, PartialEq, Eq, BitEncode, BitDecode)]
#[codec(discriminant_type = u8)]
pub enum IpHeader {
    #[codec(discriminant = 4)]
    Ipv4(Ipv4Header),
    #[codec(discriminant = 6)]
    Ipv6(Ipv6Header),
}

#[derive(Debug, PartialEq, Eq, BitEncode, BitDecode)]
pub struct Ipv4Header {
    #[codec(bits = 4)]
    pub ihl: u8, // Internet Header Length
    #[codec(bits = 6)]
    pub dscp: u8, // Differentiated Services Code Point
    #[codec(bits = 2)]
    pub ecn: u8, // Explicit Congestion Notification
    pub length: u16,         // Total Length
    pub identification: u16, // Identification
    #[codec(bits = 3)]
    pub flags: u8, // Flags
    #[codec(bits = 13)]
    pub offset: u16, // Fragment Offset
    pub ttl: u8,             // Time To Live
    pub protocol: u8,        // Protocol
    pub checksum: u16,       // Header checksum
    pub src: Ipv4Addr,       // Source IP Address
    pub dst: Ipv4Addr,       /* Destination IP Address */
}

impl Ipv4Header {
    pub fn options_size(&self) -> usize {
        self.ihl.saturating_sub(4).into()
    }

    pub fn payload_size(&self) -> usize {
        self.length.saturating_sub(self.ihl.into()).into()
    }
}

#[derive(Debug, PartialEq, Eq, BitEncode, BitDecode)]
pub struct Ipv6Header {
    #[codec(bits = 6)]
    pub dscp: u8, // Differentiated Services Code Point
    #[codec(bits = 2)]
    pub ecn: u8, // Explicit Congestion Notification
    #[codec(bits = 20)]
    pub fl: u32, // flow label
    pub length: u16,
    pub protocol: u8,
    pub hl: u8, // hop limit
    pub src: Ipv6Addr,
    pub dst: Ipv6Addr,
}

#[derive(Debug, PartialEq, Eq, BitEncode, BitDecode)]
pub struct GtpuHeader {}

impl GtpuHeader {
    pub fn wrap(teid: u32, payload: Cow<[u8]>) -> Self {
        todo!()
    }
}
