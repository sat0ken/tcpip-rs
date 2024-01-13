use tcpip_rs::socket::recv_packet;

fn main() {
    recv_packet(Box::from("host2-host1"));
}
