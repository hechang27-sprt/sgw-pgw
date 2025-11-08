use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio_tun::Tun;

// const BUFSIZE: usize = 1 * 1024 * 1024;

enum State {
    Sent,
    PdnReceived {
        addr: IpAddr,
        dest_addr: SocketAddr,
        payload: Vec<u8>,
    },
    SgwReceived {
        addr: SocketAddr,
        dest_addr: IpAddr,
        payload: Vec<u8>,
    },
}

struct PdnGateway {
    state: State,
}

impl PdnGateway {
    pub fn new() -> Self {
        Self { state: State::Sent }
    }

    pub fn handlePdnRecv(&self, buf: &[u8]) {}

    pub fn handleSgwRecv(&self, buf: &[u8]) {}
}

fn main() {}
