use crate::util::{dump_packet, get_ipaddr};
use bytes::{Buf, BufMut};
use nix::NixPath;
use std::net::IpAddr;

enum Opcode {
    Query,
    IQuery,
    Status,
}

//  DNS Header
//  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//  +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//  |                      ID                       |
//  +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//  |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
//  +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//  |                    QDCOUNT                    |
//  +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//  |                    ANCOUNT                    |
//  +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//  |                    NSCOUNT                    |
//  +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//  |                    ARCOUNT                    |
//  +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive(Debug)]
struct DNSHeader {
    id: u16,
    qr: u8,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
    qd_cnt: u16,
    answer_cnt: u16,
    authority_cnt: u16,
    additional_cnt: u16,
}

#[derive(Debug)]
struct Question {
    domain: Vec<u8>,
    record_type: u16,
    class: u16,
}

struct Answer {
    domain: Vec<u8>,
    record_type: u16,
    class: u16,
    ttl: u32,
    rd_length: u16,
    rd_data: Vec<u8>,
}

pub fn read_dns_packet(dns_packet: Vec<u8>) -> Vec<u8> {
    let mut packet = &dns_packet[..];
    let id = packet.get_u16();
    let qr = packet.get_u8();
    let ra = packet.get_u8();

    let dns_header = DNSHeader {
        id,
        // 1byte目
        qr: qr & 0x80,
        opcode: (qr >> 3) & 0x0f,
        aa: qr & 0x04,
        tc: qr & 0x02,
        // 2byte目
        rd: qr & 0x01,
        ra: ra & 0x80,
        z: (ra >> 4) & 0x7,
        rcode: ra & 0xF,
        // QDCOUNT
        qd_cnt: packet.get_u16(),
        // ANCOUNT
        answer_cnt: packet.get_u16(),
        // NSCOUNT
        authority_cnt: packet.get_u16(),
        // ARCOUNT
        additional_cnt: packet.get_u16(),
    };
    if dns_header.qd_cnt == 1 {
        let domain_length = packet.len();
        let question = Question {
            domain: packet.get(0..(domain_length - 4)).unwrap().to_vec(),
            record_type: u16::from_be_bytes([packet[domain_length - 4], packet[domain_length - 3]]),
            class: u16::from_be_bytes([packet[domain_length - 2], packet[domain_length - 1]]),
        };
        return dns_response(dns_header.id, question);
    }
    vec![]
}

fn dns_response(id: u16, question: Question) -> Vec<u8> {
    let mut buf = Vec::new();

    let dns_header = DNSHeader {
        id,
        qr: 1,
        opcode: 0,
        aa: 0,
        tc: 0,
        rd: 1,
        ra: 1,
        z: 0,
        rcode: 0,
        qd_cnt: 1,
        answer_cnt: 1,
        authority_cnt: 0,
        additional_cnt: 0,
    };

    let mut qr_byte = dns_header.qr << 7 & 0x80;
    qr_byte += dns_header.opcode & 0x0f;
    qr_byte += dns_header.aa & 0x04;
    qr_byte += dns_header.tc & 0x02;
    qr_byte += dns_header.rd & 0x01;

    let mut ra_byte = dns_header.ra << 7 & 0x80;
    ra_byte += dns_header.z << 6 & 0x70;
    ra_byte += dns_header.rcode << 3 & 0xf;

    let ip_addr = get_ipaddr(Box::from("host2-host1")).unwrap();
    let mut ip_addr_vec = vec![];
    match ip_addr {
        IpAddr::V4(ip) => {
            ip_addr_vec = Vec::from(ip.octets());
        }
        IpAddr::V6(ip) => {
            ip_addr_vec = Vec::from(ip.octets());
        }
    }
    let answer = Answer {
        domain: question.domain.clone(),
        record_type: 1,
        class: 1,
        ttl: 0xe1,
        rd_length: ip_addr_vec.len() as u16,
        rd_data: ip_addr_vec,
    };

    // DNSヘッダーの作成
    buf.put_u16(dns_header.id);
    buf.put_u8(qr_byte);
    buf.put_u8(ra_byte);
    buf.put_u16(dns_header.qd_cnt);
    buf.put_u16(dns_header.answer_cnt);
    buf.put_u16(dns_header.authority_cnt);
    buf.put_u16(dns_header.additional_cnt);
    // Questionセクションを追加
    buf.put(question.domain.as_slice());
    buf.put_u16(question.record_type);
    buf.put_u16(question.class);
    // Answerセクションを追加
    buf.put_u8(0xc0u8); // メッセージ圧縮
    buf.put_u8(12); // オフセット=12byte
    buf.put_u16(answer.record_type);
    buf.put_u16(answer.class);
    buf.put_u32(answer.ttl);
    buf.put_u16(answer.rd_length);
    buf.put(answer.rd_data.as_slice());

    buf
}
