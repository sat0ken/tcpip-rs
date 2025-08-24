use crate::arp::{add_arp_tables, add_arp_tables_v6, search_arp_tables, search_arp_tables_v6};
use crate::ethernet::EthernetHeader;
use crate::icmpv6::read_icmpv6_packet;
use bytes::{Buf, BufMut};

pub const IP_PROTOCOL_NUMBER_ICMPV6: u8 = 58;
const FLOW_LABEL: u32 = 0x137a;

#[derive(Debug)]
pub struct IPv6Header {
    version: u8,
    traffic_class: u8,
    flow_label: u32,
    header_length: u16,
    next_header: u8,
    hop_limit: u8,
    src_addr: u128,
    dst_addr: u128,
}

pub fn read_ipv6_packet(
    eth_header: EthernetHeader,
    packet: Vec<u8>,
    ipv6: u128,
) -> (u128, Vec<u8>) {
    let mut buf = &packet[..];
    let first_32_bits = buf.get_u32();

    let mut ipv6_header = IPv6Header {
        version: (first_32_bits >> 28) as u8,
        traffic_class: ((first_32_bits >> 20) & 0xff) as u8,
        flow_label: first_32_bits & 0x000fffff,
        header_length: buf.get_u16(),
        next_header: buf.get_u8(),
        hop_limit: buf.get_u8(),
        src_addr: buf.get_u128(),
        dst_addr: buf.get_u128(),
    };

    // 自分宛てのパケットでなければreturn
    if ipv6 != ipv6_header.dst_addr {
        return (0, vec![]);
    }

    // ARPテーブルを検索して存在していなければ追加
    if search_arp_tables_v6(ipv6_header.src_addr) == [0, 0, 0, 0, 0, 0] {
        add_arp_tables_v6(eth_header.src_mac_addr, ipv6_header.src_addr)
    }

    match ipv6_header.next_header {
        IP_PROTOCOL_NUMBER_ICMPV6 => {
            println!("receive icmpv6 packet");
            let packet = read_icmpv6_packet(
                ipv6_header.dst_addr,
                ipv6_header.src_addr,
                buf[..].to_owned(),
            );
            return (
                ipv6_header.src_addr,
                out_ipv6_packet(
                    ipv6_header.dst_addr,
                    ipv6_header.src_addr,
                    IP_PROTOCOL_NUMBER_ICMPV6,
                    packet,
                ),
            );
        }
        _ => {
            eprintln!("not supported ip protocol");
        }
    }
    (0, vec![])
}

pub fn out_ipv6_packet(
    src_addr: u128,
    dest_addr: u128,
    protocol: u8,
    mut payload: Vec<u8>,
) -> Vec<u8> {
    let mut ipv6_header = IPv6Header {
        version: 6,
        traffic_class: 0,
        flow_label: FLOW_LABEL,
        header_length: payload.len() as u16,
        next_header: protocol,
        hop_limit: 64,
        src_addr: src_addr,
        dst_addr: dest_addr,
    };
    let mut buf = Vec::new();
    buf.put_u8(ipv6_header.version << 4);
    // 暫定処理
    buf.put_u8(0x0f);
    buf.put_u8(0xc9);
    buf.put_u8(0x29);
    //
    buf.put_u16(ipv6_header.header_length);
    buf.put_u8(ipv6_header.next_header);
    buf.put_u8(ipv6_header.hop_limit);
    buf.put_u128(ipv6_header.src_addr);
    buf.put_u128(ipv6_header.dst_addr);
    buf.append(&mut payload);

    buf
}
