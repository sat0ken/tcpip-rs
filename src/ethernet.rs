use crate::arp::{read_arp_packet, search_arp_tables, search_arp_tables_v6};
use crate::ipv4::read_ipv4_packet;
use crate::ipv6::read_ipv6_packet;
use crate::util::to_u16;
use bytes::BufMut;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::sync::mpsc::SyncSender;

pub const ETHERNET_TYPE_IPV4: u16 = 0x0800;
const ETHERNET_TYPE_ARP: u16 = 0x0806;
const ETHERNET_TYPE_IPV6: u16 = 0x86DD;

// const ETHERNET_BRD_ADDR: [u8; 6] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

pub struct EthernetHeader {
    pub dst_mac_addr: [u8; 6], // 宛先MACアドレス
    pub src_mac_addr: [u8; 6], // 送信元MACアドレス
    pub ethernet_type: u16,    // Ethernetタイプ
}

pub fn read_ethernet(
    packet: Vec<u8>,
    tx: SyncSender<Vec<u8>>,
    my_mac_addr: [u8; 6],
    my_ip_addr: Option<IpAddr>,
) {
    let eth_header = EthernetHeader {
        dst_mac_addr: packet[0..6].try_into().unwrap(),
        src_mac_addr: packet[6..12].try_into().unwrap(),
        ethernet_type: to_u16(packet.get(12..14).unwrap()),
    };

    if my_mac_addr != eth_header.dst_mac_addr {
        tx.send(vec![]).unwrap();
    }

    let mut ipv4_addr: u32 = 0;
    let mut ipv6_addr: u128 = 0;
    if let Some(ip_addr) = my_ip_addr {
        match ip_addr {
            IpAddr::V4(ipv4) => {
                ipv4_addr = ipv4.into();
            }
            IpAddr::V6(ipv6) => {
                ipv6_addr = ipv6.into();
            }
        }
    }

    match eth_header.ethernet_type {
        ETHERNET_TYPE_IPV4 => {
            println!("receive ipv4 packet");
            let mac = eth_header.dst_mac_addr;
            let (dest_ip_addr, packet) =
                read_ipv4_packet(eth_header, packet[14..].to_owned(), ipv4_addr);
            if dest_ip_addr != 0 {
                let dest_mac_addr = search_arp_tables(dest_ip_addr);
                println!("out_ethernet dest_mac_addr {dest_mac_addr:?}");
                out_ethernet(tx, mac, dest_mac_addr, packet, ETHERNET_TYPE_IPV4);
            }
        }
        ETHERNET_TYPE_ARP => {
            println!("receive arp packet");
            let (dest_ip_addr, packet) =
                read_arp_packet(packet[14..].to_owned(), my_mac_addr, ipv4_addr);
            if dest_ip_addr != 0 {
                let dest_mac_addr = search_arp_tables(dest_ip_addr);
                println!("out_ethernet dest_mac_addr {dest_mac_addr:?}");
                out_ethernet(tx, my_mac_addr, dest_mac_addr, packet, ETHERNET_TYPE_ARP);
            }
        }
        ETHERNET_TYPE_IPV6 => {
            println!("receive ipv6 packet");
            let (dest_ipv6_addr, packet) =
                read_ipv6_packet(eth_header, packet[14..].to_owned(), ipv6_addr);
            let dest_mac_addr = search_arp_tables_v6(dest_ipv6_addr);
            println!("out_ethernet dest_mac_addr {dest_mac_addr:?}");
            if dest_ipv6_addr != 0 {
                out_ethernet(tx, my_mac_addr, dest_mac_addr, packet, ETHERNET_TYPE_IPV6);
            };
        }
        _ => {}
    }
}

pub fn out_ethernet(
    tx: SyncSender<Vec<u8>>,
    src_mac_addr: [u8; 6],
    dest_mac_addr: [u8; 6],
    packet: Vec<u8>,
    ether_type: u16,
) {
    let mut buf = Vec::new();
    // Ethernetヘッダを生成
    buf.append(&mut dest_mac_addr.to_vec());
    buf.append(&mut src_mac_addr.to_vec());
    buf.put_u16(ether_type);
    buf.append(&mut packet.to_vec());

    tx.send(buf).unwrap();
}
