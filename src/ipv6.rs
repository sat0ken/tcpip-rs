use crate::ethernet::EthernetHeader;
use crate::icmpv6::read_icmpv6_packet;
use bytes::Buf;

const IP_PROTOCOL_NUMBER_ICMPv6: u8 = 58;

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

pub fn read_ipv6_packet(eth_header: EthernetHeader, packet: Vec<u8>) {
    let mut buf = &packet[..];

    let mut ipv6_header = IPv6Header {
        version: buf.get_u8(),
        traffic_class: buf.get_u8(),
        flow_label: buf.get_u32(),
        header_length: buf.get_u16(),
        next_header: buf.get_u8(),
        hop_limit: buf.get_u8(),
        src_addr: buf.get_u128(),
        dst_addr: buf.get_u128(),
    };

    // 自分宛てのパケットでなければreturn
    // TODO: return

    // ARPテーブルを検索して存在していなければ追加
    // TODO: 処理を追加

    match ipv6_header.next_header {
        IP_PROTOCOL_NUMBER_ICMPV6 => {
            println!("receive icmpv6 packet");
            read_icmpv6_packet(buf.to_owned());
        }
        _ => {
            eprintln!("not supported ip protocol");
        }
    }
}
