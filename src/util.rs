use crate::util::UtilsError::*;
use nix::ifaddrs::getifaddrs;
use nix::sys::socket::{AddressFamily, SockaddrLike, SockaddrStorage};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
pub enum UtilsError {
    NoNetworkInterface,
}

pub fn checksum(packet: &Vec<u8>) -> u16 {
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

pub fn get_ipaddr(if_name: Box<str>) -> Option<IpAddr> {
    let interfaces = getifaddrs().unwrap();
    let mut ipv4_addr: Option<Ipv4Addr> = None;
    let mut ipv6_addr: Option<Ipv6Addr> = None;

    for interface in interfaces {
        if if_name == Box::from(interface.interface_name) {
            if let Some(sock_storage) = interface.address {
                match sock_storage.family() {
                    Some(AddressFamily::Inet) => {
                        // as_sockaddr_in() returns Option<T>, so we handle it directly
                        if let Some(ip_in) = sock_storage.as_sockaddr_in() {
                            ipv4_addr = Some(Ipv4Addr::from(ip_in.ip()));
                        }
                    }
                    Some(AddressFamily::Inet6) => {
                        // as_sockaddr_in6() returns Option<T>, we handle it here
                        if let Some(ip_in6) = sock_storage.as_sockaddr_in6() {
                            // Exclude IPv6 link-local addresses
                            if !ip_in6.ip().is_unicast_link_local() {
                                ipv6_addr = Some(ip_in6.ip());
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
    if let Some(ip) = ipv4_addr {
        return Some(IpAddr::V4(ip));
    }
    if let Some(ip) = ipv6_addr {
        return Some(IpAddr::V6(ip));
    }
    None
}

pub fn to_u32(packet: &[u8]) -> u32 {
    u32::from_be_bytes(packet[0..4].try_into().unwrap())
}

pub fn to_u16(packet: &[u8]) -> u16 {
    u16::from_be_bytes(packet[0..2].try_into().unwrap())
}

pub fn dump_packet(packet: &Vec<u8>) {
    for i in 0..packet.len() {
        print!("{:02x}", packet[i]);
    }
    println!();
}
