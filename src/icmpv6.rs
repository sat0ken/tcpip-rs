use bytes::Buf;

const ICMPV6_TYPE_ECHO_REQUEST: u8 = 128;
const ICMPV6_TYPE_ECHO_REPLY: u8 = 129;

struct ICMPV6Message {
    icmp_type: u8, // メッセージタイプ
    icmp_code: u8,
    checksum: u16, // チェックサム
    message: Vec<u8>,
}

pub fn read_icmpv6_packet(icmp_packet: Vec<u8>) -> Vec<u8> {
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
            // let echo = crate::icmp::ICMPEchoMessage {
            //     identity_number: packet.get_u16(),
            //     sequence_number: packet.get_u16(),
            //     data: packet.get(0..).unwrap(),
            // };
            icmpv6_echo_reply()
        }
        _ => {
            println!("other icmp message");
            vec![]
        }
    }
}

fn icmpv6_echo_reply() -> Vec<u8> {
    // ICMPヘッダ
    let mut buf = Vec::new();

    buf
}
