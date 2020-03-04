use std::net::UdpSocket;
pub fn socketclient(){
    println!("startclient");
    let socket = UdpSocket::bind("0.0.0.0:9999").unwrap();
    let mut buf = [1u8; 15000];
    let mut count = 1473;
    socket.send_to(&buf[0..count], "234.2.2.2:8888").unwrap();
}