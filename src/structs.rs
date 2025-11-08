use deku::{DekuRead, DekuWrite};
use std::{cmp::min, net::Ipv6Addr};

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct IpHeader<'a> {
    #[deku(bits = 4)]
    version: u8,
    #[deku(ctx = "*version")]
    header: IpSubHeader<'a>,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "big", ctx = "version: u8", id = "version")]
pub enum IpSubHeader<'a> {
    #[deku(id = 4)]
    Ipv4(Ipv4Header<'a>),
    #[deku(id = 6)]
    Ipv6(Ipv6Header<'a>),
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Ipv4Header<'a> {
    #[deku(bits = 4)]
    pub ihl: u8, // Internet Header Length
    #[deku(bits = 6)]
    pub dscp: u8, // Differentiated Services Code Point
    #[deku(bits = 2)]
    pub ecn: u8, // Explicit Congestion Notification
    pub length: u16,         // Total Length
    pub identification: u16, // Identification
    #[deku(bits = 3)]
    pub flags: u8, // Flags
    #[deku(bits = 13)]
    pub offset: u16, // Fragment Offset
    pub ttl: u8,             // Time To Live
    pub protocol: u8,        // Protocol
    pub checksum: u16,       // Header checksum
    pub src: Ipv4Addr,       // Source IP Address
    pub dst: Ipv4Addr,       /* Destination IP Address */

    #[deku(count = "min(ihl - 4, 0)")]
    pub option: &'a [u8], // Options

    #[deku(count = "min(length - ihl, 0)")]
    pub payload: &'a [u8],
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Ipv6Header {
    #[deku(bits = 6)]
    pub dscp: u8, // Differentiated Services Code Point
    #[deku(bits = 2)]
    pub ecn: u8, // Explicit Congestion Notification
    #[deku(bits = 20)]
    pub fl: u32, // flow label
    pub length: u16,
    pub protocol: u8,
    pub hl: u8, // hop limit
    pub src: Ipv6Addr,
    pub dest: Ipv6Addr,
}
