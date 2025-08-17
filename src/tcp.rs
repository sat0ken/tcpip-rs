use bytes::Buf;

const FIN: u8 = 0x01;
const SYN: u8 = 0x02;
const RST: u8 = 0x04;
const PSH: u8 = 0x08;
const ACK: u8 = 0x10;
const URG: u8 = 0x20;
const ECR: u8 = 0x40;
const CWR: u8 = 0x80;

#[derive(Debug)]
pub struct TCPHeader {
    src_port: u16,
    dst_port: u16,
    seq: u32,
    ack: u32,
    offset: u8,
    //reserve: u8,
    flag: u8,
    window_size: u16,
    checksum: u16,
    urg_pt: u16,
}

struct TCPDummyHeader {
    src_ip: u32,
    dst_ip: u32,
    protocol: u16,
    length: u16,
}

pub fn read_tcp_packet(tcp_packet: Vec<u8>) {
    let mut buf = &tcp_packet[..];

    let mut tcp = TCPHeader {
        src_port: buf.get_u16(),
        dst_port: buf.get_u16(),
        seq: buf.get_u32(),
        ack: buf.get_u32(),
        offset: buf.get_u8(),
        //reserve: buf.get_u8(),
        flag: buf.get_u8(),
        window_size: buf.get_u16(),
        checksum: buf.get_u16(),
        urg_pt: buf.get_u16(),
    };
    tcp.offset = tcp.offset >> 2;
}
