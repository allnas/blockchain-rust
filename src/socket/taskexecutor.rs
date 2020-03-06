use std::thread;



use std::net::{UdpSocket, Ipv4Addr};


pub fn socket_server(){
  let mcast_group: Ipv4Addr = "233.0.0.1".parse().unwrap();
  let port: u16 = 6000;
  let any = "0.0.0.0".parse().unwrap();
  let mut buffer = [0u8; 1600];
    // client case
    let socket = UdpSocket::bind((any, port)).expect("Could not bind client socket");
    socket.join_multicast_v4(&mcast_group, &any)
        .expect("Could not join multicast group");
    socket.recv_from(&mut buffer).expect("Failed to write to server");
    
}

fn send_message(message: String) {
  let mcast_group: Ipv4Addr = "233.0.0.1".parse().unwrap();
  let port: u16 = 6000;
  let any : Ipv4Addr = "0.0.0.0".parse().unwrap();
  println!("startserver");
  let socket = UdpSocket::bind((any, 0)).expect("Could not write buffer as string");
  socket.send_to(message.as_bytes(), &(mcast_group, port)).expect("Failed to write data");
}