use crate::{
    dns::{self, read_dns_packet},
    ipv4::IPv4Header,
    util::checksum,
};
use bytes::{Buf, BufMut};

#[derive(Debug)]
struct UDPHeader {
    src_port: u16,
    dst_port: u16,
    length: u16,
    checksum: u16,
}

#[derive(Debug)]
struct UDPDummyHeader {
    src_addr: u32,
    dst_addr: u32,
    protocol: u16,
    length: u16,
}

pub fn read_udp_packet(ipv4_header: &IPv4Header, packet: Vec<u8>) -> Vec<u8> {
    let mut buf = &packet[..];

    let udp = UDPHeader {
        src_port: buf.get_u16(),
        dst_port: buf.get_u16(),
        length: buf.get_u16(),
        checksum: buf.get_u16(),
    };
    println!(
        "recv udp packet header is {:?}, payload is {:?}",
        udp,
        String::from_utf8(Vec::from(buf)).unwrap()
    );
    match udp.dst_port {
        53 => {
            // DNSレスポンスパケットを生成
            let dns_response = read_dns_packet(packet);
            return out_udp_packet(ipv4_header, udp, dns_response);
        }
        _ => {}
    }
    vec![]
}

pub fn out_udp_packet(
    ipv4_header: &IPv4Header,
    recv_udpheader: UDPHeader,
    packet: Vec<u8>,
) -> Vec<u8> {
    let mut buf = Vec::new();
    // UDPヘッダ
    let send_udp = UDPHeader {
        src_port: recv_udpheader.dst_port,
        dst_port: recv_udpheader.src_port,
        length: (8 + packet.len()) as u16,
        checksum: 0,
    };
    buf.put_u16(send_udp.src_port);
    buf.put_u16(send_udp.dst_port);
    buf.put_u16(send_udp.length);
    buf.put_u16(send_udp.checksum);
    buf.put(packet.as_slice());

    let mut calc_checksum_buf: Vec<u8> = Vec::new();
    let udp_dummy_header = UDPDummyHeader {
        src_addr: ipv4_header.dst_addr,
        dst_addr: ipv4_header.src_addr,
        length: buf.len() as u16,
        protocol: 17, // UDP
    };
    calc_checksum_buf.put_u32(udp_dummy_header.src_addr);
    calc_checksum_buf.put_u32(udp_dummy_header.dst_addr);
    calc_checksum_buf.put_u16(udp_dummy_header.protocol);
    calc_checksum_buf.put_u16(udp_dummy_header.length);
    calc_checksum_buf.put_slice(&buf.as_slice());
    // checksumを計算してセット
    let checksum = checksum(&calc_checksum_buf).to_be_bytes().to_vec();
    buf[6] = checksum[0];
    buf[7] = checksum[1];

    buf
}
