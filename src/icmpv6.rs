use crate::ipv6::IP_PROTOCOL_NUMBER_ICMPV6;
use crate::util::{checksum, dump_packet};
use bytes::{Buf, BufMut};

const ICMPV6_TYPE_ECHO_REQUEST: u8 = 128;
const ICMPV6_TYPE_ECHO_REPLY: u8 = 129;

struct ICMPV6Message {
    icmp_type: u8, // メッセージタイプ
    icmp_code: u8,
    checksum: u16, // チェックサム
    message: Vec<u8>,
}

struct ICMPV6Echo {
    identify: u16,
    sequence: u16,
    timestamp: u128,
    data: Vec<u8>,
}

struct IPV6DummyHeader {
    src_addr: u128,
    dst_addr: u128,
    length: u32,
    protocol: u32,
}

pub fn read_icmpv6_packet(src_addr: u128, dst_addr: u128, icmp_packet: Vec<u8>) -> Vec<u8> {
    let mut packet = &icmp_packet[..];

    let icmp_header = ICMPV6Message {
        icmp_type: packet.get_u8(),
        icmp_code: packet.get_u8(),
        checksum: packet.get_u16(),
        message: packet.to_owned(),
    };

    match icmp_header.icmp_type {
        // echo要求だけ応答
        ICMPV6_TYPE_ECHO_REQUEST => {
            println!("icmpv6 echo request");
            let mut message = &icmp_header.message[..];
            let echo = ICMPV6Echo {
                identify: message.get_u16(),
                sequence: message.get_u16(),
                timestamp: message.get_u128(),
                data: message.to_owned(),
            };
            icmpv6_echo_reply(src_addr, dst_addr, echo)
        }
        _ => {
            println!("other icmp message");
            vec![]
        }
    }
}

fn icmpv6_echo_reply(src_addr: u128, dst_addr: u128, icmpv6echo: ICMPV6Echo) -> Vec<u8> {
    // ICMPv6ヘッダ
    let mut buf = Vec::new();
    buf.put_u8(ICMPV6_TYPE_ECHO_REPLY);
    buf.put_u8(0x00); // code
    buf.put_u16(0x00); // checksum
                       // ICMPv6 EchoReply メッセージ
    buf.put_u16(icmpv6echo.identify);
    buf.put_u16(icmpv6echo.sequence);
    buf.put_u128(icmpv6echo.timestamp);
    buf.append(&mut icmpv6echo.data.to_vec());

    // IPv6ダミーヘッダをセット
    let dummy = IPV6DummyHeader {
        src_addr,
        dst_addr,
        length: buf.len() as u32,
        protocol: IP_PROTOCOL_NUMBER_ICMPV6 as u32,
    };
    let mut calc_checksum_buf: Vec<u8> = Vec::new();
    calc_checksum_buf.put_u128(dummy.src_addr);
    calc_checksum_buf.put_u128(dummy.dst_addr);
    calc_checksum_buf.put_u32(dummy.length);
    calc_checksum_buf.put_u32(dummy.protocol);
    calc_checksum_buf.put_slice(&buf.as_slice());

    // checksumを計算してセット
    let checksum = checksum(&calc_checksum_buf).to_be_bytes().to_vec();
    buf[2] = checksum[0];
    buf[3] = checksum[1];

    buf
}
