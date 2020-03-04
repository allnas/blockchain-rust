use std::thread;
use crate::socket::socketserver;
use crate::socket::socketclient;


pub fn receive_task(){
    let new_thread = thread::spawn(move || {
      socketserver::socketserver();
      println!("I am a new thread,task is receive.");
  });
  new_thread.join().unwrap();
}



pub fn send_task(){
      // 创建一个线程
      let new_thread = thread::spawn(move || {
        socketclient::socketclient();
        println!("I am a new thread,task is send.");
    });
    // 等待新建线程执行完成
    new_thread.join().unwrap();
}
>>>>>>> Stashed changes
