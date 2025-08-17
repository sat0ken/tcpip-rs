use crate::ethernet::read_ethernet;
use crate::util::{get_ipv4addr, get_sockaddr};
use nix::sys::socket::{
    bind, recvfrom, send, socket, AddressFamily, LinkAddr, MsgFlags, SockFlag, SockProtocol,
    SockType,
};
use std::os::fd::AsRawFd;
use std::sync::mpsc::sync_channel;
use std::thread;

pub fn recv_packet(if_name: Box<str>) {
    let mut buf = [0; 1514];
    let sock_addr = get_sockaddr(Box::from(if_name.clone())).unwrap();

    let mac_addr = sock_addr.as_link_addr().unwrap().addr().unwrap();

    // Todo: sock_addrとIPv4アドレスを同時に取得したい
    let ipv4_addr = get_ipv4addr(if_name);
    // IPv4アドレスが0だったらエラー
    if ipv4_addr == 0 {
        eprintln!("get ipv4 addr err");
    }

    let sock = socket(
        AddressFamily::Packet,
        SockType::Raw,
        SockFlag::empty(),
        SockProtocol::EthAll,
    )
    .expect("create socket failed");

    bind(sock.as_raw_fd(), &sock_addr).unwrap();

    println!("waiting for recv packet...");

    let (tx, rx) = sync_channel::<Vec<u8>>(0);

    loop {
        let tx = tx.clone();
        match recvfrom::<LinkAddr>(sock.as_raw_fd(), &mut buf) {
            Ok((size, _)) => {
                thread::spawn(move || {
                    read_ethernet(buf[0..size].to_owned(), tx, mac_addr, ipv4_addr);
                });
                let send_buf = rx.recv().unwrap();
                if 0 < send_buf.len() {
                    println!("send buf is {send_buf:?}");
                    send(sock.as_raw_fd(), &*send_buf, MsgFlags::empty()).unwrap();
                }
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}
