use std::net::UdpSocket;
use std::net::Ipv4Addr;
pub fn socketserver(){
    let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    let mut buf = [0u8; 65535];
    let multi_addr = Ipv4Addr::new(234, 2, 2, 2);
    let inter = Ipv4Addr::new(0,0,0,0);
    socket.join_multicast_v4(&multi_addr,&inter);
    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from {:?}", amt, src);
    }
}