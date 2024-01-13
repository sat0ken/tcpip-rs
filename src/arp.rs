use std::sync::Mutex;

#[derive(Debug)]
pub struct ArpTable {
    mac_addr: [u8; 6],
    ip_addr: u32
}

static ARP_TABLES: Mutex<Vec<ArpTable>> = Mutex::new(Vec::new());

pub fn search_arp_tables(ip_addr: u32) -> [u8; 6] {
    let arp = ARP_TABLES.lock().unwrap();
    for i in 0..arp.len() {
        println!("search_arp_tables {:?}, {:?}", arp[i], ip_addr);
        if arp[i].ip_addr == ip_addr {
            return arp[i].mac_addr
        }
    }
    return [0, 0, 0, 0, 0, 0];
}

pub fn add_arp_tables(mac_addr: [u8; 6], ip_addr: u32) {
    let mut arp = ARP_TABLES.lock().unwrap();
    arp.push(ArpTable{mac_addr, ip_addr});
    println!("add arp tables entry is OK")
}