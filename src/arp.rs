use crate::ethernet::ETHERNET_TYPE_IPV4;
use crate::util::to_u32;
use bytes::{Buf, BufMut};
use std::sync::Mutex;

#[derive(Debug)]
pub struct ArpTable {
    mac_addr: [u8; 6],
    ip_addr: u32,
}

#[derive(Debug)]
pub struct ArpTableV6 {
    mac_addr: [u8; 6],
    ip_addr: u128,
}

const ARP_HARDWARE_TYPE: u16 = 0x0001;
const ARP_OPERATION_TYPE_REQUEST: u16 = 0x0001;
const ARP_OPERATION_TYPE_REPLY: u16 = 0x0002;

#[derive(Debug)]
struct ArpMessage {
    hardware_type: u16,
    protocol_type: u16,
    hardware_addr_len: u8,
    protocol_addr_len: u8,
    operation_type: u16,
    src_mac_addr: [u8; 6],
    src_ip_addr: u32,
    dst_mac_addr: [u8; 6],
    dst_ip_addr: u32,
}

static ARP_TABLES: Mutex<Vec<ArpTable>> = Mutex::new(Vec::new());
static ARP_TABLES_V6: Mutex<Vec<ArpTableV6>> = Mutex::new(Vec::new());

pub fn search_arp_tables(ip_addr: u32) -> [u8; 6] {
    let arp = ARP_TABLES.lock().unwrap();
    for i in 0..arp.len() {
        println!("search_arp_tables {:?}, {:?}", arp[i], ip_addr);
        if arp[i].ip_addr == ip_addr {
            return arp[i].mac_addr;
        }
    }
    [0, 0, 0, 0, 0, 0]
}

pub fn add_arp_tables(mac_addr: [u8; 6], ip_addr: u32) {
    let mut arp = ARP_TABLES.lock().unwrap();
    arp.push(ArpTable { mac_addr, ip_addr });
    println!("add arp tables entry is OK")
}

pub fn search_arp_tables_v6(ip_addr: u128) -> [u8; 6] {
    let arp = ARP_TABLES_V6.lock().unwrap();
    for i in 0..arp.len() {
        println!("search_arp_tables v6 {:?}, {:?}", arp[i], ip_addr);
        if arp[i].ip_addr == ip_addr {
            return arp[i].mac_addr;
        }
    }
    [0, 0, 0, 0, 0, 0]
}

pub fn add_arp_tables_v6(mac_addr: [u8; 6], ip_addr: u128) {
    let mut arp = ARP_TABLES_V6.lock().unwrap();
    arp.push(ArpTableV6 { mac_addr, ip_addr });
    println!("add arp tables v6 entry is OK")
}

pub fn read_arp_packet(packet: Vec<u8>, my_mac_addr: [u8; 6], my_ip_addr: u32) -> (u32, Vec<u8>) {
    let mut arp = &packet[..];
    let arp_message = ArpMessage {
        hardware_type: arp.get_u16(),
        protocol_type: arp.get_u16(),
        hardware_addr_len: arp.get_u8(),
        protocol_addr_len: arp.get_u8(),
        operation_type: arp.get_u16(),
        // let src_mac_addr = <[u8; 6]>::try_from(arp.get_uint(6).to_be_bytes().get(2..).unwrap()).unwrap();
        src_mac_addr: arp[0..6].try_into().unwrap(),
        src_ip_addr: to_u32(&arp[6..10]),
        dst_mac_addr: arp[10..16].try_into().unwrap(),
        dst_ip_addr: to_u32(&arp[16..20]),
    };

    // ARPテーブルを検索して存在していなければ追加
    if search_arp_tables(arp_message.src_ip_addr) == [0, 0, 0, 0, 0, 0] {
        add_arp_tables(arp_message.src_mac_addr, arp_message.src_ip_addr)
    }

    if arp_message.operation_type == ARP_OPERATION_TYPE_REQUEST
        && arp_message.dst_ip_addr == my_ip_addr
    {
        return (
            arp_message.src_ip_addr,
            out_arp_reply(arp_message, my_mac_addr, my_ip_addr),
        );
    }
    (0, vec![])
}

fn out_arp_reply(arp_req: ArpMessage, my_mac_addr: [u8; 6], my_ip_addr: u32) -> Vec<u8> {
    let reply = ArpMessage {
        hardware_type: ARP_HARDWARE_TYPE,
        protocol_type: ETHERNET_TYPE_IPV4,
        hardware_addr_len: 6,
        protocol_addr_len: 4,
        operation_type: ARP_OPERATION_TYPE_REPLY,
        src_mac_addr: my_mac_addr,
        src_ip_addr: my_ip_addr,
        dst_mac_addr: arp_req.src_mac_addr,
        dst_ip_addr: arp_req.src_ip_addr,
    };
    let mut buf = Vec::new();
    buf.put_u16(reply.hardware_type);
    buf.put_u16(reply.protocol_type);
    buf.put_u8(reply.hardware_addr_len);
    buf.put_u8(reply.protocol_addr_len);
    buf.put_u16(reply.operation_type);
    buf.append(&mut reply.src_mac_addr.to_vec());
    buf.put_u32(reply.src_ip_addr);
    buf.append(&mut reply.dst_mac_addr.to_vec());
    buf.put_u32(reply.dst_ip_addr);

    buf
}
