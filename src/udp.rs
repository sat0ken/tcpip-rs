use crate::dns::read_dns_packet;
use bytes::Buf;

#[derive(Debug)]
struct UDPHeader {
    src_port: u16,
    dst_port: u16,
    length: u16,
    checksum: u16,
}

pub fn read_udp_packet(packet: Vec<u8>) {
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
        }
        _ => {}
    }
}

pub fn out_udp_packet(udpheader: UDPHeader, packet: Vec<u8>) {}
