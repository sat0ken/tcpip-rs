use bytes::{Buf, BufMut};
use crate::arp::{add_arp_tables, search_arp_tables};
use crate::ethernet::EthernetHeader;
use crate::icmp::read_icmp_packet;
use crate::util::{checksum};

pub const IP_PROTOCOL_NUMBER_ICMP: u8 = 1;
pub const IP_PROTOCOL_NUMBER_TCP: u8  = 6;
pub const IP_PROTOCOL_NUMBER_UDP: u8  = 17;

pub struct IPv4Header {
    version: u8,            // バージョン
    header_length: u8,      // ヘッダ長
    tos: u8,                // Type of Service
    total_len: u16,         // パケット長
    identity_num: u16,      // パケットの識別番号
    frag_offset: u16,       // 最初の3bitがフラグで残りの13bitがフラグオフセット
    ttl: u8,                // Time to Live
    protocol: u8,           // 上位のプロトコル番号
    checksum: u16,          // チェックサム
    pub(crate) src_addr: u32,   // 送信元IPアドレス
    pub(crate) dst_addr: u32   // 宛先IPアドレス
}

pub fn read_ipv4_packet(eth_header :EthernetHeader, packet: Vec<u8>, ipv4: u32) -> (u32, Vec<u8>) {
    let mut buf = &packet[..];

    let mut ipv4_header = IPv4Header{
        version: buf.get_u8(),
        header_length: 0,
        tos: buf.get_u8(),
        total_len: buf.get_u16(),
        identity_num: buf.get_u16(),
        frag_offset: buf.get_u16(),
        ttl: buf.get_u8(),
        protocol: buf.get_u8(),
        checksum: buf.get_u16(),
        src_addr: buf.get_u32(),
        dst_addr: buf.get_u32()
    };
    ipv4_header.header_length = (ipv4_header.version >> 4) * 5;
    ipv4_header.version = ipv4_header.version >> 4;

    // 自分宛てのパケットでなければreturn
    if ipv4 != ipv4_header.dst_addr {
        return (0, vec![]);
    }

    // ARPテーブルを検索して存在していなければ追加
    if search_arp_tables(ipv4_header.src_addr) == [0, 0, 0, 0, 0, 0] {
        add_arp_tables(eth_header.src_mac_addr, ipv4_header.src_addr)
    }

    match ipv4_header.protocol {
        IP_PROTOCOL_NUMBER_ICMP => {
            println!("receive icmp packet");
            // replyパケットを生成
            let packet = read_icmp_packet(buf[..].to_owned());
            return (ipv4_header.src_addr, out_ipv4_packet(ipv4_header.dst_addr, ipv4_header.src_addr, IP_PROTOCOL_NUMBER_ICMP, packet))
        }
        IP_PROTOCOL_NUMBER_TCP => {
            println!("receive tcp packet")
        }
        IP_PROTOCOL_NUMBER_UDP => {
            println!("receive udp packet")
        }
        _ => {
            eprintln!("not supported ip protocol")
        }
    }

    return (0, vec![]);
}

pub fn out_ipv4_packet(src_addr: u32, dest_addr: u32, protocol: u8, mut payload: Vec<u8>) -> Vec<u8> {
    let mut ipv4_header = IPv4Header{
        version: 4,
        header_length: 20,
        tos: 0,
        total_len: 0,
        identity_num: 0,
        frag_offset: 0,
        ttl: 64,
        protocol,
        checksum: 0,
        src_addr,
        dst_addr: dest_addr,
    };
    ipv4_header.total_len = (ipv4_header.header_length + payload.len() as u8) as u16;

    let mut buf = Vec::new();
    buf.put_u8((ipv4_header.version<<4) + (ipv4_header.header_length>>2));
    buf.put_u8(ipv4_header.tos);
    buf.put_u16(ipv4_header.total_len);
    buf.put_u16(ipv4_header.identity_num);
    buf.put_u16(ipv4_header.frag_offset);
    buf.put_u8(ipv4_header.ttl);
    buf.put_u8(ipv4_header.protocol);
    buf.put_u16(ipv4_header.checksum);
    buf.put_u32(ipv4_header.src_addr);
    buf.put_u32(ipv4_header.dst_addr);

    // checksumを計算してセット
    let checksum = checksum(buf.clone()).to_be_bytes().to_vec();
    buf[10] = checksum[0];
    buf[11] = checksum[1];
    // ヘッダの後ろにpayloadを追加
    buf.append(&mut payload);

    return buf;
}