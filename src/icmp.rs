use bytes::{Buf, BufMut};

const ICMP_MESSAGE_TYPE_ECHO_REPLY: u8 = 0;
const ICMP_MESSAGE_TYPE_ECHO_REQUEST: u8 = 8;

struct ICMPHeader {
    icmp_type: u8, // メッセージタイプ
    icmp_code: u8,
    checksum: u16, // チェックサム
}

struct ICMPEchoMessage<'a> {
    identity_number: u16, // ID
    sequence_number: u16, // シーケンス番号
    data: &'a [u8],       // データ
}

pub fn read_icmp_packet(icmp_packet: Vec<u8>) -> Vec<u8> {
    let mut packet = &icmp_packet[..];

    let icmp_header = ICMPHeader {
        icmp_type: packet.get_u8(),
        icmp_code: packet.get_u8(),
        checksum: packet.get_u16(),
    };

    match icmp_header.icmp_type {
        // echo要求だけ応答
        ICMP_MESSAGE_TYPE_ECHO_REQUEST => {
            println!("icmp echo request");
            let echo = ICMPEchoMessage {
                identity_number: packet.get_u16(),
                sequence_number: packet.get_u16(),
                data: packet.get(0..).unwrap(),
            };
            return icmp_echo_reply(icmp_header, echo);
        }
        _ => {
            println!("other icmp message");
            return vec![];
        }
    }
}

fn icmp_echo_reply(mut header: ICMPHeader, mes: ICMPEchoMessage) -> Vec<u8> {
    header.icmp_type = ICMP_MESSAGE_TYPE_ECHO_REPLY;
    // 本当はchecksumを計算するべきだがめんどくさいので800を足す
    header.checksum += 0x0800;

    // ICMPヘッダ
    let mut buf = Vec::new();
    buf.put_u8(header.icmp_type);
    buf.put_u8(header.icmp_code);
    buf.put_u16(header.checksum);

    // ICMP replyメッセージ
    buf.put_u16(mes.identity_number);
    buf.put_u16(mes.sequence_number);
    buf.append(&mut mes.data.to_vec());

    return buf;
}
