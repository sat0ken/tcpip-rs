use bytes::Buf;

enum Opcode {
    Query,
    IQuery,
    Status,
}

struct DNSHeader {
    id: u16,
    qr:u8,
    opcode: Opcode,
    aa:u8,
    tc:u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
    question: u16,
    answer: u16,
    authority: u16,
    additional: u16,
    buf: Vec<u8>,
}

pub fn read_dns_packet(dns_packet: Vec<u8>) {
    let mut packet = &dns_packet[..];
    
    let dns_header = DNSHeader{
        id: packet.get_u16(),
        qr: packet[2]&0x80,
        opcode: Opcode::Query,
        aa: 0,
        tc: 0,
        rd: 0,
        ra: 0,
        z: 0,
        rcode: 0,
        question: 0,
        answer: 0,
        authority: 0,
        additional: 0,
        buf: vec![],
    };
}