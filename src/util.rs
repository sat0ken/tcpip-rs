use crate::util::UtilsError::*;
use nix::ifaddrs::getifaddrs;
use nix::sys::socket::{AddressFamily, SockaddrLike, SockaddrStorage};

#[derive(Debug)]
pub enum UtilsError {
    NoNetworkInterface,
}

pub fn checksum(packet: Vec<u8>) -> u16 {
    let mut sum: u32 = 0;
    for i in 0..packet.len() {
        if i % 2 == 0 {
            sum += (((packet[i] as u16) << 8) | packet[i + 1] as u16) as u32;
        }
    }

    sum = (sum & 0xffff) + (sum >> 16);
    (sum ^ 0xffff) as u16
}

pub fn get_sockaddr(if_name: Box<str>) -> Result<SockaddrStorage, UtilsError> {
    let interfaces = getifaddrs().unwrap();
    for interface in interfaces {
        if if_name == Box::from(interface.interface_name) {
            if let Some(sock_storage) = interface.address {
                if AddressFamily::Packet == sock_storage.family().unwrap() {
                    return Ok(sock_storage);
                }
            }
        }
    }

    Err(NoNetworkInterface)
}

pub fn get_ipv4addr(if_name: Box<str>) -> u32 {
    let interfaces = getifaddrs().unwrap();
    for interface in interfaces {
        if if_name == Box::from(interface.interface_name) {
            if let Some(sock_storage) = interface.address {
                if AddressFamily::Inet == sock_storage.family().unwrap() {
                    let ipaddr = u32::from(sock_storage.as_sockaddr_in().unwrap().ip());
                    return ipaddr;
                }
            }
        }
    }
    0
}

pub fn to_u32(packet: &[u8]) -> u32 {
    u32::from_be_bytes(packet[0..4].try_into().unwrap())
}

pub fn to_u16(packet: &[u8]) -> u16 {
    u16::from_be_bytes(packet[0..2].try_into().unwrap())
}

pub fn dump_packet(packet: Vec<u8>) {
    for i in 0..packet.len() {
        print!("{:02x}", packet[i]);
    }
    println!();
}
